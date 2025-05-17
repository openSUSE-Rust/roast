#![allow(unused_imports)]
#![allow(dead_code)]
use crate::{
    operations::{
        cli::{
            RoastArgs,
            RoastScmArgs,
        },
        roast::{
            self,
            roast_opts,
        },
    },
    utils::start_tracing,
};
use git2::{
    AutotagOption,
    FetchOptions,
    Repository,
    build::RepoBuilder,
};
use std::{
    io,
    path::{
        Path,
        PathBuf,
    },
};
use tracing::{
    Level,
    debug,
    error,
    info,
    trace,
    warn,
};

fn git_clone2(url: &str, local_clone_dir: &Path, revision: &str, depth: i32) -> io::Result<()>
{
    let mut fetch_options = FetchOptions::new();
    let tag_options = AutotagOption::All;
    fetch_options.download_tags(tag_options);
    if depth > 0
    {
        fetch_options.depth(depth);
    }

    let mut builder = RepoBuilder::new();
    builder.fetch_options(fetch_options);
    builder.clone(url, local_clone_dir).map_err(|err| {
        error!(?err);
        io::Error::new(io::ErrorKind::Other, err.to_string())
    })?;
    let local_repository = Repository::open(local_clone_dir).map_err(|err| {
        error!(?err);
        io::Error::new(io::ErrorKind::Other, err.to_string())
    })?;
    local_repository.cleanup_state().map_err(|err| {
        error!(?err);
        io::Error::new(io::ErrorKind::Other, err.to_string())
    })?;

    let (object, reference) = local_repository.revparse_ext(revision).map_err(|err| {
        error!(?err);
        io::Error::new(io::ErrorKind::Other, err.to_string())
    })?;
    local_repository.checkout_tree(&object, None).map_err(|err| {
        error!(?err);
        io::Error::new(io::ErrorKind::Other, err.to_string())
    })?;
    match reference
    {
        Some(gitref) => local_repository.set_head(gitref.name().ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidInput, "No reference name found.")
        })?),
        None => local_repository.set_head_detached(object.id()),
    }
    .map_err(|err| {
        error!(?err);
        io::Error::new(io::ErrorKind::Other, err.to_string())
    })?;

    let submodules = local_repository.submodules().map_err(|err| {
        error!(?err);
        io::Error::new(io::ErrorKind::Other, err.to_string())
    })?;

    for mut subm in submodules
    {
        subm.update(true, None).map_err(|err| {
            error!(?err);
            io::Error::new(io::ErrorKind::Other, err.to_string())
        })?;
        subm.open().map_err(|err| {
            error!(?err);
            io::Error::new(io::ErrorKind::Other, err.to_string())
        })?;
    }

    Ok(())
}

pub fn roast_scm_opts(roast_scm_args: &RoastScmArgs, start_trace: bool) -> io::Result<()>
{
    if start_trace
    {
        start_tracing();
    }
    info!("⛓️🔥 Starting Roast SCM!");
    debug!(?roast_scm_args);
    let local_clone_dir = tempfile::TempDir::new()?;
    git_clone2(
        &roast_scm_args.git_repository_url,
        local_clone_dir.path(),
        &roast_scm_args.revision,
        roast_scm_args.depth,
    )?;
    let roast_args = RoastArgs {
        target: local_clone_dir.path().to_path_buf(),
        include: None,
        exclude: roast_scm_args.exclude.clone(),
        additional_paths: None,
        outfile: roast_scm_args.outfile.clone(),
        outdir: roast_scm_args.outdir.clone(),
        preserve_root: false,
        reproducible: roast_scm_args.reproducible,
        ignore_git: roast_scm_args.ignore_git,
        ignore_hidden: roast_scm_args.ignore_hidden,
    };

    roast_opts(&roast_args, false)
        .inspect(|ok| {
            info!("⛓️🔥 Finished Roast SCM!");
            debug!(?ok);
        })
        .inspect_err(|err| {
            error!(?err);
        })
}
