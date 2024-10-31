use crate::{
    compress,
    operations::cli,
    start_tracing,
    utils::process_globs,
};
use std::{
    fs::{
        self,
        read_dir,
    },
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

fn is_hidden(entry: &fs::DirEntry, hidden: bool, ignore_git: bool, root: &Path) -> bool
{
    let binding = entry.file_name();
    let entry_str = &binding.to_string_lossy();
    debug!(?entry_str, ?root);
    let entry = &entry.path();
    let entry_canonicalized = entry.canonicalize().unwrap_or(entry.to_path_buf());
    let root_canonicalized = root.canonicalize().unwrap_or(root.to_path_buf());
    if entry_canonicalized == root_canonicalized
    {
        false
    }
    else if entry_str.starts_with(".git")
    {
        ignore_git
    }
    else if entry_str.starts_with(".")
    {
        hidden
    }
    else
    {
        false
    }
}

fn filter_paths(
    target_path: &Path,
    root: &Path,
    hidden: bool,
    ignore_git: bool,
    exclude_paths: &[PathBuf],
) -> io::Result<()>
{
    let target_dir = read_dir(target_path)?.flatten();
    for entry in target_dir
    {
        let entry_as_path = &entry.path();
        let entry_as_path_canonicalized =
            &entry_as_path.canonicalize().unwrap_or(entry_as_path.to_path_buf());
        if !is_hidden(&entry, hidden, ignore_git, root)
        {
            for exclude in exclude_paths
            {
                let is_stripped = entry_as_path_canonicalized.strip_prefix(exclude);
                if is_stripped.is_err() && entry_as_path_canonicalized.is_dir()
                {
                    let entry_stripped_by_target_path = entry_as_path_canonicalized
                        .strip_prefix(target_path)
                        .unwrap_or(target_path);
                    let genesis_dir = &root.join(entry_stripped_by_target_path);
                    fs::create_dir_all(genesis_dir)?;
                    filter_paths(
                        entry_as_path_canonicalized,
                        genesis_dir,
                        hidden,
                        ignore_git,
                        exclude_paths,
                    )?;
                }
                if entry_as_path_canonicalized.is_file() && is_stripped.is_err()
                {
                    let entry_stripped_by_target_path = entry_as_path_canonicalized
                        .strip_prefix(target_path)
                        .unwrap_or(target_path);
                    let genesis_path = &root.join(entry_stripped_by_target_path);
                    let genesis_path_parent = genesis_path.parent().unwrap_or(root);
                    let is_stripped_genesis_path_parent = genesis_path_parent.strip_prefix(exclude);
                    if is_stripped_genesis_path_parent.is_ok() && (*genesis_path_parent != *root)
                    {
                        warn!(
                            "⚠️ Adding file `{}` that is WITHIN an EXCLUDED directory `{}`.",
                            entry_as_path_canonicalized.display(),
                            &exclude.display()
                        );
                    }
                    fs::create_dir_all(genesis_path_parent)?;
                    fs::copy(entry_as_path_canonicalized, genesis_path)?;
                }
            }
        }
    }
    Ok(())
}

fn process_additional_paths(adtnl_path: &str, root: &Path) -> (PathBuf, PathBuf)
{
    if let Some((ar, tgt)) = adtnl_path.split_once(",")
    {
        debug!(?ar, ?tgt);
        let tgt = if tgt.is_empty() { root } else { &root.join(tgt) };
        (PathBuf::from(&ar), tgt.to_path_buf())
    }
    else
    {
        (PathBuf::from(&adtnl_path), root.to_path_buf())
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
        let newworkdir = workdir.join(target_path.file_name().unwrap_or_default());
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

    let mut exclude_canonicalized_path: Vec<PathBuf> =
        roast_args.exclude.clone().unwrap_or_default();

    exclude_canonicalized_path = exclude_canonicalized_path
        .iter()
        .map(|p| target_path.join(p).canonicalize().unwrap_or_default())
        .collect();

    debug!(?exclude_canonicalized_path);

    if let Some(additional_paths) = roast_args.additional_paths
    {
        for adtnlp in additional_paths
        {
            debug!(?adtnlp);
            let (src, tgt) = process_additional_paths(&adtnlp, &setup_workdir);
            if src.is_file()
            {
                let tgt_stripped = tgt.strip_prefix(&setup_workdir).unwrap_or(Path::new("/"));
                let target_with_tgt = &target_path.join(tgt_stripped);
                if exclude_canonicalized_path.contains(target_with_tgt)
                {
                    warn!(
                        "Excluded path `{}` has added a file OUTSIDE of target directory. Added \
                         file: {}",
                        &target_with_tgt.display(),
                        &src.display()
                    );
                }
                // create directory and warn if it's an excluded directory
                fs::create_dir_all(&tgt)?;
                // Copy file to target path
                fs::copy(&src, tgt.join(&src))?;
            }
            else if src.is_dir()
            {
                let tgt_stripped = tgt.strip_prefix(&setup_workdir).unwrap_or(Path::new("/"));
                let target_with_tgt = &target_path.join(tgt_stripped);
                if exclude_canonicalized_path.contains(target_with_tgt)
                {
                    warn!(
                        "Added directory that is excluded will be ignored. Ignored directory: {}",
                        &target_with_tgt.display()
                    );
                }
                else
                {
                    filter_paths(
                        &src,
                        &tgt,
                        roast_args.ignore_hidden,
                        roast_args.ignore_git,
                        &exclude_canonicalized_path,
                    )?;
                }
            }
        }
    }

    if let Some(include_paths) = roast_args.include
    {
        for include_path in include_paths
        {
            debug!(?include_path);
            let include_to_path = &setup_workdir.join(&include_path);
            let include_from_path = &target_path.join(&include_path);
            if include_from_path.is_dir()
            {
                if exclude_canonicalized_path.contains(include_from_path)
                {
                    warn!(
                        "Added directory that is excluded will be ignored. Ignored directory: {}",
                        &include_from_path.display()
                    );
                }
                filter_paths(
                    include_from_path,
                    include_to_path,
                    roast_args.ignore_hidden,
                    roast_args.ignore_git,
                    &exclude_canonicalized_path,
                )?;
            }
            else if include_from_path.is_file()
            {
                let include_from_path_parent =
                    include_from_path.parent().unwrap_or(&target_path.to_path_buf()).to_path_buf();
                let include_to_path_parent =
                    include_to_path.parent().unwrap_or(&setup_workdir.to_path_buf()).to_path_buf();
                if !exclude_canonicalized_path.contains(&include_from_path_parent)
                {
                    // create directory and warn if it's an excluded directory
                    fs::create_dir_all(&include_to_path_parent)?;
                    // Copy file to target path
                    fs::copy(include_from_path, include_to_path)?;
                }
                warn!(
                    "Excluded path `{}` has added a file IN target directory. Added file: {}",
                    &include_from_path_parent.display(),
                    &include_from_path.display()
                );
            }
        }
    }

    filter_paths(
        &target_path,
        &setup_workdir,
        roast_args.ignore_hidden,
        roast_args.ignore_git,
        &exclude_canonicalized_path,
    )?;

    let archive_files: Vec<PathBuf> = WalkDir::new(workdir)
        .into_iter()
        .flatten()
        .map(|f| {
            debug!(?f);
            f.into_path()
        })
        .filter(|p| p.is_file())
        .collect();

    debug!(?archive_files);

    let reproducible = roast_args.reproducible;

    let outpath_str = outpath.as_os_str().to_string_lossy();
    let result = if outpath_str.ends_with("tar.gz")
    {
        compress::targz(&outpath, workdir, &archive_files, reproducible)
    }
    else if outpath_str.ends_with("tar.xz")
    {
        compress::tarxz(&outpath, workdir, &archive_files, reproducible)
    }
    else if outpath_str.ends_with("tar.zst") | outpath_str.ends_with("tar.zstd")
    {
        compress::tarzst(&outpath, workdir, &archive_files, reproducible)
    }
    else if outpath_str.ends_with("tar.bz")
    {
        compress::tarbz2(&outpath, workdir, &archive_files, reproducible)
    }
    else if outpath_str.ends_with("tar")
    {
        compress::vanilla(&outpath, workdir, &archive_files, reproducible)
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
