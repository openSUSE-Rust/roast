use crate::{
    compress,
    copy_dir_all,
    operations::cli,
    start_tracing,
};
use std::{
    ffi::OsStr,
    fs,
    io,
    path::PathBuf,
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
use walkdir::WalkDir;

pub fn roast_opts(roast_args: cli::RoastArgs, start_trace: bool) -> io::Result<()>
{
    if start_trace
    {
        start_tracing();
    }

    info!("❤️‍🔥 Starting Roast.");
    debug!(?roast_args);
    let target_path = roast_args.target;
    let target_path = target_path.canonicalize().unwrap_or(target_path);
    let tmp_binding = tempfile::Builder::new()
        .prefix(".rooooooooooaaaaaaaasssst")
        .rand_bytes(8)
        .tempdir()
        .inspect_err(|err| {
            error!(?err, "Failed to create temporary directory");
        })?;
    let workdir = &tmp_binding.path();

    if roast_args.preserve_root
    {
        let newworkdir = workdir.join(target_path.file_name().unwrap_or(target_path.as_os_str()));
        copy_dir_all(&target_path, &newworkdir)?;
    }
    else
    {
        copy_dir_all(&target_path, workdir)?;
    };

    let outpath = roast_args.outfile;
    let outpath = outpath.canonicalize().unwrap_or(outpath);

    if let Some(additional_paths) = roast_args.additional_paths
    {
        for path in additional_paths
        {
            let path = path.canonicalize().unwrap_or(path);
            if path.is_file()
            {
                debug!(?path, "Additional file");
                let dst = &workdir.join(path.file_name().unwrap_or(path.as_os_str()));
                let dst = dst.canonicalize().unwrap_or(dst.to_path_buf());
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
                let dst = dst.canonicalize().unwrap_or(dst.to_path_buf());
                if dst.exists()
                {
                    warn!(
                        "Additional directory may overwrite contents of existing directory at \
                         path `{}`. Consider adding `-p` to mitigate this.",
                        dst.display()
                    );
                }
                debug!(?dst, "Destination path");
                copy_dir_all(&path, &dst)?;
            }
        }
    }

    let updated_paths: Vec<PathBuf> = WalkDir::new(workdir)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .map(|f| {
            debug!(?f);
            f.path().canonicalize().unwrap_or(f.path().to_path_buf())
        })
        .filter(|p| {
            p.canonicalize().unwrap_or(p.clone())
                != workdir.canonicalize().unwrap_or(workdir.to_path_buf())
        })
        .filter(|p| p.is_file())
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
                debug!(?bind_ft);
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
