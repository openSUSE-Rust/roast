use crate::{
    operations::{
        cli::{
            RoastArgs,
            RoastScmArgs,
        },
        roast::roast_opts,
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
    path::Path,
};
use tracing::{
    debug,
    error,
    info,
};
use url::Url;

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
        io::Error::other(err.to_string())
    })?;
    let local_repository = Repository::open(local_clone_dir).map_err(|err| {
        error!(?err);
        io::Error::other(err.to_string())
    })?;
    local_repository.cleanup_state().map_err(|err| {
        error!(?err);
        io::Error::other(err.to_string())
    })?;

    let (object, reference) = local_repository.revparse_ext(revision).map_err(|err| {
        error!(?err);
        io::Error::other(err.to_string())
    })?;
    local_repository.checkout_tree(&object, None).map_err(|err| {
        error!(?err);
        io::Error::other(err.to_string())
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
        io::Error::other(err.to_string())
    })?;

    let submodules = local_repository.submodules().map_err(|err| {
        error!(?err);
        io::Error::other(err.to_string())
    })?;

    for mut subm in submodules
    {
        subm.update(true, None).map_err(|err| {
            error!(?err);
            io::Error::other(err.to_string())
        })?;
        subm.open().map_err(|err| {
            error!(?err);
            io::Error::other(err.to_string())
        })?;
    }

    Ok(())
}

fn process_filename_prefix_from_url(url_string: &str, revision: &str) -> io::Result<String>
{
    let url = Url::parse(url_string).map_err(|err| {
        error!(?err);
        io::Error::new(io::ErrorKind::InvalidInput, "Not able to parse URL string!")
    })?;
    let path_segments = url
        .path_segments()
        .map(|c| c.collect::<Vec<&str>>())
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Cannot generate basename!"))?;
    debug!(?path_segments);
    // NOTE: Some people copy `.git` e.g. https://github.com/octocat/octocat.git.
    let basename = match path_segments[1].rsplit_once(".git")
    {
        Some((basename, _)) => basename,
        None => path_segments[1],
    };

    let filename = if revision.is_empty()
    {
        basename.to_string()
    }
    else
    {
        format!("{}-{}", basename, revision)
    };

    Ok(filename)
}

pub fn roast_scm_opts(roast_scm_args: &RoastScmArgs, start_trace: bool) -> io::Result<()>
{
    if start_trace
    {
        start_tracing();
    }
    info!("⛓️🔥 Starting Roast SCM!");
    debug!(?roast_scm_args);
    let workdir = tempfile::TempDir::new()?;
    let workdir = if !roast_scm_args.is_temporary { &workdir.keep() } else { workdir.path() };

    let filename_prefix = process_filename_prefix_from_url(
        &roast_scm_args.git_repository_url,
        &roast_scm_args.revision,
    )?;
    let local_clone_dir = workdir.join(&filename_prefix);
    let local_clone_dir = local_clone_dir.as_path();

    let outfile = match roast_scm_args.outfile.clone()
    {
        Some(outfile) => outfile,
        None =>
        {
            // TODO: Maybe create a function that gives the file extension??
            let extension = match &roast_scm_args.compression
            {
                crate::common::Compression::Gz => "tar.gz",
                crate::common::Compression::Xz => "tar.xz",
                crate::common::Compression::Zst | crate::common::Compression::Zstd => "tar.zst",
                crate::common::Compression::Bz2 => "tar.bz",
                crate::common::Compression::Not => "tar",
            };
            let full_filename = format!("{}.{}", filename_prefix, extension);
            Path::new(&full_filename).to_path_buf()
        }
    };

    let git_url = &roast_scm_args.git_repository_url.to_string();

    info!(?git_url, "🫂 Cloning remote repository.");
    info!(?local_clone_dir, "🏃 Cloning to local directory...");
    git_clone2(git_url, &local_clone_dir, &roast_scm_args.revision, roast_scm_args.depth)?;
    info!(?git_url, "🫂 Finished cloning remote repository.");
    info!("🍄 Cloned to `{}`.", local_clone_dir.display());

    let roast_args = RoastArgs {
        target: local_clone_dir.to_path_buf(),
        include: None,
        exclude: roast_scm_args.exclude.clone(),
        additional_paths: None,
        outfile,
        outdir: roast_scm_args.outdir.clone(),
        preserve_root: true,
        reproducible: roast_scm_args.reproducible,
        ignore_git: roast_scm_args.ignore_git,
        ignore_hidden: roast_scm_args.ignore_hidden,
    };

    roast_opts(&roast_args, false)
        .inspect(|ok| {
            info!("⛓️🔥 Finished Roast SCM!");
            if roast_scm_args.is_temporary
            {
                info!(
                    "👁️ Locally cloned repository is not deleted and located at `{}`.",
                    local_clone_dir.display()
                );
            };
            debug!(?ok);
        })
        .inspect_err(|err| {
            error!(?err);
        })
}
