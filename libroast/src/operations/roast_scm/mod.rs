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
    Branch,
    BranchType,
    Commit,
    FetchOptions,
    Object,
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
    warn,
};
use url::Url;

fn describe_revision(object: &Object) -> io::Result<String>
{
    let mut describe_options = git2::DescribeOptions::default();
    let mut describe_format = git2::DescribeFormatOptions::new();
    describe_options.describe_all();
    describe_options.describe_tags();
    describe_options.show_commit_oid_as_fallback(true);
    describe_format.always_use_long_format(true);
    let describe_string = if let Ok(describe_with_tag) = object.describe(&describe_options)
    {
        describe_with_tag.format(Some(&describe_format)).map_err(|err| {
            warn!(?err);
            io::Error::other(err)
        })?
    }
    else
    {
        let mut new_describe_options = git2::DescribeOptions::default();
        new_describe_options.describe_all();
        let new_describe = object.describe(&new_describe_options).map_err(|err| {
            error!(?err);
            io::Error::other(err)
        })?;
        new_describe.format(Some(&describe_format)).map_err(|err| {
            error!(?err);
            io::Error::other(err)
        })?
    };
    Ok(describe_string)
}

// NOTE: checkout only remote branches. It does not make sense to checkout local
// branches
fn checkout_branch<'a>(
    local_repository: &'a Repository,
    branch: &'a Branch<'a>,
) -> io::Result<Object<'a>>
{
    let branch_ref = branch.get();
    let branch_commit = branch_ref.peel_to_commit().map_err(|err| {
        error!(?err);
        io::Error::other(err)
    })?;
    let Some(branch_shortname) = branch_ref.shorthand()
    else
    {
        return Err(io::Error::other("No shortname or fullname found!"));
    };
    // NOTE: The branch ref will look like `refs/remotes/<name of remote>/<name of
    // branch>` so we `rsplit_once` just to get the name of the remote branch
    if let Some((_rest, last_name)) = branch_shortname.rsplit_once("/")
    {
        local_repository.branch(last_name, &branch_commit, true).map_err(|err| {
            error!(?err);
            io::Error::other(err)
        })?;
        last_name
    }
    else
    {
        // NOTE: Not sure if this is the best approach
        local_repository.branch(branch_shortname, &branch_commit, true).map_err(|err| {
            error!(?err);
            io::Error::other(err)
        })?;
        branch_shortname
    };
    let branch_obj = branch_commit.as_object();

    local_repository.checkout_tree(branch_obj, None).map_err(|err| {
        error!(?err);
        io::Error::other(err)
    })?;
    local_repository.set_head_detached(branch_obj.id()).map_err(|err| {
        error!(?err);
        io::Error::other(err)
    })?;
    Ok(branch_obj.to_owned())
}

fn revision_in_remote_exists(branch: &Branch, revision: &str) -> bool
{
    if let Some(name) = &branch.get().name()
    {
        // NOTE: For whatever reason, `refs/remotes/<name of remote>/HEAD` is not
        // a valid branch name 🥴
        match name.split_once(revision)
        {
            Some((refremote, remote_branch)) =>
            {
                debug!(?refremote, ?remote_branch);
                if let Some(should_be_slash) = refremote.chars().last()
                {
                    should_be_slash == '/'
                }
                else
                {
                    false
                }
            }
            _ => false,
        }
    }
    else
    {
        false
    }
}
/// Helper function to clone a repository. Options are self-explanatory.
///
/// If a repository has submodules, it will always attempt to update a
/// repository's submodule that matches at a given commit.
fn git_clone2(url: &str, local_clone_dir: &Path, revision: &str, depth: i32) -> io::Result<()>
{
    let mut fetch_options = FetchOptions::new();
    let tag_options = AutotagOption::All;
    fetch_options.download_tags(tag_options);

    if depth > 0
    {
        warn!(
            "⚠️ Careful when setting depth. You might lose some refs and important information \
             that might affect `git describe` if set too low."
        );
        warn!(
            "⚠️ Careful when setting depth. You might lose some refs that might affect finding \
             your revision string."
        );
        warn!("⚠️ Depth is currently set to `{}`", depth);
        fetch_options.depth(depth);
    }

    let mut builder = RepoBuilder::new();
    builder.fetch_options(fetch_options);
    // builder.branch(revision);
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

    // If a reference does not exist, let's check all out remote branches, thus,
    // creating local copies.
    let branch_type = BranchType::Remote;
    let mut branches = local_repository
        .branches(Some(branch_type))
        .map_err(|err| {
            error!(?err);
            io::Error::other(err)
        })?
        .flatten();

    let remote_branch_to_copy =
        branches.find(|(branch, _branch_type)| revision_in_remote_exists(branch, revision));

    // for (branch, _) in branches {
    //     if let Some(name) = &branch.get().name() {
    //         // NOTE: For whatever reason, `refs/remotes/<name of remote>/HEAD` is
    // not         // a valid branch name 🥴
    //         if let Some((refremote, remote_branch)) = name.split_once(revision) {
    //             debug!(?refremote, ?remote_branch);
    //             if let Some(should_be_slash) = refremote.chars().last() {
    //                 if should_be_slash == '/' {
    //                     checkout_branch(&local_repository, &branch)?;
    //                     break;
    //                 }
    //             }
    //         }
    //     }
    // }

    let resulting_git_object = if let Some((ref found_branch, _branch_type)) = remote_branch_to_copy
    {
        checkout_branch(&local_repository, found_branch).map_err(|err| {
            error!(?err);
            io::Error::other(err)
        })?
    }
    else
    {
        // Then it's likely a tag or a commitish
        let object_ref_result = local_repository.revparse_ext(revision).map_err(|err| {
            error!(?err);
            io::Error::other(err)
        });

        if let Ok((object, reference)) = object_ref_result
        {
            info!("❤️ Found a valid revision tag or commit.");
            // TODO: Move this describe logic to another function
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
            object
        }
        else
        {
            // Otherwise, we'll just return an error here.
            return Err(io::Error::other(format!("No revision `{}` found!", revision)));
        }
    };
    // Then recursively just update any submodule of the repository to match
    // the index and tree.
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

    // NOTE: Delete tag so we can get the previous tag to describe.
    // NOTE: Run `describe_revision` twice. one for output information and removing
    // a tag if there is, and the other to generate changelog.
    let describe_string = describe_revision(&resulting_git_object)?;
    info!(?describe_string, "Result of `git describe`: ");
    if let Some(resulting_tag) = resulting_git_object.as_tag()
    {
        let resulting_tag_string = resulting_tag.name().unwrap_or_default();
        info!("Git object is a tag: {}", &resulting_tag_string);
        local_repository.tag_delete(resulting_tag_string).map_err(|err| {
            error!(?err);
            io::Error::other(err)
        })?;
    }
    else
    {
        // NOTE: Let's process if the describe string has a tag.
        // The format is long-format so if it's just a branch with no tag,
        // it will be `heads/<name-of-branch>-g<short commit hash>`.
        // If it has a tag, it will be `<tag>-<number of commits since tag>-g<current
        // commit hash pointed by HEAD>`
        if let Some((_prefix, long_name)) = describe_string.split_once("heads/")
        {
            if let Some((tag_string, _)) = long_name.split_once("-")
            {
                local_repository.tag_delete(tag_string).map_err(|err| {
                    error!(?err);
                    io::Error::other(err)
                })?;
            }
        }
        else if let Some((tag_string, _)) = describe_string.split_once("-")
        {
            local_repository.tag_delete(tag_string).map_err(|err| {
                error!(?err);
                io::Error::other(err)
            })?;
        }
    }
    // Rerun `describe_revision` here for changelog generation.
    let describe_string = describe_revision(&resulting_git_object)?;
    debug!(?describe_string, "Result of `git describe` after ATTEMPTING to remove a tag: ");
    let tunc_count = if let Some((rest, _g_hash)) = describe_string.rsplit_once("-")
    {
        if let Some((_new_rest, new_cunt)) = rest.rsplit_once("-")
        {
            let new_tunc = new_cunt.parse::<u32>().map_err(|err| {
                error!(?err);
                io::Error::other(err)
            })?;
            if new_tunc > 0 { new_tunc } else { 5 }
        }
        else
        {
            5
        }
    }
    else
    {
        5
    };
    let mut bulk_commit_message = String::new();
    if let Some(commitish) = resulting_git_object.as_commit()
    {
        bad_parenting(commitish, tunc_count, &mut bulk_commit_message)?;
    }
    else
    {
        // NOTE: at this point, it's clearly a tag
        let tag = resulting_git_object
            .as_tag()
            .ok_or_else(|| io::Error::other("Object does not point to a tag."))?;
        let tagged_obj = tag.target().map_err(|err| {
            error!(?err);
            io::Error::other(err)
        })?;
        let describe_string = describe_revision(&tagged_obj)?;
        let tagged_commit = tagged_obj.peel_to_commit().map_err(|err| {
            error!(?err);
            io::Error::other(err)
        })?;

        let tunc_count = if let Some((rest, _hash)) = describe_string.rsplit_once("-")
        {
            if let Some((_new_rest, new_cunt)) = rest.rsplit_once("-")
            {
                let new_cunt = new_cunt.parse::<u32>().map_err(|err| {
                    error!(?err);
                    io::Error::other(err)
                })?;
                if new_cunt > 0 { new_cunt } else { 5 }
            }
            else
            {
                5 // TODO: we might want to change this to something else but for now, this should suffice.
            }
        }
        else
        {
            5
        };
        bad_parenting(&tagged_commit, tunc_count, &mut bulk_commit_message)?;
    }
    if !&bulk_commit_message.is_empty()
    {
        info!("✍🏻 Copy the changelog below:");
        println!("{}", &bulk_commit_message);
    }
    else
    {
        warn!("⚠️📋 No changelog generated.");
    }
    Ok(())
}

fn bad_parenting(
    commit: &Commit,
    countdown: u32,
    bulk_commit_message: &mut String,
) -> io::Result<()>
{
    if countdown == 0
    {
        return Ok(());
    }
    let parents = commit.parents();
    let hash = commit.id().to_string();
    debug!("Commit hash: {}", hash);
    let summary = commit.summary().unwrap_or_default();
    if !summary.is_empty()
    {
        debug!("Commit summary: {}", summary);
        let format_summary = format!("* {}", &summary);
        bulk_commit_message.push_str(&format_summary);
        bulk_commit_message.push('\n');
    }
    for parent in parents
    {
        let new_countdown = countdown - 1;
        bad_parenting(&parent, new_countdown, bulk_commit_message)?;
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

/// Creates a tarball from a given URL. URL must be a valid remote git
/// repository.
///
/// It uses `crate::operations::roast` under the hood. Locally cloned
/// repositories can be not deleted if the `crate::cli::RoastScmArgs` has its
/// field `is_temporary` set to `false`.
pub fn roast_scm_opts(
    roast_scm_args: &RoastScmArgs,
    start_trace: bool,
) -> io::Result<Option<std::path::PathBuf>>
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
            let extension = &roast_scm_args.compression.to_extension();
            let full_filename = format!("{}{}", filename_prefix, extension);
            Path::new(&full_filename).to_path_buf()
        }
    };

    let git_url = &roast_scm_args.git_repository_url.to_string();

    info!(?git_url, "🫂 Cloning remote repository.");
    info!(?local_clone_dir, "🏃 Cloning to local directory...");
    git_clone2(git_url, local_clone_dir, &roast_scm_args.revision, roast_scm_args.depth)?;
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
        .map(|ok| {
            debug!(?ok);
            info!("⛓️🔥 Finished Roast SCM!");
            if !roast_scm_args.is_temporary
            {
                info!(
                    "👁️ Locally cloned repository is not deleted and located at `{}`.",
                    local_clone_dir.display()
                );
                return Some(local_clone_dir.to_path_buf());
            };
            None
        })
        .inspect_err(|err| {
            error!(?err);
        })
}
