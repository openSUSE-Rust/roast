use crate::{
    operations::{
        cli::{
            RoastArgs,
            RoastScmArgs,
        },
        roast::roast_opts,
    },
    utils::{
        copy_dir_all,
        start_tracing,
    },
};
use core::str::FromStr;
use hifitime::{
    efmt::Format,
    prelude::*,
};

const CHANGELOG_LONG_SET_OF_DASHES: &str =
    "-------------------------------------------------------------------";
const CHANGELOG_DATE_TIME_FORMAT: &str = "%a %b %H:%M:%S %T %Y";

use git2::{
    AutotagOption,
    Branch,
    BranchType,
    Commit,
    FetchOptions,
    Object,
    Repository,
    Submodule,
    build::RepoBuilder,
};
use regex::Regex;
use std::{
    io::{
        self,
    },
    path::Path,
};
use tracing::{
    debug,
    error,
    info,
    warn,
};
use url::Url;

/// Performs a `git describe` operation on the repository.
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

/// Creates a local copy of the remote branch if the local branch does not exist
/// then checkout to that local branch.
fn remote_checkout_branch<'a>(
    local_repository: &'a Repository,
    branch: &'a Branch<'a>,
    remote_name: &str,
) -> io::Result<Object<'a>>
{
    let branch_ref = branch.get();
    let branch_commit = branch_ref.peel_to_commit().map_err(|err| {
        error!(?err);
        io::Error::other(err)
    })?;
    let branch_obj = branch_commit.as_object();
    let Some(branch_shortname) = branch_ref.shorthand()
    else
    {
        return Err(io::Error::other("No shortname or fullname found!"));
    };

    let refremote_path = format!("ref/remotes/{}/", remote_name);
    // NOTE: The branch ref will look like `refs/remotes/<name of remote>/<name of
    // branch>` so we `rsplit_once` just to get the name of the remote branch
    let local_branch_name = if let Some((_rest, last_name)) =
        branch_shortname.split_once(&refremote_path)
    {
        debug!(?_rest, ?last_name);
        let _ = local_repository.branch(last_name, &branch_commit, true).inspect_err(|err| {
            debug!(?err);
            debug!("This means the local branch exists and is the current HEAD of the repository!");
        });
        last_name
    }
    else if let Some((_rest, last_name)) =
        branch_shortname.split_once(&format!("{}/", remote_name))
    {
        debug!(?_rest, ?last_name);
        let _ = local_repository.branch(last_name, &branch_commit, true).inspect_err(|err| {
            debug!(?err);
            debug!("This means the local branch exists and is the current HEAD of the repository!");
        });
        last_name
    }
    else
    {
        let _ =
            local_repository.branch(branch_shortname, &branch_commit, true).inspect_err(|err| {
                debug!(?err);
                debug!(
                    "This means the local branch exists and is the current HEAD of the repository!"
                );
            });
        branch_shortname
    };

    debug!(?local_branch_name, "The local branch name:");

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

pub struct ChangelogDetails
{
    pub changelog: String,
    pub describe_string: String,
    pub commit_hash: String,
    pub tag_or_version: String,
    pub offset_since_current_commit: u32,
}

fn update_repo_from_ref<'a>(
    local_repository: &'a Repository,
    revision: &'a str,
) -> io::Result<Object<'a>>
{
    let object_ref_result = local_repository.revparse_ext(revision).map_err(|err| {
        error!(?err);
        io::Error::other(err)
    });

    if let Ok((object, reference)) = object_ref_result
    {
        info!("❤️ Found a valid revision tag or commit.");

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
        Ok(object)
    }
    else
    {
        // Otherwise, we'll just return an error here.
        Err(io::Error::other(format!("No revision `{}` found!", revision)))
    }
}
/// Helper function to clone a repository. Options are self-explanatory.
///
/// If a repository has submodules, it will always attempt to update a
/// repository's submodule that matches at a given commit.
///
/// The return type is `io::Result<ChangelogDetails>`. The `Ok` variant will
/// contain the changelog string. which can be further processed for other
/// means.
fn git_clone2(
    url: &str,
    local_clone_dir: &Path,
    revision: &str,
    depth: i32,
) -> io::Result<ChangelogDetails>
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

    let branch_type = BranchType::Remote;
    let repository_config = local_repository.config().map_err(|err| {
        error!(?err);
        io::Error::other(err)
    })?;

    let mut config_entries = repository_config.entries(None).map_err(|err| {
        error!(?err);
        io::Error::other(err)
    })?;

    let mut default_remote_name = "origin".to_string();
    while let Some(entry) = config_entries.next()
    {
        let entry = entry.map_err(|err| {
            error!(?err);
            io::Error::other(err)
        })?;
        let Some(entry_name) = entry.name()
        else
        {
            return Err(io::Error::other("No valid entry name!"));
        };

        let Some(entry_value) = entry.value()
        else
        {
            return Err(io::Error::other("No valid entry value!"));
        };

        debug!(?entry_name, ?entry_value);

        if *entry_name == *"clone.defaultRemoteName"
        {
            default_remote_name = entry_value.to_string();
        }
    }

    let remote_branch_name = format!("{}/{}", &default_remote_name, revision);

    let find_branch_result = local_repository.find_branch(&remote_branch_name, branch_type);
    let resulting_git_object = match find_branch_result
    {
        Ok(ref remote_branch_to_copy) =>
        {
            match remote_checkout_branch(
                &local_repository,
                remote_branch_to_copy,
                &default_remote_name,
            )
            {
                Ok(obj) => obj,
                Err(err) =>
                {
                    debug!(?err);
                    // Then it's likely a tag or a commitish
                    update_repo_from_ref(&local_repository, revision)?
                }
            }
        }
        Err(err) =>
        {
            debug!(?err);
            // Then it's likely a tag or a commitish
            update_repo_from_ref(&local_repository, revision)?
        }
    };

    // Then recursively just update any submodule of the repository to match
    // the index and tree.
    let mut submodules = local_repository.submodules().map_err(|err| {
        error!(?err);
        io::Error::other(err.to_string())
    })?;

    submodules.iter_mut().try_for_each(|mut subm| update_submodule(&mut subm))?;

    changelog_details_generate(&local_repository, &resulting_git_object)
}

fn update_submodule(subm: &mut Submodule) -> io::Result<()>
{
    subm.update(true, None).map_err(|err| {
        error!(?err);
        io::Error::other(err.to_string())
    })?;
    subm.open().map_err(|err| {
        error!(?err);
        io::Error::other(err.to_string())
    })?;
    Ok(())
}

fn changelog_details_generate(
    local_repository: &Repository,
    git_object: &Object,
) -> io::Result<ChangelogDetails>
{
    let mut bulk_commit_message = String::new();
    let mut number_of_refs_since_commit: u32 = 0;
    let mut commit_hash: String = String::new();
    let mut tag_or_version: String = String::new();

    // NOTE: Delete tag so we can get the previous tag to describe.
    // NOTE: Run `describe_revision` twice. one for output information and removing
    // a tag if there is, and the other to generate changelog.
    let describe_string = describe_revision(git_object)?;
    info!(?describe_string, "Result of `git describe`: ");
    if let Some(resulting_tag) = git_object.as_tag()
    {
        // Since the object directly points to a tag
        // we delete it auto-magically
        let resulting_tag_string = resulting_tag.name().unwrap_or_default();
        commit_hash = "".to_string();
        tag_or_version = resulting_tag_string.to_string();
        let tagged_obj = resulting_tag.target().map_err(|err| {
            debug!(?err);
            io::Error::other(err)
        })?;
        info!("Git object is a tag: {}", &resulting_tag_string);
        local_repository.tag_delete(resulting_tag_string).map_err(|err| {
            error!(?err);
            io::Error::other(err)
        })?;
        let describe_string_after_tag_delete = describe_revision(&tagged_obj)?;
        let tagged_commit = tagged_obj.peel_to_commit().map_err(|err| {
            error!(?err);
            io::Error::other(err)
        })?;

        if let Some((rest, _hash)) = describe_string_after_tag_delete.rsplit_once("-")
        {
            if let Some((_new_rest, new_cunt)) = rest.rsplit_once("-")
            {
                let new_count = new_cunt.parse::<u32>().map_err(|err| {
                    error!(?err);
                    io::Error::other(err)
                })?;
                mutate_bulk_commit_message_string(
                    &tagged_commit,
                    new_count,
                    &mut bulk_commit_message,
                )?;
            }
        };
    }
    else if let Some(commitish) = git_object.as_commit()
    {
        // NOTE: Let's process if the describe string has a tag.
        // The format is long-format so if it's just a branch with no tag,
        // it will be `heads/<name-of-branch>-<number of commits since tag>-g<commit
        // hash of current commit>`. If it has a tag, it will be `<tag>-<number
        // of commits since tag>-g<current commit hash of current commit>`
        // NOTE: Some users pass a hash, and the git object won't be considered as a
        // tag. so we have to split the `describe_string`.
        if let Some((_prefix, long_name)) = describe_string.split_once("heads/")
        {
            if let Some((the_rest, g_hash)) = long_name.rsplit_once("-")
            {
                commit_hash = g_hash.to_string();
                if let Some((tag_, number_string)) = the_rest.rsplit_once("-")
                {
                    tag_or_version = tag_.to_string();
                    number_of_refs_since_commit = number_string.parse::<u32>().map_err(|err| {
                        error!(?err);
                        io::Error::other(err)
                    })?;
                }
                if number_of_refs_since_commit == 0 || the_rest.is_empty()
                {
                    local_repository.tag_delete(&tag_or_version).map_err(|err| {
                        error!(?err);
                        io::Error::other(err)
                    })?;
                }
            }
        }
        else if let Some((the_rest, g_hash)) = describe_string.rsplit_once("-")
        {
            commit_hash = g_hash.to_string();
            if let Some((tag_, number_string)) = the_rest.rsplit_once("-")
            {
                tag_or_version = tag_.to_string();
                number_of_refs_since_commit = number_string.parse::<u32>().map_err(|err| {
                    error!(?err);
                    io::Error::other(err)
                })?;
            }
            if number_of_refs_since_commit == 0 || the_rest.is_empty()
            {
                local_repository.tag_delete(&tag_or_version).map_err(|err| {
                    error!(?err);
                    io::Error::other(err)
                })?;
                let describe_string_after_tag_delete = describe_revision(git_object)?;
                let tagged_commit = git_object.peel_to_commit().map_err(|err| {
                    error!(?err);
                    io::Error::other(err)
                })?;

                if let Some((rest, _hash)) = describe_string_after_tag_delete.rsplit_once("-")
                {
                    if let Some((_new_rest, new_cunt)) = rest.rsplit_once("-")
                    {
                        let new_count = new_cunt.parse::<u32>().map_err(|err| {
                            error!(?err);
                            io::Error::other(err)
                        })?;
                        mutate_bulk_commit_message_string(
                            &tagged_commit,
                            new_count,
                            &mut bulk_commit_message,
                        )?;
                    }
                };
            }
        }
        else
        {
            // Perform a revwalk. This means there were no tags! And we only got a hash
            commit_hash = format!("g{}", commitish.id());
            let mut revwalk = local_repository.revwalk().map_err(|err| {
                error!(?err);
                io::Error::other(err)
            })?;
            revwalk.push_head().map_err(|err| {
                error!(?err);
                io::Error::other(err)
            })?;
            revwalk.set_sorting(git2::Sort::TIME).map_err(|err| {
                error!(?err);
                io::Error::other(err)
            })?;
            let mut start_count = false;
            let mut count = 0;

            revwalk.into_iter().try_for_each(|rev| {
                let commit = match rev
                {
                    Ok(rev_) => local_repository.find_commit(rev_).map_err(io::Error::other)?,
                    Err(err) => return Err(io::Error::other(err)),
                };
                debug!(?rev, ?commit);
                if commit.id() == commitish.id()
                {
                    start_count = true
                }
                if start_count
                {
                    count += 1;
                }
                Ok(())
            })?;

            number_of_refs_since_commit = count;
        }
        mutate_bulk_commit_message_string(
            commitish,
            number_of_refs_since_commit,
            &mut bulk_commit_message,
        )?;
    }
    else
    {
        return Err(io::Error::other("Object is not a commit nor a tag!"));
    }

    // Rerun `describe_revision` here for DEBUG
    let describe_string_for_debug = describe_revision(git_object)?;
    debug!(
        ?describe_string_for_debug,
        "Result of `git describe` after ATTEMPTING to remove a tag: "
    );
    if !&bulk_commit_message.is_empty()
    {
        info!("✍🏻 You can copy the changelog below:");
        println!("{}", &bulk_commit_message);
    }
    else
    {
        warn!("⚠️📋 No changelog generated.");
    }

    Ok(ChangelogDetails {
        changelog: bulk_commit_message,
        describe_string,
        commit_hash,
        tag_or_version,
        offset_since_current_commit: number_of_refs_since_commit,
    })
}

fn mutate_bulk_commit_message_string(
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

    parents.into_iter().try_for_each(|parent| {
        let new_countdown = countdown - 1;
        mutate_bulk_commit_message_string(&parent, new_countdown, bulk_commit_message)
    })?;

    Ok(())
}

fn process_basename_from_url(url_string: &str) -> io::Result<String>
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
    Ok(basename.to_string())
}

fn process_filename_from_url_and_revision(url_string: &str, revision: &str) -> io::Result<String>
{
    let basename = process_basename_from_url(url_string)?;
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

/// Edit the tag or version as desired. For example,
/// wezterm's version format is a date but separated with dashes:
/// 20220330-<time>-<gitHash> we need to turn the dashes into `.` ->
/// 20220330.<time>.<gitHash>. This function allows that using the `regex`
/// crate.
fn rewrite_version_or_revision_from_changelog_details(
    changelog_details: &ChangelogDetails,
    roast_scm_args: &RoastScmArgs,
) -> io::Result<String>
{
    let mut stub_format = String::new();
    if !changelog_details.tag_or_version.is_empty()
    {
        if let Some(versionrewriteregex) = &roast_scm_args.versionrewriteregex
        {
            if let Some(versionrewrite_pattern) = &roast_scm_args.versionrewritepattern
            {
                let versionformat = Regex::new(versionrewriteregex).map_err(|err| {
                    error!(?err);
                    io::Error::other(err)
                })?;
                let after = versionformat
                    .replace_all(&changelog_details.tag_or_version, versionrewrite_pattern);
                stub_format.push_str(&after);
            }
        }
        else
        {
            stub_format.push_str(&changelog_details.tag_or_version);
        }
        if changelog_details.offset_since_current_commit > 0
        {
            let git_offset = format!("+git{}", changelog_details.offset_since_current_commit);
            stub_format.push_str(&git_offset);
            if !changelog_details.commit_hash.is_empty()
            {
                let git_hash_section = format!(".{}", changelog_details.commit_hash);
                stub_format.push_str(&git_hash_section);
            }
        }
    }
    else
    {
        let git_offset = format!("0+git{}", changelog_details.offset_since_current_commit);
        stub_format.push_str(&git_offset);
        if !changelog_details.commit_hash.is_empty()
        {
            let git_hash_section = format!(".{}", changelog_details.commit_hash);
            stub_format.push_str(&git_hash_section);
        }
    }
    Ok(stub_format)
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

    let git_url = &roast_scm_args.git_repository_url.to_string();

    info!(?git_url, "🫂 Cloning remote repository.");
    info!(?workdir, "🏃 Cloning to local directory...");

    let changelog_details =
        git_clone2(git_url, workdir, &roast_scm_args.revision, roast_scm_args.depth)?;

    let final_revision_format =
        rewrite_version_or_revision_from_changelog_details(&changelog_details, roast_scm_args)?;

    let filename_prefix = process_filename_from_url_and_revision(
        &roast_scm_args.git_repository_url,
        &final_revision_format,
    )?;

    let new_workdir_for_copy = tempfile::TempDir::new()?;
    let local_copy_dir = new_workdir_for_copy.path().join(&filename_prefix);

    copy_dir_all(workdir, &local_copy_dir)?;

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
    info!(?git_url, "🫂 Finished cloning remote repository.");
    info!("🍄 Cloned to `{}`.", workdir.display());

    let roast_args = RoastArgs {
        target: local_copy_dir,
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

    if roast_scm_args.changesgenerate
    {
        if let Some(changesauthor) = &roast_scm_args.changesauthor
        {
            let changesoutfile = match &roast_scm_args.changesoutfile
            {
                Some(v) => v,
                None => &std::env::current_dir()
                    .map_err(|err| {
                        error!(?err);
                        io::Error::other(err)
                    })?
                    .join(format!("{}.changes", process_basename_from_url(git_url)?)),
            };

            let time_format = Format::from_str(CHANGELOG_DATE_TIME_FORMAT).map_err(|err| {
                error!(?err);
                io::Error::other(err)
            })?;
            let time_now = Epoch::now().map_err(|err| {
                error!(?err);
                io::Error::other(err)
            })?;
            let formatted_time_now = Formatter::new(time_now, time_format);
            let changelog_header = format!(
                "{}\n{} - {}",
                CHANGELOG_LONG_SET_OF_DASHES, formatted_time_now, changesauthor
            );
            let update_statement = format!("- Update to version {}:", final_revision_format);
            let mut final_changelog_lines = String::new();
            if !changelog_details.changelog.trim().is_empty()
            {
                let changelog_lines = changelog_details.changelog.split('\n');
                changelog_lines.into_iter().for_each(|line| {
                    let format_with_two_spaces = format!("  {}\n", line);
                    final_changelog_lines.push_str(&format_with_two_spaces);
                });
            }
            else
            {
                final_changelog_lines.push_str("  * NO CHANGELOG\n");
            }

            let changes_string_from_file = match std::fs::File::create_new(changesoutfile)
            {
                Ok(file) =>
                {
                    debug!(?file);
                    std::fs::read_to_string(changesoutfile)
                }
                Err(err) =>
                {
                    debug!(?err);
                    // If the file exists
                    if changesoutfile.exists()
                    {
                        std::fs::read_to_string(changesoutfile)
                    }
                    else
                    {
                        Err(io::Error::other(err))
                    }
                }
            }?;

            let final_changes_string_for_file = format!(
                "{}\n\n{}\n{}{}",
                changelog_header, update_statement, final_changelog_lines, changes_string_from_file
            );
            std::fs::write(changesoutfile, &final_changes_string_for_file).inspect(|_| {
                info!("🗒️ Successfully generated changelog to `{}`.", &changesoutfile.display());
            })?
        }
        else
        {
            return Err(io::Error::other("No changes author provided."));
        }
    }

    roast_opts(&roast_args, false)
        .map(|ok| {
            debug!(?ok);
            info!("⛓️🔥 Finished Roast SCM!");
            if !roast_scm_args.is_temporary
            {
                info!(
                    "👁️ Locally cloned repository is not deleted and located at `{}`.",
                    workdir.display()
                );
                return Some(workdir.to_path_buf());
            };
            None
        })
        .inspect_err(|err| {
            error!(?err);
        })
}
