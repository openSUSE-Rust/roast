use crate::operations::cli::RoastScmArgs;
use crate::operations::roast;
use crate::{common::Compression, utils::start_tracing};

use std::{
    io,
    path::{Path, PathBuf},
};

use git2::{AutotagOption, FetchOptions, Repository, build::RepoBuilder};
use rayon::prelude::*;
use tempfile;

#[allow(unused_imports)]
use tracing::{Level, debug, error, info, trace, warn};

fn git_clone2(url: &str, local_clone_dir: &Path, tag_or_branch: &str) -> io::Result<PathBuf> {
    let mut fetch_options = FetchOptions::new();
    let tag_options = AutotagOption::All;
    fetch_options.download_tags(tag_options);

    let mut builder = RepoBuilder::new();
    builder.fetch_options(fetch_options);
    builder.clone(url, local_clone_dir)?;
    let local_repository = Repository::open(local_clone_dir)?;
    local_repository.cleanup_state()?;

    let (object, reference) = local_repository.revparse_ext(tag_or_branch)?;
    local_repository.checkout_tree(&object, None)?;
    match reference {
        Some(gitref) => local_repository.set_head(gitref.name()?),
        None => local_repository.set_head_detached(object.id()),
    }?;

    let submodules = local_repository.submodules()?;

    submodules.par_iter.try_for_each(|mut subm| {
        subm.update(true, None)?;
        subm.open()?
    });

    Ok(local_clone_dir.to_path_buf())
}

pub fn roast_scm_opts(opts: &RoastScmArgs, start_trace: bool) -> io::Result<()> {
    if start_trace {
        start_tracing();
    }
    info!("⛓️🔥 Starting Roast SCM!");
    debug!(opts);

    Ok(())
}
