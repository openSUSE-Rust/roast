use git2::Repository;
use std::path::Path;

pub fn clone(
    repo_url: &str,
    local_clone_path: &Path,
    recursive_submodules: bool,
) -> Result<Repository, git2::Error>
{
    let result = if recursive_submodules
    {
        Repository::clone_recurse(repo_url, local_clone_path)
    }
    else
    {
        Repository::clone(repo_url, local_clone_path)
    };
    result
}
