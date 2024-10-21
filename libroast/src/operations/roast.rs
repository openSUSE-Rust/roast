use crate::{
    compress,
    copy_dir_all,
    operations::cli,
    start_tracing,
    utils::process_globs,
};
use std::{
    fs,
    io,
    path::{
        Path,
        PathBuf,
    },
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

fn filter_paths(
    target_path: &Path,
    root: &Path,
    hidden: bool,
    ignore_git: bool,
    ignore_paths: &[PathBuf],
) -> io::Result<()>
{
    let target_canonicalized = target_path.canonicalize().unwrap_or(target_path.to_path_buf());
    let root_canonicalized = root.canonicalize().unwrap_or(root.to_path_buf());
    if target_canonicalized != root_canonicalized
    {
        let walker = WalkDir::new(target_path).into_iter();
        for entry in walker.filter_entry(|e| !is_hidden(e, hidden, ignore_git, root)).flatten()
        {
            debug!(?entry, "entry to copy");
            let p_path = &entry.clone().into_path().canonicalize().unwrap_or(entry.into_path());
            debug!(?p_path, "Path to copy");
            debug!("PATH EXISTS? {}", p_path.exists());
            let w_path = p_path
                .strip_prefix(target_path)
                .unwrap_or(&PathBuf::from(&p_path.file_name().unwrap_or(p_path.as_os_str())))
                .to_path_buf();
            debug!(?w_path);
            if !ignore_paths.contains(p_path)
            {
                if p_path.is_file()
                {
                    let dst = &root.join(&w_path);
                    debug!(?dst, "destination");
                    if dst.exists()
                    {
                        warn!(
                            "⚠️ Additional file will overwrite existing file at path `{}`.",
                            dst.display()
                        );
                    }
                    fs::copy(p_path, dst)?;
                }
                else if p_path.is_dir()
                {
                    let dst = &root.join(&w_path);
                    debug!(?root, "WHAT THE FUCK IS ROOT! GROOT!");
                    debug!(?dst, "destination");
                    // Avoid the setup workdir the same as dst
                    if dst.canonicalize().unwrap_or(dst.to_path_buf())
                        != root.canonicalize().unwrap_or(root.to_path_buf())
                    {
                        if dst.exists()
                        {
                            warn!(
                                "⚠️ Additional directory will overwrite existing directory at \
                                 path `{}`.",
                                dst.display()
                            );
                        }
                        debug!(?dst, "final destination");
                        fs::create_dir_all(dst)?;
                    }
                }
            }
        }
    }
    Ok(())
}

fn is_hidden(entry: &DirEntry, hidden: bool, ignore_git: bool, root: &Path) -> bool
{
    let entry_str = &entry.file_name().to_string_lossy();
    debug!(?entry_str, ?root);
    let entry = &entry.clone().into_path();
    let entry_canonicalized = entry.canonicalize().unwrap_or(entry.to_path_buf());
    let root_canonicalized = root.canonicalize().unwrap_or(root.to_path_buf());
    if entry_canonicalized == root_canonicalized
    {
        return false;
    }
    else if ignore_git
    {
        entry_str.starts_with(".git")
    }
    else if hidden
    {
        entry_str.starts_with(".")
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
        .prefix(".rooooooooooaaaaaaaasssst")
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

    let mut ignore_paths: Vec<PathBuf> = roast_args.ignore_paths.unwrap_or_default();
    ignore_paths =
        ignore_paths.iter().map(|p| p.canonicalize().unwrap_or(p.to_path_buf())).collect();

    debug!(?ignore_paths, "IGNORED");

    filter_paths(
        &target_path,
        &setup_workdir,
        roast_args.hidden,
        roast_args.ignore_git,
        &ignore_paths,
    )?;

    if let Some(additional_paths) = roast_args.additional_paths
    {
        info!("⏫ Adding extra paths to source!");
        for path in additional_paths
        {
            let path_canonicalized = path.canonicalize().unwrap_or(path.to_path_buf());
            let target_canonicalized =
                target_path.canonicalize().unwrap_or(target_path.to_path_buf());
            let is_stripped = path_canonicalized.strip_prefix(&target_canonicalized);
            if is_stripped.is_ok()
            {
                info!(
                    "ℹ️ Added path `{}` is within target path `{}`",
                    path_canonicalized.display(),
                    target_canonicalized.display()
                );
                if ignore_paths.contains(&path)
                {
                    if path.is_file()
                    {
                        warn!(
                            "⚠️ You are adding a file path that's also been declared to be \
                             ignored. Excluding file paths takes higher precendence. File to be \
                             ignored: {}",
                            path.display()
                        );
                    }
                    else if path.is_dir()
                    {
                        warn!(
                            "⚠️ You are adding a directory path that's also been declared to be \
                             ignored. Excluding paths takes higher precendence. Directory to be \
                             ignored: {}",
                            path.display()
                        );
                        warn!(
                            "⚠️ Files that are explictly added that are within this directory \
                             will be added as we believe ignoring the directory and its contents \
                             are intentional except explicitly added files. If you think this \
                             behaviour is not the desired behaviour, please file an issue! 🙏",
                        );
                    }
                }
                else
                {
                    debug!(?path, "Additional directory");
                    let dst = &setup_workdir.join(path.file_name().unwrap_or_default());
                    if dst.is_dir()
                    {
                        warn!(
                            "⚠️ Additional directory may overwrite contents of existing directory \
                             at path `{}`.",
                            dst.display()
                        );
                    }
                    else if dst.is_file()
                    {
                        warn!(
                            "⚠️ Additional file may overwrite contents of existing file at path \
                             `{}`.",
                            dst.display()
                        );
                    };
                    if path.is_dir()
                    {
                        fs::create_dir_all(dst)?;
                        debug!(?dst, "Destination path");
                        filter_paths(
                            &path,
                            dst,
                            roast_args.hidden,
                            roast_args.ignore_git,
                            &ignore_paths,
                        )?;
                    }
                    else if path.is_file()
                    {
                        let parent = path.parent().unwrap_or(Path::new("/"));
                        for ig_path in &ignore_paths
                        {
                            let ig_path_canonicalized =
                                ig_path.canonicalize().unwrap_or(ig_path.to_path_buf());
                            let path_canonicalized =
                                &path.canonicalize().unwrap_or(path.to_path_buf());
                            let is_stripped =
                                path_canonicalized.strip_prefix(ig_path_canonicalized);
                            if is_stripped.is_ok()
                            {
                                warn!(
                                    "⚠️ File `{}` is within an ignored directory `{}`.",
                                    &path.display(),
                                    &path_canonicalized.display()
                                );
                                warn!(
                                    "⚠️ Files that are added that are within this directory will \
                                     be added as we believe ignoring the directory and its \
                                     contents are intentional except explicitly added files. If \
                                     you think this behaviour is not the desired behaviour, \
                                     please file an issue! 🙏",
                                );
                                warn!("⚠️ File to be added: {}", &path.display());
                                break;
                            }
                        }
                        let dst_parent =
                            &setup_workdir.join(parent.file_name().unwrap_or(parent.as_os_str()));
                        fs::create_dir_all(dst_parent)?;
                        debug!(?path, "Destination path");
                        filter_paths(
                            &path,
                            dst_parent,
                            roast_args.hidden,
                            roast_args.ignore_git,
                            &ignore_paths,
                        )?;
                    }
                }
            }
            else
            {
                info!("ℹ️ You are adding paths outside the target path!");
                let dst = &setup_workdir.join(path.file_name().unwrap_or_default());
                if path.is_dir()
                {
                    if dst.exists()
                    {
                        warn!(
                            "⚠️ Additional directory may overwrite contents of existing directory \
                             at path `{}`.",
                            dst.display()
                        );
                    }
                    copy_dir_all(&path, dst)?;
                }
                else if path.is_file()
                {
                    if dst.exists()
                    {
                        warn!(
                            "⚠️ Additional file may overwrite contents of existing file at path \
                             `{}`.",
                            dst.display()
                        );
                    }
                    fs::copy(&path, dst)?;
                }
                info!("ℹ️ Added path `{}`", path_canonicalized.display(),);
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

    let outpath_str = outpath.as_os_str().to_string_lossy();
    let result = if outpath_str.ends_with("tar.gz")
    {
        compress::targz(&outpath, workdir, &updated_paths, reproducible)
    }
    else if outpath_str.ends_with("tar.xz")
    {
        compress::tarxz(&outpath, workdir, &updated_paths, reproducible)
    }
    else if outpath_str.ends_with("tar.zst") | outpath_str.ends_with("tar.zstd")
    {
        compress::tarzst(&outpath, workdir, &updated_paths, reproducible)
    }
    else if outpath_str.ends_with("tar.bz")
    {
        compress::tarbz2(&outpath, workdir, &updated_paths, reproducible)
    }
    else if outpath_str.ends_with("tar")
    {
        compress::vanilla(&outpath, workdir, &updated_paths, reproducible)
    }
    else
    {
        let msg = format!("Unsupported file: {}", outpath_str);
        Err(io::Error::new(io::ErrorKind::Unsupported, msg))
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
