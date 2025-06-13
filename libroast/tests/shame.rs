use rayon::prelude::*;
use std::{
    io,
    path::{
        Path,
        PathBuf,
    },
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

const MANIFEST_DIR: &str = std::env!("CARGO_MANIFEST_DIR", "No such manifest dir");

fn get_all_files(updated_paths: &mut Vec<PathBuf>, workdir: &Path) -> io::Result<()>
{
    if workdir.is_dir()
    {
        let processed_paths: Vec<PathBuf> = workdir
            .read_dir()?
            .par_bridge()
            .flatten()
            .into_par_iter()
            .map(|f| {
                debug!(?f);
                f.path()
            })
            .filter(|p| {
                p.canonicalize().unwrap_or(p.to_path_buf())
                    != workdir.canonicalize().unwrap_or(workdir.to_path_buf())
            })
            .collect();
        processed_paths.into_iter().try_for_each(|f| -> io::Result<()> {
            if f.is_dir()
            {
                get_all_files(updated_paths, &f.canonicalize().unwrap_or(f.to_path_buf()))?;
            }
            else if f.is_file()
            {
                updated_paths.push(f.canonicalize().unwrap_or(f.to_path_buf()));
            }
            Ok(())
        })?;
    }
    Ok(())
}

#[test]
fn is_gz_tarball() -> io::Result<()>
{
    let src = Path::new(MANIFEST_DIR).join("tests");
    let tmp_binding = tempfile::TempDir::new().map_err(|err| {
        error!(?err, "Failed to create temporary directory");
        err
    })?;
    let workdir = &tmp_binding.path();
    libroast::utils::copy_dir_all(src, workdir)?;
    let mut updated_paths: Vec<PathBuf> = Vec::new();
    get_all_files(&mut updated_paths, workdir)?;
    // let updated_paths: Vec<PathBuf> = WalkDir::new(workdir)
    //     .into_iter()
    //     .filter_map(|entry| entry.ok())
    //     .map(|f| {
    //         debug!(?f);
    //         PathBuf::from(f.path())
    //     })
    //     .filter(|p| {
    //         p.canonicalize().unwrap_or(p.to_path_buf())
    //             != workdir.canonicalize().unwrap_or(workdir.to_path_buf())
    //     })
    //     .filter(|p| p.is_file())
    //     .collect();
    let out = Path::new("/tmp/ballsofDeezNuts");
    libroast::compress::targz(out, workdir, &updated_paths, true)?;
    let res = libroast::utils::is_supported_format(out).inspect_err(|err| error!(?err));
    info!(?res);
    assert!(res.is_ok());
    Ok(())
}

#[test]
fn is_xz_tarball() -> io::Result<()>
{
    let src = Path::new(MANIFEST_DIR).join("tests");
    let tmp_binding = tempfile::TempDir::new().map_err(|err| {
        error!(?err, "Failed to create temporary directory");
        err
    })?;
    let workdir = &tmp_binding.path();
    libroast::utils::copy_dir_all(src, workdir)?;
    let mut updated_paths: Vec<PathBuf> = Vec::new();
    get_all_files(&mut updated_paths, workdir)?;
    let out = Path::new("/tmp/ballsofJiaTan");
    libroast::compress::tarxz(out, workdir, &updated_paths, true)?;
    let res = libroast::utils::is_supported_format(out).inspect_err(|err| error!(?err));
    info!(?res);
    assert!(res.is_ok());
    Ok(())
}

#[test]
fn is_zst_tarball() -> io::Result<()>
{
    let src = Path::new(MANIFEST_DIR).join("tests");
    let tmp_binding = tempfile::TempDir::new().map_err(|err| {
        error!(?err, "Failed to create temporary directory");
        err
    })?;
    let workdir = &tmp_binding.path();
    libroast::utils::copy_dir_all(src, workdir)?;
    let mut updated_paths: Vec<PathBuf> = Vec::new();
    get_all_files(&mut updated_paths, workdir)?;
    let out = Path::new("/tmp/ballsfacebook");
    libroast::compress::tarzst(out, workdir, &updated_paths, true)?;
    let res = libroast::utils::is_supported_format(out).inspect_err(|err| error!(?err));
    info!(?res);
    assert!(res.is_ok());
    Ok(())
}

#[test]
fn is_bz2_tarball() -> io::Result<()>
{
    let src = Path::new(MANIFEST_DIR).join("tests");
    let tmp_binding = tempfile::TempDir::new().map_err(|err| {
        error!(?err, "Failed to create temporary directory");
        err
    })?;
    let workdir = &tmp_binding.path();
    libroast::utils::copy_dir_all(src, workdir)?;
    let mut updated_paths: Vec<PathBuf> = Vec::new();
    get_all_files(&mut updated_paths, workdir)?;
    let out = Path::new("/tmp/ballswhatsbz");
    libroast::compress::tarbz2(out, workdir, &updated_paths, true)?;
    let res = libroast::utils::is_supported_format(out).inspect_err(|err| error!(?err));
    info!(?res);
    assert!(res.is_ok());
    Ok(())
}

#[test]
fn is_vanilla_tarball() -> io::Result<()>
{
    let src = Path::new(MANIFEST_DIR).join("tests");
    let tmp_binding = tempfile::TempDir::new().map_err(|err| {
        error!(?err, "Failed to create temporary directory");
        err
    })?;
    let workdir = &tmp_binding.path();
    libroast::utils::copy_dir_all(src, workdir)?;
    let mut updated_paths: Vec<PathBuf> = Vec::new();
    get_all_files(&mut updated_paths, workdir)?;
    let out = Path::new("/tmp/ballsvanillacreampie");
    libroast::compress::vanilla(out, workdir, &updated_paths, true)?;
    let res = libroast::utils::is_supported_format(out).inspect_err(|err| error!(?err));
    info!(?res);
    assert!(res.is_ok());
    Ok(())
}
