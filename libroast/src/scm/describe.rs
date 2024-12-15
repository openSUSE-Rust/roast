use git2::{
    DescribeFormatOptions,
    DescribeOptions,
    Repository,
};
use std::path::Path;

pub fn describe(local_repo_path: &Path) -> Result<String, git2::Error>
{
    let repository = Repository::open(local_repo_path)?;
    let mut describe_options = DescribeOptions::new();
    let mut describe_format_options = DescribeFormatOptions::new();
    describe_options.describe_all();
    describe_format_options.always_use_long_format(true);
    let describe_result = repository.describe(&describe_options)?;
    let ret = describe_result.format(Some(&describe_format_options));
    ret
}
