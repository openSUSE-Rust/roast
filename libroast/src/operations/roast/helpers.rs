use rayon::prelude::*;
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
    Level,
    debug,
    error,
    info,
    trace,
    warn,
};

pub(crate) fn is_hidden(entry: &Path, hidden: bool, ignore_git: bool, root: &Path) -> bool
{
    let entry_str = entry.file_name().unwrap_or(entry.as_os_str());
    let entry_str = entry_str.to_string_lossy();
    debug!(?entry_str, ?root);
    let entry_canonicalized = entry.canonicalize().unwrap_or(entry.to_path_buf());
    let root_canonicalized = root.canonicalize().unwrap_or(root.to_path_buf());
    if entry_canonicalized == root_canonicalized
    {
        debug!(?entry_str, ?root, "not set as hidden because it's the root");
        false
    }
    else if entry_str.starts_with(".git")
    {
        debug!(?entry_str, ?root, ".git* path");
        ignore_git
    }
    else if entry_str.starts_with(".")
    {
        debug!(?entry_str, ?root, "dotfile/hidden file");
        hidden
    }
    else
    {
        debug!(?entry_str, ?root, "not hidden");
        false
    }
}

pub(crate) fn is_excluded(
    entry_as_path_canonicalized: &Path,
    exclude_canonicalized_paths: &[PathBuf],
) -> bool
{
    exclude_canonicalized_paths.par_iter().any(|exclude| {
        let is_stripped = entry_as_path_canonicalized.strip_prefix(exclude);
        debug!(?is_stripped, ?entry_as_path_canonicalized, ?exclude);
        is_stripped.is_ok()
    })
}

pub(crate) fn helper_archiver(
    entry_as_path_canonicalized: &Path,
    target_path: &Path,
    hidden: bool,
    ignore_git: bool,
    root: &Path,
    exclude_paths: &[PathBuf],
) -> io::Result<()>
{
    if entry_as_path_canonicalized.is_dir()
    {
        if !is_hidden(entry_as_path_canonicalized, hidden, ignore_git, root)
        {
            let entry_stripped_by_target_path = entry_as_path_canonicalized
                .strip_prefix(target_path)
                .unwrap_or(entry_as_path_canonicalized);
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
    }
    else if entry_as_path_canonicalized.is_file()
        && !is_hidden(entry_as_path_canonicalized, hidden, ignore_git, root)
    {
        let entry_stripped_by_target_path = entry_as_path_canonicalized
            .strip_prefix(target_path)
            .unwrap_or(entry_as_path_canonicalized);
        let entry_as_path_canonicalized_parent =
            entry_as_path_canonicalized.parent().unwrap_or(target_path);
        let genesis_path = &root.join(entry_stripped_by_target_path);
        let genesis_path_parent = genesis_path.parent().unwrap_or(root);
        if is_excluded(entry_as_path_canonicalized, exclude_paths)
            && (*genesis_path_parent != *root)
        {
            warn!(
                "⚠️ Adding file `{}` that is WITHIN an EXCLUDED directory `{}`.",
                &entry_as_path_canonicalized.display(),
                &entry_as_path_canonicalized_parent.display()
            );
        }
        fs::create_dir_all(genesis_path_parent)?;
        fs::copy(entry_as_path_canonicalized, genesis_path)?;
    }
    Ok(())
}

pub(crate) fn filter_paths(
    target_path: &Path,
    root: &Path,
    hidden: bool,
    ignore_git: bool,
    exclude_paths: &[PathBuf],
) -> io::Result<()>
{
    let target_dir = fs::read_dir(target_path)
        .inspect_err(|err| {
            error!(?err);
        })?
        .flatten();

    target_dir.par_bridge().into_par_iter().try_for_each(|entry| {
        let entry_as_path = &entry.path();
        let entry_as_path_canonicalized =
            &entry_as_path.canonicalize().unwrap_or(entry_as_path.to_path_buf());
        if exclude_paths.is_empty()
        {
            helper_archiver(
                entry_as_path_canonicalized,
                target_path,
                hidden,
                ignore_git,
                root,
                exclude_paths,
            )
        }
        else
        {
            if !is_excluded(entry_as_path_canonicalized, exclude_paths)
            {
                helper_archiver(
                    entry_as_path_canonicalized,
                    target_path,
                    hidden,
                    ignore_git,
                    root,
                    exclude_paths,
                )?;
            }
            Ok(())
        }
    })?;
    Ok(())
}
