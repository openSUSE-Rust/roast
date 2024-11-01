pub mod helpers;
use crate::{
    compress,
    operations::cli,
    utils::{
        process_globs,
        start_tracing,
    },
};
use helpers::{
    filter_paths,
    is_excluded,
};
use std::{
    fs::{
        self,
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
use walkdir::WalkDir;

pub fn get_additional_paths(adtnl_path: &str, root: &Path) -> (PathBuf, PathBuf)
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

pub fn process_additional_paths(
    additional_paths: &[String],
    target_path: &Path,
    exclude_canonicalized_paths: &[PathBuf],
    setup_workdir: &Path,
    roast_args: &cli::RoastArgs,
) -> io::Result<()>
{
    for adtnlp in additional_paths
    {
        debug!(?adtnlp);
        let (additional_from_path, additional_to_path) =
            get_additional_paths(adtnlp, setup_workdir);
        debug!(?additional_from_path, ?additional_to_path);
        let src_canonicalized =
            additional_from_path.canonicalize().unwrap_or(additional_from_path.to_path_buf());
        debug!(?src_canonicalized);
        if src_canonicalized.is_file()
        {
            let tgt_stripped =
                additional_to_path.strip_prefix(setup_workdir).unwrap_or(Path::new("/"));
            let target_with_tgt = &target_path.join(tgt_stripped);
            if is_excluded(target_with_tgt, exclude_canonicalized_paths)
            {
                warn!(
                    "⚠️ Directory `{}` is WITHIN an EXCLUDED path. Added a file OUTSIDE of target \
                     directory: {}",
                    &target_with_tgt.display(),
                    &src_canonicalized.display()
                );
            }
            // create directory and warn if it's an excluded directory
            fs::create_dir_all(&additional_to_path)?;
            // Copy file to target path
            fs::copy(
                &src_canonicalized,
                additional_to_path.join(additional_from_path.file_name().unwrap_or_default()),
            )?;
        }
        else if src_canonicalized.is_dir()
        {
            let tgt_stripped =
                additional_to_path.strip_prefix(setup_workdir).unwrap_or(Path::new("/"));
            let target_with_tgt = &target_path.join(tgt_stripped);
            if is_excluded(target_with_tgt, exclude_canonicalized_paths)
            {
                warn!(
                    "⚠️ ADDITIONAL directory that was WITHIN one of the EXCLUDED paths was added \
                     back from OUTSIDE target path: {}",
                    &target_with_tgt.display()
                );
                warn!("⚠️ This may not contain the same contents!");
            }
            let new_additional_to_path =
                additional_to_path.join(src_canonicalized.file_name().unwrap_or_default());
            fs::create_dir_all(&new_additional_to_path)?;
            filter_paths(
                &src_canonicalized,
                &new_additional_to_path,
                roast_args.ignore_hidden,
                roast_args.ignore_git,
                &[],
            )?;
        }
    }
    Ok(())
}

pub fn process_include_paths(
    include_paths: &[PathBuf],
    exclude_canonicalized_paths: &[PathBuf],
    target_path: &Path,
    setup_workdir: &Path,
    roast_args: &cli::RoastArgs,
) -> io::Result<()>
{
    for include_path in include_paths
    {
        let include_from_path = &target_path.join(include_path);
        let include_from_path =
            include_from_path.canonicalize().unwrap_or(include_from_path.to_path_buf());
        if !include_from_path.exists()
        {
            let err = io::Error::new(
                io::ErrorKind::NotFound,
                "Path does not exist. This means that this path is not WITHIN the target \
                 directory.",
            );
            error!(?err);
            return Err(err);
        }

        let include_to_path = &setup_workdir.join(include_path);
        debug!(?include_path, ?include_from_path, ?include_to_path);
        if include_from_path.is_dir()
        {
            if is_excluded(&include_from_path, exclude_canonicalized_paths)
            {
                warn!(
                    "⚠️ INCLUDED directory that is EXCLUDED will be IGNORED: {}",
                    &include_from_path.display()
                );
            }
            else
            {
                filter_paths(
                    &include_from_path,
                    include_to_path,
                    roast_args.ignore_hidden,
                    roast_args.ignore_git,
                    &[],
                )?;
            }
        }
        else if include_from_path.is_file()
        {
            let include_from_path_parent =
                include_from_path.parent().unwrap_or(target_path).to_path_buf();
            let include_to_path_parent =
                include_to_path.parent().unwrap_or(setup_workdir).to_path_buf();
            if is_excluded(&include_from_path_parent, exclude_canonicalized_paths)
            {
                warn!(
                    "⚠️ Path `{}` WITHIN an EXCLUDED path has added a file IN target directory. \
                     Added file: {}",
                    &include_from_path_parent.display(),
                    &include_from_path.display()
                );
            }
            if is_excluded(&include_from_path, exclude_canonicalized_paths)
            {
                warn!(
                    "⚠️ EXCLUDED file `{}` has also been declared INCLUDED. Adding file takes \
                     precedence. Added file: {}",
                    &include_from_path.display(),
                    &include_from_path.display()
                );
            }
            // create directory and warn if it's an excluded directory
            fs::create_dir_all(&include_to_path_parent)?;
            // Copy file to target path
            fs::copy(include_from_path, include_to_path)?;
        }
    }
    Ok(())
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

    let outdir = match &roast_args.outdir
    {
        Some(v) => v,
        None => &std::env::current_dir()?,
    };

    let outpath = outdir.join(&roast_args.outfile);
    let outpath = outpath.canonicalize().unwrap_or(outpath);

    let mut exclude_canonicalized_paths: Vec<PathBuf> =
        roast_args.exclude.clone().unwrap_or_default();

    exclude_canonicalized_paths = exclude_canonicalized_paths
        .iter()
        .map(|p| target_path.join(p).canonicalize().unwrap_or_default())
        // NOTE: This is important. as unwrap_or_default contains at least one element of
        // Path::from("") or a PathBuf::new()
        .filter(|p| !p.as_os_str().is_empty())
        .collect();

    debug!(?exclude_canonicalized_paths);

    if let Some(additional_paths) = &roast_args.additional_paths
    {
        process_additional_paths(
            additional_paths,
            &target_path,
            &exclude_canonicalized_paths,
            &setup_workdir,
            &roast_args,
        )?;
    }

    if let Some(include_paths) = &roast_args.include
    {
        process_include_paths(
            include_paths,
            &exclude_canonicalized_paths,
            &target_path,
            &setup_workdir,
            &roast_args,
        )?;
    }

    filter_paths(
        &target_path,
        &setup_workdir,
        roast_args.ignore_hidden,
        roast_args.ignore_git,
        &exclude_canonicalized_paths,
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
        info!("🧑‍🍳 Your new tarball is now in {}", &outpath.display());
    }

    tmp_binding.close().inspect_err(|e| {
        error!(?e, "Failed to delete temporary directory!");
    })?;

    Ok(())
}