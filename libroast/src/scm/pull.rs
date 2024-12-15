use git2::Repository;
use std::path::Path;

pub fn pull_new_changes(local_repo_path: &Path, remote: Option<&str>) -> Result<(), git2::Error>
{
    let repository = Repository::open(local_repo_path)?;
    let remote = match remote
    {
        Some(rem) => rem,
        None => "origin",
    };
    let remote = repository.find_remote(remote)?;
    Ok(())
}
