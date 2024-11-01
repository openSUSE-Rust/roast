use std::{
    io,
    path::{
        Path,
        PathBuf,
    },
};
#[allow(unused_imports)]
use tracing::{
    debug,
    error,
    info,
    trace,
    warn,
    Level,
};
use tracing_test::traced_test;
use walkdir::WalkDir;

const MANIFEST_DIR: &str = std::env!("CARGO_MANIFEST_DIR", "No such manifest dir");

#[traced_test]
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
    let updated_paths: Vec<PathBuf> = WalkDir::new(workdir)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .map(|f| {
            debug!(?f);
            PathBuf::from(f.path())
        })
        .filter(|p| {
            p.canonicalize().unwrap_or(p.to_path_buf())
                != workdir.canonicalize().unwrap_or(workdir.to_path_buf())
        })
        .filter(|p| p.is_file())
        .collect();
    let out = Path::new("/tmp/ballsofDeezNuts");
    libroast::compress::targz(out, workdir, &updated_paths, true)?;
    let res = libroast::utils::is_supported_format(out).inspect_err(|err| error!(?err));
    info!(?res);
    assert!(res.is_ok());
    Ok(())
}

#[traced_test]
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
    let updated_paths: Vec<PathBuf> = WalkDir::new(workdir)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .map(|f| {
            debug!(?f);
            PathBuf::from(f.path())
        })
        .filter(|p| {
            p.canonicalize().unwrap_or(p.to_path_buf())
                != workdir.canonicalize().unwrap_or(workdir.to_path_buf())
        })
        .filter(|p| p.is_file())
        .collect();
    let out = Path::new("/tmp/ballsofJiaTan");
    libroast::compress::tarxz(out, workdir, &updated_paths, true)?;
    let res = libroast::utils::is_supported_format(out).inspect_err(|err| error!(?err));
    info!(?res);
    assert!(res.is_ok());
    Ok(())
}

#[traced_test]
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
    let updated_paths: Vec<PathBuf> = WalkDir::new(workdir)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .map(|f| {
            debug!(?f);
            PathBuf::from(f.path())
        })
        .filter(|p| {
            p.canonicalize().unwrap_or(p.to_path_buf())
                != workdir.canonicalize().unwrap_or(workdir.to_path_buf())
        })
        .filter(|p| p.is_file())
        .collect();
    let out = Path::new("/tmp/ballsfacebook");
    libroast::compress::tarzst(out, workdir, &updated_paths, true)?;
    let res = libroast::utils::is_supported_format(out).inspect_err(|err| error!(?err));
    info!(?res);
    assert!(res.is_ok());
    Ok(())
}

#[traced_test]
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
    let updated_paths: Vec<PathBuf> = WalkDir::new(workdir)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .map(|f| {
            debug!(?f);
            PathBuf::from(f.path())
        })
        .filter(|p| {
            p.canonicalize().unwrap_or(p.to_path_buf())
                != workdir.canonicalize().unwrap_or(workdir.to_path_buf())
        })
        .filter(|p| p.is_file())
        .collect();
    let out = Path::new("/tmp/ballswhatsbz");
    libroast::compress::tarbz2(out, workdir, &updated_paths, true)?;
    let res = libroast::utils::is_supported_format(out).inspect_err(|err| error!(?err));
    info!(?res);
    assert!(res.is_ok());
    Ok(())
}

#[traced_test]
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
    let updated_paths: Vec<PathBuf> = WalkDir::new(workdir)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .map(|f| {
            debug!(?f);
            PathBuf::from(f.path())
        })
        .filter(|p| {
            p.canonicalize().unwrap_or(p.to_path_buf())
                != workdir.canonicalize().unwrap_or(workdir.to_path_buf())
        })
        .filter(|p| p.is_file())
        .collect();
    let out = Path::new("/tmp/ballsvanillacreampie");
    libroast::compress::vanilla(out, workdir, &updated_paths, true)?;
    let res = libroast::utils::is_supported_format(out).inspect_err(|err| error!(?err));
    info!(?res);
    assert!(res.is_ok());
    Ok(())
}
