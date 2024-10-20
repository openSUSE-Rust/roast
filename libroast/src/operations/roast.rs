use crate::{
    compress,
    copy_dir_all,
    operations::cli,
    start_tracing,
    utils::process_globs,
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
use walkdir::{
    DirEntry,
    WalkDir,
};

fn is_hidden(entry: &DirEntry, hidden: bool, ignore_git: bool) -> bool
{
    let entry = entry.file_name().to_string_lossy();
    if hidden
    {
        let h = entry.starts_with(".");
        let g = if ignore_git { true } else { !entry.starts_with(".git") };
        h && g
    }
    else if ignore_git
    {
        entry.starts_with(".git")
    }
    else
    {
        false
    }
}

pub fn roast_opts(roast_args: cli::RoastArgs, start_trace: bool) -> io::Result<()>
{
    if start_trace
    {
        start_tracing();
    }

    info!("❤️‍🔥 Starting Roast.");
    debug!(?roast_args);
    let target_path = process_globs(&roast_args.target)?;
    let target_path = target_path.canonicalize().unwrap_or(target_path);
    let tmp_binding = tempfile::Builder::new()
        .prefix("rooooooooooaaaaaaaasssst")
        .rand_bytes(8)
        .tempdir()
        .inspect_err(|err| {
            error!(?err, "Failed to create temporary directory");
        })?;
    let workdir = &tmp_binding.path();
    let setup_workdir = if roast_args.preserve_root
    {
        let newworkdir = workdir.join(target_path.file_name().unwrap_or(target_path.as_os_str()));
        newworkdir
    }
    else
    {
        workdir.to_path_buf()
    };
    fs::create_dir_all(&setup_workdir)?;

    let outdir = match roast_args.outdir
    {
        Some(v) => v,
        None => std::env::current_dir()?,
    };
    let outpath = outdir.join(roast_args.outfile);
    let outpath = outpath.canonicalize().unwrap_or(outpath);

    let ignore_paths: Vec<PathBuf> = roast_args.ignore_paths.unwrap_or_default();

    debug!(?ignore_paths, "IGNORED");

    let walker = WalkDir::new(&target_path).into_iter();

    for entry in
        walker.filter_entry(|e| !is_hidden(e, roast_args.hidden, roast_args.ignore_git)).flatten()
    {
        debug!(?entry, "entry to copy");
        let p_path = &entry.clone().into_path().canonicalize().unwrap_or(entry.into_path());
        debug!(?p_path, "Path to copy");
        debug!("PATH EXISTS? {}", p_path.exists());
        let w_path = p_path
            .strip_prefix(&target_path)
            .unwrap_or(&PathBuf::from(&p_path.file_name().unwrap_or(p_path.as_os_str())))
            .to_path_buf();
        debug!(?w_path);
        if p_path.is_file()
        {
            let dst = &setup_workdir.join(&w_path);
            debug!(?dst, "destination");
            if dst.exists()
            {
                warn!(
                    "Additional file will overwrite existing file at path `{}`. Consider adding \
                     `-p` to mitigate this.",
                    dst.display()
                );
            }
            fs::copy(p_path, dst)?;
        }
        else if p_path.is_dir()
        {
            let dst = &setup_workdir.join(&w_path);
            debug!(?dst, "destination");
            // Avoid the setup workdir the same as dst
            if dst.canonicalize().unwrap_or(dst.to_path_buf())
                != setup_workdir.canonicalize().unwrap_or(setup_workdir.to_path_buf())
            {
                if dst.exists()
                {
                    warn!(
                        "Additional file will overwrite existing file at path `{}`. Consider \
                         adding `-p` to mitigate this.",
                        dst.display()
                    );
                }
                fs::create_dir_all(dst)?;
            }
        }
    }

    if let Some(additional_paths) = roast_args.additional_paths
    {
        // TODO: Set this before everything so we won't have to do it again and again.
        for path in additional_paths
        {
            if ignore_paths.contains(&path)
            {
                warn!(
                    "⚠️ You are adding a path that's also been declared to be ignored. Excluding \
                     paths takes higher precendence. Path to be ignored: {}",
                    path.display()
                );
            }
            else if path.is_file()
            {
                debug!(?path, "Additional file");
                let dst = &setup_workdir.join(path.file_name().unwrap_or(path.as_os_str()));
                if dst.exists()
                {
                    warn!(
                        "⚠️ Additional file will overwrite existing file at path `{}`.",
                        dst.display()
                    );
                }
                debug!(?dst, "Destination path");
                fs::copy(&path, dst)?;
            }
            else if path.is_dir()
            {
                debug!(?path, "Additional directory");
                let dst = &setup_workdir.join(path.file_name().unwrap_or(path.as_os_str()));
                if dst.exists()
                {
                    warn!(
                        "⚠️ Additional directory may overwrite contents of existing directory at \
                         path `{}`.",
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
        .flatten()
        .map(|f| {
            debug!(?f);
            f.into_path()
        })
        .filter(|p| p.is_file())
        .collect();

    debug!(?updated_paths, "Updated paths");
    debug!("Ignore paths: {:#?}", &ignore_paths);
    debug!("Workdir: {}", &workdir.display());

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
