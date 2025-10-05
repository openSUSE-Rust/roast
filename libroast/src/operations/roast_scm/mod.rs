use crate::{
    operations::{
        cli::{RoastArgs, RoastScmArgs, print_completions},
        roast::roast_opts,
    },
    utils::{copy_dir_all, start_tracing},
};
use clap::CommandFactory;
use core::str::FromStr;
use hifitime::{efmt::Format, prelude::*};

const CHANGELOG_LONG_SET_OF_DASHES: &str =
    "-------------------------------------------------------------------";
const CHANGELOG_DATE_TIME_FORMAT: &str = "%a %b %d %H:%M:%S %T %Y";

use git2::{
    AutotagOption, Branch, BranchType, Commit, FetchOptions, Object, Oid, Repository, Submodule,
    SubmoduleUpdateOptions, build::RepoBuilder,
};
use regex::Regex;
use std::{
    fs::read_to_string,
    io::{self},
    path::{Path, PathBuf},
};
use tracing::{debug, error, info, warn};
use url::Url;

/// Performs a `git describe` operation on the repository.
fn describe_revision(object: &Object) -> io::Result<String> {
    let mut describe_options = git2::DescribeOptions::default();
    let mut describe_format = git2::DescribeFormatOptions::new();
    describe_options.describe_all();
    describe_options.describe_tags();
    describe_options.show_commit_oid_as_fallback(true);
    describe_format.always_use_long_format(true);
    let describe_string = if let Ok(describe_with_tag) = object.describe(&describe_options) {
        describe_with_tag.format(Some(&describe_format)).map_err(|err| {
            warn!(?err);
            io::Error::other(err)
        })?
    } else {
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
) -> io::Result<Object<'a>> {
    let branch_ref = branch.get();
    let branch_commit = branch_ref.peel_to_commit().map_err(|err| {
        error!(?err);
        io::Error::other(err)
    })?;
    let branch_obj = branch_commit.as_object();
    let Some(branch_shortname) = branch_ref.shorthand() else {
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
    } else if let Some((_rest, last_name)) =
        branch_shortname.split_once(&format!("{}/", remote_name))
    {
        debug!(?_rest, ?last_name);
        let _ = local_repository.branch(last_name, &branch_commit, true).inspect_err(|err| {
            debug!(?err);
            debug!("This means the local branch exists and is the current HEAD of the repository!");
        });
        last_name
    } else {
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

pub struct ChangelogDetails {
    pub changelog: String,
    pub describe_string: String,
    pub commit_hash: String,
    pub tag_or_version: String,
    pub offset_since_current_commit: u32,
}

fn update_repo_from_ref<'a>(
    local_repository: &'a Repository,
    revision: &'a str,
) -> io::Result<Object<'a>> {
    let object_ref_result = local_repository.revparse_ext(revision).map_err(|err| {
        error!(?err);
        io::Error::other(err)
    });

    if let Ok((object, reference)) = object_ref_result {
        info!("❤️ Found a valid revision tag or commit.");

        local_repository.checkout_tree(&object, None).map_err(|err| {
            error!(?err);
            io::Error::other(err.to_string())
        })?;

        match reference {
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
    } else {
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
) -> io::Result<ChangelogDetails> {
    let mut fetch_options = FetchOptions::new();
    let tag_options = AutotagOption::All;
    fetch_options.download_tags(tag_options);

    if depth > 0 {
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
    while let Some(entry) = config_entries.next() {
        let entry = entry.map_err(|err| {
            error!(?err);
            io::Error::other(err)
        })?;
        let Some(entry_name) = entry.name() else {
            return Err(io::Error::other("No valid entry name!"));
        };

        let Some(entry_value) = entry.value() else {
            return Err(io::Error::other("No valid entry value!"));
        };

        debug!(?entry_name, ?entry_value);

        if *entry_name == *"clone.defaultRemoteName" {
            default_remote_name = entry_value.to_string();
        }
    }

    let remote_branch_name = format!("{}/{}", &default_remote_name, revision);

    let find_branch_result = local_repository.find_branch(&remote_branch_name, branch_type);
    let resulting_git_object = match find_branch_result {
        Ok(ref remote_branch_to_copy) => {
            match remote_checkout_branch(
                &local_repository,
                remote_branch_to_copy,
                &default_remote_name,
            ) {
                Ok(obj) => obj,
                Err(err) => {
                    debug!(?err);
                    // Then it's likely a tag or a commitish
                    update_repo_from_ref(&local_repository, revision)?
                }
            }
        }
        Err(err) => {
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

    submodules.iter_mut().try_for_each(|subm| update_submodule(&local_repository, subm))?;

    changelog_details_generate(&local_repository, &resulting_git_object)
}

fn update_submodule(local_repository: &Repository, subm: &mut Submodule) -> io::Result<()> {
    let Some(local_clone_dir) = local_repository.workdir() else {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Repository workdir not found!"));
    };
    let submodule_path_in_workdir = local_clone_dir.join(subm.path());
    info!("Cloning submodule at path: `{}`", submodule_path_in_workdir.display());
    let mut submodule_update_options = SubmoduleUpdateOptions::default();
    if !submodule_path_in_workdir.exists() {
        subm.init(true).map_err(|err| {
            error!(?err);
            error!("Error happened here in init");
            io::Error::other(err)
        })?;
        subm.repo_init(true).map_err(|err| {
            error!(?err);
            error!("Error happened here in repo init");
            io::Error::other(err)
        })?;
        subm.clone(Some(&mut submodule_update_options)).map_err(|err| {
            error!(?err);
            error!("Error happened here in clone");
            io::Error::other(err)
        })?;
        subm.add_finalize().map_err(|err| {
            error!(?err);
            error!("Error happened here in add_finalize");
            io::Error::other(err)
        })?;
        subm.sync().map_err(|err| {
            error!(?err);
            error!("Error happened here in add_finalize");
            io::Error::other(err)
        })?;
        subm.add_to_index(true).map_err(|err| {
            error!(?err);
            error!("Error happened here in index");
            io::Error::other(err)
        })?;
    } else {
        // NOTE: The reason it's inside else-block is because
        // the parent repository has not committed the "new" submodule.
        // If we do remove put this outside the else-block, it will
        // error that it could not find the ID of the submodule
        subm.update(true, Some(&mut submodule_update_options)).map_err(|err| {
            error!(?err);
            error!("Error happened here in update");
            io::Error::other(err)
        })?;
    }
    let subm_repo = subm.open().map_err(|err| {
        error!(?err);
        error!("Error happened here in open");
        io::Error::other(err)
    })?;
    let mut subm_repo_submodules = subm_repo.submodules().map_err(|err| {
        error!(?err);
        io::Error::other(err)
    })?;
    subm_repo_submodules
        .iter_mut()
        .try_for_each(|subm_repo_submodule| update_submodule(&subm_repo, subm_repo_submodule))?;
    Ok(())
}

fn get_commit<'a>(git_object: &'a Object<'a>) -> io::Result<Commit<'a>> {
    let commit = if let Some(resulting_tag) = git_object.as_tag() {
        // Since the object directly points to a tag
        // we delete it auto-magically
        let resulting_tag_string = resulting_tag.name().unwrap_or_default();
        let tagged_obj = resulting_tag.target().map_err(|err| {
            debug!(?err);
            io::Error::other(err)
        })?;
        info!("Git object is a tag: {}", &resulting_tag_string);

        tagged_obj.peel_to_commit().map_err(|err| {
            error!(?err);
            io::Error::other(err)
        })?
    } else if let Some(commitish) = git_object.as_commit() {
        commitish.to_owned()
    } else {
        return Err(io::Error::other("Object is not a commit nor a tag!"));
    };
    Ok(commit)
}

fn get_number_of_commits_since(
    local_repository: &Repository,
    from_this_describe_string: &str,
    to_this_commit: &Commit,
) -> io::Result<u32> {
    // NOTE: Let's process if the describe string has a tag.
    // The format is long-format so if it's just a branch with no tag,
    // it will be `heads/<name-of-branch>-<number of commits since tag>-g<commit
    // hash of current commit>`. If it has a tag, it will be `<tag>-<number
    // of commits since tag>-g<current commit hash of current commit>`
    // NOTE: Some users pass a hash, and the git object won't be considered as a
    // tag. so we have to split the `describe_string`.
    if let Some((_prefix, long_name)) = from_this_describe_string.split_once("heads/") {
        if let Some((the_rest, _g_hash)) = long_name.rsplit_once("-")
            && let Some((tag_, number_string)) = the_rest.rsplit_once("-")
        {
            let tag_or_version = tag_.to_string();
            let number_of_refs_since_commit = number_string.parse::<u32>().map_err(|err| {
                error!(?err);
                io::Error::other(err)
            })?;
            if number_of_refs_since_commit == 0 || the_rest.trim().is_empty() {
                local_repository.tag_delete(&tag_or_version).map_err(|err| {
                    error!(?err);
                    io::Error::other(err)
                })?;
                let new_describe_string = describe_revision(to_this_commit.as_object())?;
                if let Some((the_rest, _g_hash)) = new_describe_string.rsplit_once("-") {
                    if let Some((_the_rest, number_string)) = the_rest.rsplit_once("-") {
                        let number_of_refs_since_commit =
                            number_string.parse::<u32>().map_err(|err| {
                                error!(?err);
                                io::Error::other(err)
                            })?;
                        return Ok(number_of_refs_since_commit);
                    } else {
                        return count_commit_history_since_ref(to_this_commit, local_repository);
                    }
                } else {
                    return count_commit_history_since_ref(to_this_commit, local_repository);
                }
            }
            return Ok(number_of_refs_since_commit);
        }
    } else if let Some((the_rest, _g_hash)) = from_this_describe_string.rsplit_once("-") {
        if let Some((tag_, number_string)) = the_rest.rsplit_once("-") {
            let tag_or_version = tag_.to_string();
            let number_of_refs_since_commit = number_string.parse::<u32>().map_err(|err| {
                error!(?err);
                io::Error::other(err)
            })?;
            if number_of_refs_since_commit == 0 || the_rest.trim().is_empty() {
                local_repository.tag_delete(&tag_or_version).map_err(|err| {
                    error!(?err);
                    error!("Error happened here?");
                    io::Error::other(err)
                })?;
                let new_describe_string = describe_revision(to_this_commit.as_object())?;
                if let Some((the_rest, _g_hash)) = new_describe_string.rsplit_once("-") {
                    if let Some((_the_rest, number_string)) = the_rest.rsplit_once("-") {
                        let number_of_refs_since_commit =
                            number_string.parse::<u32>().map_err(|err| {
                                error!(?err);
                                io::Error::other(err)
                            })?;
                        return Ok(number_of_refs_since_commit);
                    } else {
                        return count_commit_history_since_ref(to_this_commit, local_repository);
                    }
                } else {
                    return count_commit_history_since_ref(to_this_commit, local_repository);
                }
            }
            return Ok(number_of_refs_since_commit);
        }
    } else {
        return count_commit_history_since_ref(to_this_commit, local_repository);
    }
    Err(io::Error::other("Cannot process number of commit history."))
}

fn get_commit_from_roast_info<'a>(
    local_repository: &'a Repository,
    roast_info_path: &Path,
) -> io::Result<Commit<'a>> {
    let content = read_to_string(roast_info_path)?;
    if let Some((_head, commit_hash)) = content.trim().split_once(" ") {
        let oid = Oid::from_str(commit_hash).map_err(|err| {
            error!(?err);
            io::Error::other(err)
        })?;

        let commit = local_repository.find_commit(oid).map_err(|err| {
            error!(?err);
            io::Error::other(err)
        })?;

        return Ok(commit);
    }
    Err(io::Error::other("Cannot split string to get info"))
}

fn get_number_of_changes_from_describe_string(describe_string: &str) -> io::Result<(String, u32)> {
    if let Some((_prefix, long_name)) = describe_string.split_once("heads/") {
        if let Some((the_rest, _g_hash)) = long_name.rsplit_once("-")
            && let Some((tag_, number_string)) = the_rest.rsplit_once("-")
        {
            return Ok((
                tag_.to_string(),
                number_string.parse::<u32>().map_err(|err| {
                    error!(?err);
                    io::Error::other(err)
                })?,
            ));
        }
    } else if let Some((the_rest, _g_hash)) = describe_string.rsplit_once("-")
        && let Some((tag_, number_string)) = the_rest.rsplit_once("-")
    {
        return Ok((
            tag_.to_string(),
            number_string.parse::<u32>().map_err(|err| {
                error!(?err);
                io::Error::other(err)
            })?,
        ));
    }
    Err(io::Error::other("Number of changes is 0"))
}

fn changelog_details_generate(
    local_repository: &Repository,
    git_object: &Object,
) -> io::Result<ChangelogDetails> {
    let mut bulk_commit_message = String::new();
    let mut tag_or_version: String = String::new();
    let commit_hash: String;
    let number_of_refs_since_commit: u32;

    let from_this_commit = get_commit(git_object)?;
    let roast_info_path = std::env::current_dir()?.join("roast_scm.info");

    if roast_info_path.is_file() && cfg!(feature = "obs") {
        let to_this_commit = get_commit_from_roast_info(local_repository, &roast_info_path)?;
        number_of_refs_since_commit =
            count_commit_history(local_repository, &from_this_commit, &to_this_commit)?;
        commit_hash = to_this_commit.id().to_string()
    } else {
        let describe_string = describe_revision(git_object)?;
        info!(?describe_string, "Result of `git describe`: ");
        if let Ok((tag_, num)) = get_number_of_changes_from_describe_string(&describe_string) {
            tag_or_version = tag_;
            number_of_refs_since_commit = num;
        } else {
            number_of_refs_since_commit =
                get_number_of_commits_since(local_repository, &describe_string, &from_this_commit)?;
        }
        commit_hash = from_this_commit.id().to_string();
    }

    debug!(?from_this_commit);
    debug!(?number_of_refs_since_commit);
    debug!(?bulk_commit_message);
    debug!(?commit_hash);

    mutate_bulk_commit_message_string(
        &from_this_commit,
        local_repository,
        &mut bulk_commit_message,
        number_of_refs_since_commit,
    )?;

    let describe_string_for_debug = describe_revision(git_object)?;
    debug!(
        ?describe_string_for_debug,
        "Result of `git describe` after processing commit object. This is used to check if a tag \
         was deleted if number of changes are 0: "
    );

    if !&bulk_commit_message.trim().is_empty() {
        info!("✍🏻 You can copy the changelog below:");
        println!("{}", &bulk_commit_message);
    } else {
        warn!("⚠️📋 No changelog generated.");
    }

    let new_describe_string =
        format!("{}-{}-g{}", tag_or_version, number_of_refs_since_commit, commit_hash);

    Ok(ChangelogDetails {
        changelog: bulk_commit_message,
        describe_string: new_describe_string,
        commit_hash,
        tag_or_version,
        offset_since_current_commit: number_of_refs_since_commit,
    })
}

// This is only useful if there are no tags
fn count_commit_history_since_ref(
    processed_git_commit: &Commit,
    local_repository: &Repository,
) -> io::Result<u32> {
    let mut number_of_changes = 0;
    // Perform a revwalk. This means there were no tags! And we only got a hash
    let mut revwalk = local_repository.revwalk().map_err(|err| {
        error!(?err);
        io::Error::other(err)
    })?;
    revwalk.push_head().map_err(|err| {
        error!(?err);
        io::Error::other(err)
    })?;
    revwalk.set_sorting(git2::Sort::TIME | git2::Sort::TOPOLOGICAL).map_err(|err| {
        error!(?err);
        io::Error::other(err)
    })?;
    let mut start_count = false;

    for rev in revwalk {
        let commit = match rev {
            Ok(rev_) => local_repository.find_commit(rev_).map_err(io::Error::other)?,
            Err(err) => return Err(io::Error::other(err)),
        };
        debug!(?rev, ?commit);
        if commit.id() == processed_git_commit.id() {
            start_count = true
        }
        if start_count {
            number_of_changes += 1;
        }
    }
    Ok(number_of_changes)
}

fn count_commit_history(
    local_repository: &Repository,
    from: &Commit,
    to: &Commit,
) -> io::Result<u32> {
    let mut number_of_changes: u32 = 0;
    let mut start_count = false;

    let mut revwalk = local_repository.revwalk().map_err(|err| {
        error!(?err);
        io::Error::other(err)
    })?;

    revwalk.push_head().map_err(|err| {
        error!(?err);
        io::Error::other(err)
    })?;

    revwalk.set_sorting(git2::Sort::TIME | git2::Sort::TOPOLOGICAL | git2::Sort::REVERSE).map_err(
        |err| {
            error!(?err);
            io::Error::other(err)
        },
    )?;

    for rev in revwalk {
        let commit = match rev {
            Ok(rev_) => local_repository.find_commit(rev_).map_err(io::Error::other)?,
            Err(err) => return Err(io::Error::other(err)),
        };
        debug!(?rev, ?commit);
        if commit.id() == from.id() {
            start_count = true
        }
        if start_count {
            number_of_changes += 1;
            if commit.id() == to.id() {
                break;
            }
        }
    }

    Ok(number_of_changes)
}

fn mutate_bulk_commit_message_string(
    processed_git_commit: &Commit,
    local_repository: &Repository,
    bulk_commit_message: &mut String,
    number_of_refs_since_commit: u32,
) -> io::Result<()> {
    // Perform a revwalk. This means there were no tags! And we only got a hash
    let mut revwalk = local_repository.revwalk().map_err(|err| {
        error!(?err);
        io::Error::other(err)
    })?;
    revwalk.push_head().map_err(|err| {
        error!(?err);
        io::Error::other(err)
    })?;
    revwalk.set_sorting(git2::Sort::TIME | git2::Sort::TOPOLOGICAL).map_err(|err| {
        error!(?err);
        io::Error::other(err)
    })?;
    let mut start_count = false;
    let mut count = 0;

    for rev in revwalk {
        let commit = match rev {
            Ok(rev_) => local_repository.find_commit(rev_).map_err(io::Error::other)?,
            Err(err) => return Err(io::Error::other(err)),
        };
        debug!(?rev, ?commit);
        if commit.id() == processed_git_commit.id() {
            start_count = true
        }
        if start_count {
            if count == number_of_refs_since_commit {
                break;
            } else {
                mut_commit_msg(&commit, bulk_commit_message)?;
            }
            count += 1;
        }
    }

    Ok(())
}

fn mut_commit_msg(commit: &Commit, bulk_commit_message: &mut String) -> io::Result<()> {
    let hash = commit.id().to_string();
    debug!("Commit hash: {}", hash);
    let summary = commit.summary().unwrap_or_default();
    if !summary.trim().is_empty() {
        debug!("Commit summary: {}", summary);
        let format_summary = format!("* {}", &summary);
        bulk_commit_message.push_str(&format_summary);
        bulk_commit_message.push('\n');
    }
    Ok(())
}

fn process_basename_from_url(url_string: &str) -> io::Result<String> {
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
    let basename = match path_segments[1].rsplit_once(".git") {
        Some((basename, _)) => basename,
        None => path_segments[1],
    };
    Ok(basename.to_string())
}

fn process_filename_from_url_and_revision(url_string: &str, revision: &str) -> io::Result<String> {
    let basename = process_basename_from_url(url_string)?;
    let filename = if revision.trim().is_empty() {
        basename.to_string()
    } else {
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
) -> io::Result<String> {
    let mut stub_format = String::new();
    if !changelog_details.tag_or_version.trim().is_empty() {
        if let Some(versionrewriteregex) = &roast_scm_args.versionrewriteregex {
            if let Some(versionrewrite_pattern) = &roast_scm_args.versionrewritepattern {
                let versionformat = Regex::new(versionrewriteregex).map_err(|err| {
                    error!(?err);
                    io::Error::other(err)
                })?;
                let after = versionformat
                    .replace_all(&changelog_details.tag_or_version, versionrewrite_pattern);
                stub_format.push_str(&after);
            }
        } else {
            stub_format.push_str(&changelog_details.tag_or_version);
        }
        if changelog_details.offset_since_current_commit > 0 {
            let git_offset = format!("+git{}", changelog_details.offset_since_current_commit);
            stub_format.push_str(&git_offset);
            if !changelog_details.commit_hash.trim().is_empty() {
                let git_hash_section = format!(".g{}", changelog_details.commit_hash);
                stub_format.push_str(&git_hash_section);
            }
        }
    } else {
        let git_offset = format!("0+git{}", changelog_details.offset_since_current_commit);
        stub_format.push_str(&git_offset);
        if !changelog_details.commit_hash.trim().is_empty() {
            let git_hash_section = format!(".g{}", changelog_details.commit_hash);
            stub_format.push_str(&git_hash_section);
        }
    }
    Ok(stub_format)
}

fn generate_changelog_file(
    roast_scm_args: &RoastScmArgs,
    changelog_details: &ChangelogDetails,
    final_revision_format: &str,
) -> io::Result<()> {
    if roast_scm_args.changesgenerate {
        if let Some(changesauthor) = &roast_scm_args.changesauthor {
            let changesauthor = if let Some(changesemail) = &roast_scm_args.changesemail {
                format!("{} <{}>", changesauthor, changesemail)
            } else {
                changesauthor.to_string()
            };
            let changesoutfile = match &roast_scm_args.changesoutfile {
                Some(v) => v,
                None => &std::env::current_dir()
                    .map_err(|err| {
                        error!(?err);
                        io::Error::other(err)
                    })?
                    .join(format!(
                        "{}.changes",
                        process_basename_from_url(
                            roast_scm_args
                                .git_repository_url
                                .as_ref()
                                .ok_or("No URL provided.")
                                .map_err(|err| {
                                    error!(err);
                                    io::Error::new(io::ErrorKind::InvalidInput, err)
                                })?
                        )?
                    )),
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
            if !changelog_details.changelog.trim().is_empty() {
                let changelog_lines = changelog_details.changelog.lines();
                changelog_lines.into_iter().for_each(|line| {
                    let format_with_two_spaces = format!("  {}\n", line);
                    final_changelog_lines.push_str(&format_with_two_spaces);
                });
            } else {
                final_changelog_lines.push_str("  * NO CHANGELOG\n");
            }
            // Add the last newline
            final_changelog_lines.push('\n');

            let changes_string_from_file = match std::fs::File::create_new(changesoutfile) {
                Ok(file) => {
                    debug!(?file);
                    std::fs::read_to_string(changesoutfile)
                }
                Err(err) => {
                    debug!(?err);
                    // If the file exists
                    if changesoutfile.exists() {
                        std::fs::read_to_string(changesoutfile)
                    } else {
                        Err(io::Error::other(err))
                    }
                }
            }?;

            let new_changes_to_append = format!("{}\n{}", update_statement, final_changelog_lines);
            let mut final_changes_string_for_file: String;
            if !changes_string_from_file.contains(&new_changes_to_append) {
                final_changes_string_for_file = format!(
                    "{}\n\n{}{}",
                    changelog_header, new_changes_to_append, changes_string_from_file
                );
                final_changes_string_for_file =
                    final_changes_string_for_file.trim_end().to_string();
                final_changes_string_for_file.push('\n');

                std::fs::write(changesoutfile, &final_changes_string_for_file).inspect(|_| {
                    info!(
                        "🗒️ Successfully generated changelog to `{}`.",
                        &changesoutfile.display()
                    );
                })?
            } else {
                let lines_from_changes_file = changes_string_from_file.lines();
                let new_changes_string_without_header_yet =
                    lines_from_changes_file.skip(2).collect::<Vec<&str>>().join("\n");
                final_changes_string_for_file =
                    format!("{}\n{}", changelog_header, new_changes_string_without_header_yet);
                final_changes_string_for_file =
                    final_changes_string_for_file.trim_end().to_string();
                final_changes_string_for_file.push('\n');

                std::fs::write(changesoutfile, &final_changes_string_for_file).inspect(|_| {
                    info!(
                        "🗒️ Changelog was already appended before. Not updating the changelog \
                         lines located at `{}`.",
                        &changesoutfile.display()
                    );
                    info!("🗒️ The changelog date and author will be updated, however.");
                    info!(
                        "🗒️ Successfully updated the latest changelog header to `{}`.",
                        &changesoutfile.display()
                    );
                })?
            }
        } else {
            return Err(io::Error::other("No changes author provided."));
        }
    }
    Ok(())
}

fn set_version_in_specfile(
    hard_coded_version: &Option<String>,
    from_formatted_revision: &str,
) -> io::Result<()> {
    use std::{fs, io::Write};
    let new_version = if let Some(new_version) = hard_coded_version {
        new_version
    } else {
        &{ from_formatted_revision.to_string() }
    };

    let r = r"(^Version:)(\s+)(\S+)";
    let regex = Regex::new(r).map_err(|err| {
        error!(?err);
        io::Error::other(err)
    })?;
    let cwd = std::env::current_dir().map_err(|err| {
        error!(?err);
        io::Error::other(err)
    })?;
    let read_dir = cwd.read_dir()?;

    // NOTE: The specfile name should be the same as the name of the directory
    // without the `.spec` extension. Currently, I don't think this is a good
    // idea but is it?
    let Some(basename) = cwd.file_name() else {
        return Err(io::Error::other("There is no basename found!"));
    };

    debug!(?basename);

    let Some(specfile) = read_dir.flatten().find(|f| {
        let the_filename = format!("{}.spec", basename.to_string_lossy());
        let binding = f.path();
        let file_basename = binding.file_name().unwrap_or_default();
        let filename = file_basename.to_string_lossy();

        debug!(?filename, ?the_filename);
        *filename == the_filename
    }) else {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Specfile not found. Might have a different name?",
        ));
    };

    let specfile_path = specfile.path();

    let mut specfile_temporary = tempfile::NamedTempFile::new()?;

    let specfile_read = std::fs::read_to_string(&specfile_path)?;

    let mut specfile_lines = specfile_read.lines();

    let mut update_only_once = false;

    specfile_lines.try_for_each(|line| -> io::Result<()> {
        if regex.is_match(line) {
            if !update_only_once {
                update_only_once = regex.is_match(line);
                debug!(?line, "Match found:");
                let mut after = regex.replace_all(line, r"$1$2").to_string();
                after.push_str(new_version);
                writeln!(specfile_temporary, "{}", after)?;
            }
        } else {
            writeln!(specfile_temporary, "{}", line)?;
        }
        Ok(())
    })?;

    fs::copy(specfile_temporary, specfile_path).inspect(|bytes| {
        info!("✍️ Updated version in specfile.");
        debug!("💽 Total bytes written: {}", bytes);
    })?;

    Ok(())
}

fn produce_roast_info(outdir: &Path, commit_hash: &str) -> io::Result<()> {
    let output_file = outdir.join("roast_scm.info");
    let content = format!(r#"commit: {}"#, commit_hash);
    if output_file.is_file() {
        warn!("Overwriting {}", output_file.display());
    }
    std::fs::write(output_file, content).inspect_err(|err| {
        error!(?err);
    })?;
    Ok(())
}

/// Creates a tarball from a given URL. URL must be a valid remote git
/// repository.
///
/// It uses `crate::operations::roast` under the hood. Locally cloned
/// repositories can be not deleted if the `crate::cli::RoastScmArgs` has its
/// field `is_temporary` set to `false`.
pub fn roast_scm_opts(
    custom_workdir: Option<PathBuf>,
    roast_scm_args: &RoastScmArgs,
    start_trace: bool,
) -> io::Result<Option<std::path::PathBuf>> {
    if let Some(ref subcommand) = roast_scm_args.subcommands {
        let mut cmd = RoastScmArgs::command();
        match subcommand {
            crate::operations::cli::Subcommands::GenerateCompletionsFor { shell } => {
                print_completions(*shell, &mut cmd);
            }
        }
        Ok(None)
    } else {
        #[allow(clippy::if_same_then_else)] // It's not actually the same
        if cfg!(feature = "obs") {
            if start_trace {
                start_tracing();
            }
        } else if !roast_scm_args.silent && start_trace {
            start_tracing();
        }
        info!("⛓️🔥 Starting Roast SCM!");
        debug!(?roast_scm_args);
        let workdir = if let Some(workdir) = custom_workdir {
            workdir
        } else {
            let tmp_workdir = tempfile::TempDir::new()?;
            if !roast_scm_args.is_temporary {
                tmp_workdir.keep()
            } else {
                tmp_workdir.path().to_owned()
            }
        };

        let revision = &roast_scm_args
            .revision
            .as_ref()
            .ok_or("No revision provided.")
            .map_err(|err| {
                error!(err);
                io::Error::new(io::ErrorKind::InvalidInput, err)
            })?
            .to_string();

        let git_url = &roast_scm_args
            .git_repository_url
            .as_ref()
            .ok_or("No URL provided.")
            .map_err(|err| {
                error!(err);
                io::Error::new(io::ErrorKind::InvalidInput, err)
            })?
            .to_string();

        info!(?git_url, "🫂 Cloning remote repository.");
        info!(?workdir, "🏃 Cloning to local directory...");

        let changelog_details = git_clone2(git_url, &workdir, revision, roast_scm_args.depth)?;

        let final_revision_format =
            rewrite_version_or_revision_from_changelog_details(&changelog_details, roast_scm_args)?;

        let filename_prefix = if let Some(set_name) = &roast_scm_args.set_name {
            if let Some(set_version) = &roast_scm_args.set_version {
                format!("{}-{}", set_name, set_version)
            } else {
                format!("{}-{}", set_name, &final_revision_format)
            }
        } else if let Some(set_version) = &roast_scm_args.set_version {
            process_filename_from_url_and_revision(git_url, set_version)?
        } else {
            process_filename_from_url_and_revision(git_url, &final_revision_format)?
        };

        let new_workdir_for_copy = tempfile::TempDir::new()?;
        let local_copy_dir = new_workdir_for_copy.path().join(&filename_prefix);

        copy_dir_all(&workdir, &local_copy_dir)?;

        let outfile = match roast_scm_args.outfile.clone() {
            Some(outfile) => outfile,
            None => {
                let extension = &roast_scm_args.compression.to_extension();
                let full_filename = format!("{}{}", filename_prefix, extension);
                Path::new(&full_filename).to_path_buf()
            }
        };
        info!(?git_url, "🫂 Finished cloning remote repository.");
        info!("🍄 Cloned to `{}`.", &workdir.display());

        let roast_args = RoastArgs {
            target: Some(local_copy_dir),
            include: None,
            exclude: roast_scm_args.exclude.clone(),
            additional_paths: None,
            outfile: Some(outfile),
            outdir: roast_scm_args.outdir.clone(),
            preserve_root: true,
            reproducible: roast_scm_args.reproducible,
            ignore_git: roast_scm_args.ignore_git,
            ignore_hidden: roast_scm_args.ignore_hidden,
            silent: roast_scm_args.silent,
            subcommands: None,
        };

        roast_opts(&roast_args, false)
            .map(|_| {
                generate_changelog_file(
                    roast_scm_args,
                    &changelog_details,
                    &final_revision_format,
                )?;

                if cfg!(feature = "obs") {
                    set_version_in_specfile(&roast_scm_args.set_version, &final_revision_format)?;
                    produce_roast_info(
                        &roast_scm_args.outdir.clone().unwrap_or(std::env::current_dir()?),
                        &changelog_details.commit_hash,
                    )?;
                }

                if !roast_scm_args.is_temporary {
                    info!(
                        "👁️ Locally cloned repository is not deleted and located at `{}`.",
                        workdir.display()
                    );
                    Ok(Some(workdir.to_path_buf()))
                } else {
                    Ok(None)
                }
            })
            .inspect_err(|err| {
                error!(?err);
            })?
            .inspect(|_| info!("⛓️🔥 Finished Roast SCM!"))
    }
}
