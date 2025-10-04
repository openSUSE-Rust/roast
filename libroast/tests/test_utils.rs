use libroast::utils;
use std::{
    env,
    fs::{
        read_dir,
        remove_dir,
    },
    io,
    path::Path,
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
fn test_copy_empty_dir() -> io::Result<()>
{
    let dir = tempfile::tempdir()?;
    assert!(dir.path().exists());

    // The directory will be deleted when `dir` goes out of scope
    let dir_as_path = dir.path();

    for el in read_dir(dir_as_path)?.flatten()
    {
        info!("{}", el.path().display())
    }

    assert!(read_dir(dir_as_path)?.flatten().count() == 0);

    let new_dir = env::temp_dir().join("destination");
    utils::copy_dir_all(dir_as_path, &new_dir)?;
    assert!(new_dir.is_dir());
    remove_dir(new_dir)
}

#[test]
fn test_copy_project_to_another_temp_dir() -> io::Result<()>
{
    const MANIFEST_DIR: &str = std::env!("CARGO_MANIFEST_DIR", "No such manifest dir");
    let dir = tempfile::tempdir()?;
    assert!(dir.path().exists());

    let dir_as_path = dir.path();

    info!("{}", dir_as_path.display());

    let libroast_dir = Path::new(MANIFEST_DIR);

    utils::copy_dir_all(libroast_dir, dir_as_path)?;

    let ls = read_dir(dir_as_path)?.flatten();
    assert!(ls.count() > 0);
    Ok(())
}
