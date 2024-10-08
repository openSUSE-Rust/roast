use clap::Parser;
use libroast::compress;
use roast_cli::cli;
use std::{
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
    let workdir = tempfile::TempDir::new()
        .map_err(|err| {
            error!(?err, "Failed to create temporary directory");
            err
        })?
        .into_path();

    let target_path = if roast_args.preserve_root
    {
        let newworkdir = workdir.join(target_path.file_name().unwrap_or(target_path.as_os_str()));
        copy_dir_all(&target_path, &newworkdir)?;
        newworkdir
    }
    else
    {
        copy_dir_all(&target_path, &workdir)?;
        workdir.clone()
    };

    let outpath = roast_args.outpath;
    println!("{}", target_path.display());

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

    let updated_paths: Vec<PathBuf> = WalkDir::new(&workdir)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .map(|f| {
            debug!(?f);
            PathBuf::from(f.path())
        })
        .filter(|p| *p != workdir)
        .collect();

    debug!("Workdir is now in {}", &workdir.display());

    let reproducible = roast_args.reproducible;

    match outpath.extension()
    {
        Some(ext) => match ext.to_string_lossy().to_string().as_str()
        {
            "gz" =>
            {
                compress::targz(&outpath, &workdir, &updated_paths, reproducible).map_err(
                    |err| {
                        error!(?err);
                        err
                    },
                )?;
                info!("Your new tarball is now in {}", &outpath.display());
            }
            "zst" | "zstd" =>
            {
                compress::tarzst(&outpath, &workdir, &updated_paths, reproducible).map_err(
                    |err| {
                        error!(?err);
                        err
                    },
                )?;
                info!("Your new tarball is now in {}", &outpath.display());
            }
            "bz" =>
            {
                compress::tarbz2(&outpath, &workdir, &updated_paths, reproducible).map_err(
                    |err| {
                        error!(?err);
                        err
                    },
                )?;
                info!("Your new tarball is now in {}", &outpath.display());
            }
            "xz" =>
            {
                compress::tarxz(&outpath, &workdir, &updated_paths, reproducible).map_err(
                    |err| {
                        error!(?err);
                        err
                    },
                )?;
                info!("Your new tarball is now in {}", &outpath.display());
            }
            _ =>
            {
                let err = io::Error::new(
                    io::ErrorKind::Unsupported,
                    "Compression type unsupported. Valid options are xz, zst / zstd, bz, and gz.",
                );
                error!(?err);
                return Err(err);
            }
        },
        None =>
        {
            let err = io::Error::new(
                io::ErrorKind::Unsupported,
                "Please provide a file extension. Compression type unsupported. Valid options are \
                 xz, zst / zstd, bz, and gz.",
            );
            error!(?err);
            return Err(err);
        }
    }
    Ok(())
}
