use clap::Parser;
use libroast::compress;
use roast_cli::cli;
use std::{
    ffi::OsStr,
    fs,
    io,
    path::{
        Path,
        PathBuf,
    },
};
use terminfo::{
    capability as cap,
    Database,
};
#[allow(unused_imports)]
use tracing::{
    debug,
    error,
    info,
    trace,
    warn,
    Level,
};
use tracing_subscriber::EnvFilter;
use walkdir::WalkDir;

fn copy_dir_all(src: impl AsRef<Path>, dst: &Path) -> Result<(), io::Error>
{
    debug!("Copying sources");
    debug!(?dst);
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)?
    {
        let entry = entry?;
        let ty = entry.file_type()?;
        trace!(?entry);
        trace!(?ty);
        if ty.is_dir()
        {
            trace!(?ty, "Is directory?");
            copy_dir_all(entry.path(), &dst.join(entry.file_name()))?;

        // Should we respect symlinks?
        // } else if ty.is_symlink() {
        //     debug!("Is symlink");
        //     let path = fs::read_link(&entry.path())?;
        //     let path = fs::canonicalize(&path).unwrap();
        //     debug!(?path);
        //     let pathfilename = path.file_name().unwrap_or(OsStr::new("."));
        //     if path.is_dir() {
        //         copy_dir_all(&path, &dst.join(pathfilename))?;
        //     } else {
        //         fs::copy(&path, &mut dst.join(pathfilename))?;
        //     }

        // Be pedantic or you get symlink error
        }
        else if ty.is_file()
        {
            trace!(?ty, "Is file?");
            fs::copy(entry.path(), dst.join(entry.file_name()))?;
        };
    }
    Ok(())
}

fn main() -> io::Result<()>
{
    let roast_args = cli::RoastArgs::parse();
    let terminfodb = Database::from_env().map_err(|e| {
        error!(err = ?e, "Unable to access terminfo db. This is a bug!");
        io::Error::new(
            io::ErrorKind::Other,
            "Unable to access terminfo db. This is a bug! Setting color option to false!",
        )
    });

    let is_termcolorsupported = match terminfodb
    {
        Ok(hasterminfodb) => hasterminfodb.get::<cap::MaxColors>().is_some(),
        Err(_) => false,
    };
    let filter_layer = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let builder = tracing_subscriber::fmt()
        .with_level(true)
        .with_ansi(is_termcolorsupported)
        .with_env_filter(filter_layer)
        .with_level(true);

    let builder = if cfg!(debug_assertions)
    {
        builder.with_file(true).with_line_number(true)
    }
    else
    {
        builder
    };

    builder.init();

    info!("❤️‍🔥 Starting Roast.");
    debug!(?roast_args);
    let target_path = roast_args.target;
    let tmp_binding = tempfile::TempDir::new().map_err(|err| {
        error!(?err, "Failed to create temporary directory");
        err
    })?;
    let workdir = &tmp_binding.path();

    if roast_args.preserve_root
    {
        let newworkdir = workdir.join(target_path.file_name().unwrap_or(target_path.as_os_str()));
        copy_dir_all(&target_path, &newworkdir)?;
        newworkdir
    }
    else
    {
        copy_dir_all(&target_path, workdir)?;
        workdir.to_path_buf()
    };

    let outpath = roast_args.outpath;

    if let Some(additional_paths) = roast_args.additional_paths
    {
        for path in additional_paths
        {
            if path.is_file()
            {
                debug!(?path, "Additional file");
                let dst = &workdir.join(path.file_name().unwrap_or(path.as_os_str()));
                if dst.exists()
                {
                    warn!(
                        "Additional file will overwrite existing file at path `{}`. Consider \
                         adding `-p` to mitigate this.",
                        dst.display()
                    );
                }
                debug!(?dst, "Destination path");
                fs::copy(&path, dst)?;
            }
            else if path.is_dir()
            {
                debug!(?path, "Additional directory");
                let dst = &workdir.join(path.file_name().unwrap_or(path.as_os_str()));
                if dst.exists()
                {
                    warn!(
                        "Additional directory may overwrite contents of existing directory at \
                         path `{}`. Consider adding `-p` to mitigate this.",
                        dst.display()
                    );
                }
                debug!(?dst, "Destination path");
                copy_dir_all(&path, dst)?;
            }
        }
    }

    let updated_paths: Vec<PathBuf> = WalkDir::new(workdir)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .map(|f| {
            debug!(?f);
            PathBuf::from(f.path())
        })
        .filter(|p| *p != *workdir)
        .collect();

    debug!("Workdir is now in {}", &workdir.display());

    let reproducible = roast_args.reproducible;

    let has_ext = &outpath.extension();

    let result = match has_ext
    {
        Some(ext) =>
        {
            let ext_val = &outpath.with_extension("");
            let ext_val = ext_val.extension();
            debug!(?ext_val);
            let is_tar = [Some(OsStr::new("tar")), None].contains(&ext_val);
            if is_tar
            {
                let bind_ft = ext.to_string_lossy().to_string();
                let some_ft = bind_ft.as_str();
                match some_ft
                {
                    "gz" => compress::targz(&outpath, workdir, &updated_paths, reproducible),
                    "xz" => compress::tarxz(&outpath, workdir, &updated_paths, reproducible),
                    "bz" => compress::tarbz2(&outpath, workdir, &updated_paths, reproducible),
                    "zst" | "zstd" =>
                    {
                        compress::tarzst(&outpath, workdir, &updated_paths, reproducible)
                    }
                    "tar" => compress::vanilla(&outpath, workdir, &updated_paths, reproducible),
                    _ =>
                    {
                        let message = format!("Unsupported file type: {}", ext.to_string_lossy());
                        Err(io::Error::new(io::ErrorKind::Unsupported, message))
                    }
                }
            }
            else
            {
                let message = format!("Unsupported file type: {}", ext.to_string_lossy());
                Err(io::Error::new(io::ErrorKind::Unsupported, message))
            }
        }
        None => Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "Cannot determine compression with no file extension",
        )),
    };

    // Do not return the error. Just inform the user.
    // This will allow us to delete the temporary directory.
    if let Err(err) = result
    {
        error!(?err);
    }
    else
    {
        info!("Your new tarball is now in {}", &outpath.display());
    }

    tmp_binding.close().inspect_err(|e| {
        error!(?e, "Failed to delete temporary directory!");
    })?;

    Ok(())
}
