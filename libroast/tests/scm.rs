use libroast::{
    self,
    operations::cli::RoastScmArgs,
};
use sha3::{
    Digest,
    Keccak256,
};
use std::{
    fs::{
        read,
        read_dir,
    },
    io,
    path::PathBuf,
};
use test_log::test;
#[allow(unused_imports)]
use tracing::{
    Level,
    debug,
    error,
    info,
    trace,
    warn,
};

#[test]
fn different_revisions_pointing_to_the_same_commit_produce_the_same_files() -> io::Result<()>
{
    let tmp_binding = tempfile::TempDir::new().map_err(|err| {
        error!(?err, "Failed to create temporary directory");
        err
    })?;
    let mut hasher1 = Keccak256::new();
    let mut hasher2 = Keccak256::new();
    let outdir = tmp_binding.path();
    let r1 = RoastScmArgs {
        subcommands: None,
        set_version: None,
        set_name: None,
        changesgenerate: false,
        changesauthor: None,
        changesemail: None,
        changesoutfile: None,
        git_repository_url: Some("https://github.com/openSUSE-Rust/obs-service-cargo".to_string()),
        exclude: None,
        revision: Some("v5.1.0".to_string()),
        versionrewriteregex: None,
        versionrewritepattern: None,
        depth: 0,
        is_temporary: true,
        outfile: Some(PathBuf::new().join("r1.tar.zst")),
        outdir: Some(outdir.to_path_buf()),
        reproducible: true,
        ignore_git: true,
        ignore_hidden: false,
        silent: false,
        compression: libroast::common::Compression::default(),
    };
    libroast::operations::roast_scm::roast_scm_opts(None, &r1, false)?;
    let buf1 = read(outdir.join("r1.tar.zst"))?;
    hasher1.update(buf1);
    let r2 = RoastScmArgs {
        silent: false,
        subcommands: None,
        set_version: None,
        set_name: None,
        changesgenerate: false,
        changesauthor: None,
        changesemail: None,
        changesoutfile: None,
        git_repository_url: Some("https://github.com/openSUSE-Rust/obs-service-cargo".to_string()),
        exclude: None,
        revision: Some("2910335f66158a658dfeebca8b8bf6cfc09ba1c0".to_string()),
        versionrewriteregex: None,
        versionrewritepattern: None,
        depth: 0,
        is_temporary: true,
        outfile: Some(PathBuf::new().join("r2.tar.zst")),
        outdir: Some(outdir.to_path_buf()),
        reproducible: true,
        ignore_git: true,
        ignore_hidden: false,
        compression: libroast::common::Compression::default(),
    };
    libroast::operations::roast_scm::roast_scm_opts(None, &r2, false)?;
    let buf2 = read(outdir.join("r2.tar.zst"))?;
    hasher2.update(buf2);
    assert_eq!(hasher1.finalize(), hasher2.finalize());
    Ok(())
}

#[test]
fn different_revisions_pointing_to_the_same_commit_produce_the_same_filenames() -> io::Result<()>
{
    let tmp_binding1 = tempfile::TempDir::new().map_err(|err| {
        error!(?err, "Failed to create temporary directory");
        err
    })?;
    let tmp_binding2 = tempfile::TempDir::new().map_err(|err| {
        error!(?err, "Failed to create temporary directory");
        err
    })?;
    let mut hasher1 = Keccak256::new();
    let mut hasher2 = Keccak256::new();
    let outdir1 = tmp_binding1.path();
    let outdir2 = tmp_binding2.path();
    let r1 = RoastScmArgs {
        silent: false,
        subcommands: None,
        set_version: None,
        set_name: None,
        changesgenerate: false,
        changesauthor: None,
        changesemail: None,
        changesoutfile: None,
        git_repository_url: Some("https://github.com/openSUSE-Rust/obs-service-cargo".to_string()),
        exclude: None,
        revision: Some("v5.1.0".to_string()),
        versionrewriteregex: None,
        versionrewritepattern: None,
        depth: 0,
        is_temporary: true,
        outfile: None,
        outdir: Some(outdir1.to_path_buf()),
        reproducible: true,
        ignore_git: true,
        ignore_hidden: false,
        compression: libroast::common::Compression::default(),
    };
    libroast::operations::roast_scm::roast_scm_opts(None, &r1, false)?;
    let r2 = RoastScmArgs {
        silent: false,
        subcommands: None,
        set_version: None,
        set_name: None,
        changesgenerate: false,
        changesauthor: None,
        changesemail: None,
        changesoutfile: None,
        git_repository_url: Some("https://github.com/openSUSE-Rust/obs-service-cargo".to_string()),
        exclude: None,
        revision: Some("2910335f66158a658dfeebca8b8bf6cfc09ba1c0".to_string()),
        versionrewriteregex: None,
        versionrewritepattern: None,
        depth: 0,
        is_temporary: true,
        outfile: None,
        outdir: Some(outdir2.to_path_buf()),
        reproducible: true,
        ignore_git: true,
        ignore_hidden: false,
        compression: libroast::common::Compression::default(),
    };
    libroast::operations::roast_scm::roast_scm_opts(None, &r2, false)?;
    let read_dir1 = read_dir(outdir1)?;
    let read_dir2 = read_dir(outdir2)?;
    let Some(file1) = read_dir1.flatten().find(|entry| entry.path().is_file())
    else
    {
        return Err(io::Error::new(io::ErrorKind::NotFound, "file1 not found."));
    };
    let Some(file2) = read_dir2.flatten().find(|entry| entry.path().is_file())
    else
    {
        return Err(io::Error::new(io::ErrorKind::NotFound, "file2 not found."));
    };
    assert_eq!(file1.file_name(), file2.file_name());
    let hash1 = read(file1.path())?;
    let hash2 = read(file2.path())?;
    hasher1.update(hash1);
    hasher2.update(hash2);
    assert_eq!(hasher1.finalize(), hasher2.finalize());
    Ok(())
}

#[test]
fn repo_with_submodules_1() -> io::Result<()>
{
    let tmp_binding1 = tempfile::TempDir::new().map_err(|err| {
        error!(?err, "Failed to create temporary directory");
        err
    })?;
    let tmp_binding2 = tempfile::TempDir::new().map_err(|err| {
        error!(?err, "Failed to create temporary directory");
        err
    })?;
    let mut hasher1 = Keccak256::new();
    let mut hasher2 = Keccak256::new();
    let outdir1 = tmp_binding1.path();
    let outdir2 = tmp_binding2.path();
    let r1 = RoastScmArgs {
        silent: false,
        subcommands: None,
        set_version: None,
        set_name: None,
        changesgenerate: false,
        changesauthor: None,
        changesemail: None,
        changesoutfile: None,
        git_repository_url: Some("https://codeberg.org/river/river".to_string()),
        exclude: None,
        revision: Some("v0.2.2".to_string()),
        versionrewriteregex: None,
        versionrewritepattern: None,
        depth: 0,
        is_temporary: true,
        outfile: None,
        outdir: Some(outdir1.to_path_buf()),
        reproducible: true,
        ignore_git: true,
        ignore_hidden: false,
        compression: libroast::common::Compression::default(),
    };
    libroast::operations::roast_scm::roast_scm_opts(None, &r1, false)?;
    let r2 = RoastScmArgs {
        silent: false,
        subcommands: None,
        set_version: None,
        set_name: None,
        changesgenerate: false,
        changesauthor: None,
        changesemail: None,
        changesoutfile: None,
        git_repository_url: Some("https://codeberg.org/river/river".to_string()),
        exclude: None,
        revision: Some("v0.2.2".to_string()),
        versionrewriteregex: None,
        versionrewritepattern: None,
        depth: 0,
        is_temporary: true,
        outfile: None,
        outdir: Some(outdir2.to_path_buf()),
        reproducible: true,
        ignore_git: true,
        ignore_hidden: false,
        compression: libroast::common::Compression::default(),
    };
    libroast::operations::roast_scm::roast_scm_opts(None, &r2, false)?;
    let read_dir1 = read_dir(outdir1)?;
    let read_dir2 = read_dir(outdir2)?;
    let Some(file1) = read_dir1.flatten().find(|entry| entry.path().is_file())
    else
    {
        return Err(io::Error::new(io::ErrorKind::NotFound, "file1 not found."));
    };
    let Some(file2) = read_dir2.flatten().find(|entry| entry.path().is_file())
    else
    {
        return Err(io::Error::new(io::ErrorKind::NotFound, "file2 not found."));
    };
    assert_eq!(file1.file_name(), file2.file_name());
    let hash1 = read(file1.path())?;
    let hash2 = read(file2.path())?;
    hasher1.update(hash1);
    hasher2.update(hash2);
    assert_eq!(hasher1.finalize(), hasher2.finalize());
    Ok(())
}

#[test]
fn repo_with_submodules_2() -> io::Result<()>
{
    let tmp_binding1 = tempfile::TempDir::new().map_err(|err| {
        error!(?err, "Failed to create temporary directory");
        err
    })?;
    let tmp_binding2 = tempfile::TempDir::new().map_err(|err| {
        error!(?err, "Failed to create temporary directory");
        err
    })?;
    let mut hasher1 = Keccak256::new();
    let mut hasher2 = Keccak256::new();
    let outdir1 = tmp_binding1.path();
    let outdir2 = tmp_binding2.path();
    let r1 = RoastScmArgs {
        silent: false,
        subcommands: None,
        set_version: None,
        set_name: None,
        changesgenerate: false,
        changesauthor: None,
        changesemail: None,
        changesoutfile: None,
        git_repository_url: Some("https://github.com/mahkoh/jay".to_string()),
        exclude: None,
        revision: Some("master".to_string()),
        versionrewriteregex: None,
        versionrewritepattern: None,
        depth: 0,
        is_temporary: true,
        outfile: None,
        outdir: Some(outdir1.to_path_buf()),
        reproducible: true,
        ignore_git: true,
        ignore_hidden: false,
        compression: libroast::common::Compression::default(),
    };
    libroast::operations::roast_scm::roast_scm_opts(None, &r1, false)?;
    let r2 = RoastScmArgs {
        silent: false,
        subcommands: None,
        set_version: None,
        set_name: None,
        changesgenerate: false,
        changesauthor: None,
        changesemail: None,
        changesoutfile: None,
        git_repository_url: Some("https://github.com/mahkoh/jay".to_string()),
        exclude: None,
        revision: Some("master".to_string()),
        versionrewriteregex: None,
        versionrewritepattern: None,
        depth: 0,
        is_temporary: true,
        outfile: None,
        outdir: Some(outdir2.to_path_buf()),
        reproducible: true,
        ignore_git: true,
        ignore_hidden: false,
        compression: libroast::common::Compression::default(),
    };
    libroast::operations::roast_scm::roast_scm_opts(None, &r2, false)?;
    let read_dir1 = read_dir(outdir1)?;
    let read_dir2 = read_dir(outdir2)?;
    let Some(file1) = read_dir1.flatten().find(|entry| entry.path().is_file())
    else
    {
        return Err(io::Error::new(io::ErrorKind::NotFound, "file1 not found."));
    };
    let Some(file2) = read_dir2.flatten().find(|entry| entry.path().is_file())
    else
    {
        return Err(io::Error::new(io::ErrorKind::NotFound, "file2 not found."));
    };
    assert_eq!(file1.file_name(), file2.file_name());
    let hash1 = read(file1.path())?;
    let hash2 = read(file2.path())?;
    hasher1.update(hash1);
    hasher2.update(hash2);
    assert_eq!(hasher1.finalize(), hasher2.finalize());
    Ok(())
}
