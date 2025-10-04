use libroast::operations::{
    cli::RoastArgs,
    roast::roast_opts,
};
use rayon::prelude::*;
use sha3::{
    Digest,
    Keccak256,
};
use std::{
    env,
    fs::{
        File,
        create_dir_all,
        read,
    },
    io,
    path::{
        Path,
        PathBuf,
    },
};
use tar::Archive;
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

fn generate_gz_tarball(outpath: &Path) -> io::Result<()>
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
    libroast::compress::targz(outpath, workdir, &updated_paths, true)?;
    let res = libroast::utils::is_supported_format(outpath).inspect_err(|err| error!(?err));
    info!(?res);
    assert!(res.is_ok());
    Ok(())
}

fn generate_xz_tarball(outpath: &Path) -> io::Result<()>
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
    libroast::compress::tarxz(outpath, workdir, &updated_paths, true)?;
    let res = libroast::utils::is_supported_format(outpath).inspect_err(|err| error!(?err));
    info!(?res);
    assert!(res.is_ok());
    Ok(())
}

fn generate_zst_tarball(outpath: &Path) -> io::Result<()>
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
    libroast::compress::tarzst(outpath, workdir, &updated_paths, true)?;
    let res = libroast::utils::is_supported_format(outpath).inspect_err(|err| error!(?err));
    info!(?res);
    assert!(res.is_ok());
    Ok(())
}

fn generate_bz2_tarball(outpath: &Path) -> io::Result<()>
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
    libroast::compress::tarbz2(outpath, workdir, &updated_paths, true)?;
    let res = libroast::utils::is_supported_format(outpath).inspect_err(|err| error!(?err));
    info!(?res);
    assert!(res.is_ok());
    Ok(())
}

fn generate_icecream_tarball(outpath: &Path) -> io::Result<()>
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
    libroast::compress::vanilla(outpath, workdir, &updated_paths, true)?;
    let res = libroast::utils::is_supported_format(outpath).inspect_err(|err| error!(?err));
    info!(?res);
    assert!(res.is_ok());
    Ok(())
}

#[test]
fn repro_gz_tarball() -> io::Result<()>
{
    let mut hasher1 = Keccak256::new();
    let mut hasher2 = Keccak256::new();
    let outpath1 = Path::new("/tmp/ballsofDeezNutsRepro1");
    generate_gz_tarball(outpath1)?;
    let outpath2 = Path::new("/tmp/ballsofDeezNutsRepro2");
    generate_gz_tarball(outpath2)?;
    let buf1 = read(outpath1)?;
    hasher1.update(buf1);
    let hash1 = hasher1.finalize();
    let buf2 = read(outpath2)?;
    hasher2.update(buf2);
    let hash2 = hasher2.finalize();
    assert_eq!(hash1, hash2);
    Ok(())
}

#[test]
fn repro_xz_tarball() -> io::Result<()>
{
    let mut hasher1 = Keccak256::new();
    let mut hasher2 = Keccak256::new();
    let outpath1 = Path::new("/tmp/ballsofJiaTanRepro1");
    generate_xz_tarball(outpath1)?;
    let outpath2 = Path::new("/tmp/ballsofJiaTanRepro2");
    generate_xz_tarball(outpath2)?;
    let buf1 = read(outpath1)?;
    hasher1.update(buf1);
    let hash1 = hasher1.finalize();
    let buf2 = read(outpath2)?;
    hasher2.update(buf2);
    let hash2 = hasher2.finalize();
    assert_eq!(hash1, hash2);
    Ok(())
}

#[test]
fn repro_zst_tarball() -> io::Result<()>
{
    let mut hasher1 = Keccak256::new();
    let mut hasher2 = Keccak256::new();
    let outpath1 = Path::new("/tmp/ballsfacebookRepro1");
    generate_zst_tarball(outpath1)?;
    let outpath2 = Path::new("/tmp/ballsfacebookRepro2");
    generate_zst_tarball(outpath2)?;
    let buf1 = read(outpath1)?;
    hasher1.update(buf1);
    let hash1 = hasher1.finalize();
    let buf2 = read(outpath2)?;
    hasher2.update(buf2);
    let hash2 = hasher2.finalize();
    assert_eq!(hash1, hash2);
    Ok(())
}

#[test]
fn repro_bz2_tarball() -> io::Result<()>
{
    let mut hasher1 = Keccak256::new();
    let mut hasher2 = Keccak256::new();
    let outpath1 = Path::new("/tmp/ballswhatsbzRepro1");
    generate_bz2_tarball(outpath1)?;
    let outpath2 = Path::new("/tmp/ballswhatsbzRepro2");
    generate_bz2_tarball(outpath2)?;
    let buf1 = read(outpath1)?;
    hasher1.update(buf1);
    let hash1 = hasher1.finalize();
    let buf2 = read(outpath2)?;
    hasher2.update(buf2);
    let hash2 = hasher2.finalize();
    assert_eq!(hash1, hash2);
    Ok(())
}

#[test]
fn repro_vanilla_tarball() -> io::Result<()>
{
    let mut hasher1 = Keccak256::new();
    let mut hasher2 = Keccak256::new();
    let outpath1 = Path::new("/tmp/ballsvanillacreampie1");
    generate_icecream_tarball(outpath1)?;
    let outpath2 = Path::new("/tmp/ballsvanillacreampie2");
    generate_icecream_tarball(outpath2)?;
    let buf1 = read(outpath1)?;
    hasher1.update(buf1);
    let hash1 = hasher1.finalize();
    let buf2 = read(outpath2)?;
    hasher2.update(buf2);
    let hash2 = hasher2.finalize();
    assert_eq!(hash1, hash2);
    Ok(())
}

#[test]
fn a_tree_of_empty_dirs_is_not_empty() -> io::Result<()>
{
    let tmp = tempfile::tempdir()?;
    let tmp_path = tmp.path();
    let empty_dirs: Vec<PathBuf> = vec![
        tmp_path.join("1/z/e/1/4"),
        tmp_path.join("2/b/a"),
        tmp_path.join("1/a/d/e/f"),
        tmp_path.join("c/e/f/d"),
    ];

    for em in &empty_dirs
    {
        create_dir_all(em)?;
    }

    let outfile = env::temp_dir().join("tree.tar");

    let roast_args = RoastArgs {
        target: Some(tmp_path.to_path_buf()),
        include: None,
        exclude: None,
        additional_paths: None,
        outfile: Some(outfile.to_path_buf()),
        outdir: None,
        preserve_root: false,
        reproducible: true,
        ignore_git: true,
        ignore_hidden: true,
        silent: false,
        subcommands: None,
    };

    roast_opts(&roast_args, false)?;

    let file = File::open(outfile)?;

    let mut a = Archive::new(&file);

    for file in a.entries()?
    {
        let file = file?;
        info!("{:?}", file.header().path());
        info!("{:?}", file.header().size());

        let filepath = file.header().path()?.to_path_buf();
        assert!(empty_dirs.contains(&tmp_path.join(&filepath)))
    }

    Ok(())
}
