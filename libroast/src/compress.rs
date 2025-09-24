// SPDX-License-Identifier: MPL-2.0

// Copyright (C) 2025 Soc Virnyl Estela and contributors

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
use rayon::prelude::*;
use std::{
    fs::{
        self,
    },
    io::{
        self,
        Write,
    },
    path::{
        Path,
        PathBuf,
    },
};
use tar;
#[allow(unused_imports)]
use tracing::{
    debug,
    error,
    info,
    trace,
    warn,
};

/// Create a deterministic tar-header for creating reproducible tarballs. Used
/// for `super::tar_builder()` for the parameter `reproducible` to generate
/// deterministic output.
fn create_deterministic_header(path: impl AsRef<Path>) -> Result<tar::Header, io::Error>
{
    let metadata = path.as_ref().symlink_metadata()?;
    let mut h = tar::Header::new_gnu();
    h.set_metadata_in_mode(&metadata, tar::HeaderMode::Deterministic);
    h.set_mtime(0);
    h.set_uid(0);
    h.set_gid(0);
    h.set_cksum();
    Ok(h)
}

/// Helper function to help add other paths to the archive for
/// `super::tar_builder()`.
fn add_path_to_archive<T: Write>(
    builder: &mut tar::Builder<T>,
    additional_path: &Path,
    target_dir: &Path,
    reproducible: bool,
) -> io::Result<()>
{
    let additional_path = additional_path.canonicalize().unwrap_or(additional_path.to_path_buf());
    let target_dir = target_dir.canonicalize().unwrap_or(target_dir.to_path_buf());
    let mut h = if reproducible
    {
        create_deterministic_header(&additional_path)?
    }
    else
    {
        let metadata = additional_path.symlink_metadata()?;
        let mut hsub = tar::Header::new_gnu();
        hsub.set_metadata(&metadata);
        hsub
    };
    // Each path is relative to prjdir. So we can split the
    // prjdir prefix to get the relative archive path.
    let subpath = &additional_path.strip_prefix(&target_dir).map_err(|err| {
        error!(
            ?err,
            "THIS IS A BUG. Unable to proceed. {} is not within {}.",
            additional_path.to_string_lossy(),
            target_dir.to_string_lossy()
        );
        io::Error::other(additional_path.to_string_lossy())
    })?;

    if additional_path.is_file()
    {
        let src = std::fs::File::open(&additional_path).map(std::io::BufReader::new)?;
        builder.append_data(&mut h, subpath, src)?;
    }
    else if additional_path.is_symlink()
    {
        let target = additional_path.read_link()?;
        builder.append_link(&mut h, subpath, target)?;
    }
    else if additional_path.is_dir()
    {
        // Adding the dir as an empty node
        builder.append_data(&mut h, subpath, std::io::Cursor::new([]))?;
    }
    else
    {
        error!("Ignoring unexpected special file: {:?}", additional_path);
    }
    trace!("Added {} to archive", additional_path.to_string_lossy());
    Ok(())
}

/// Helper function to produce a tarball. `super::targz`, `super::tarbz2`,
/// `super::tarzst`, `super::vanilla` and `super::tarxz` use this function as a
/// "tar builder" since all of them have a trait bound for trait
/// `std::io::Write` and have similar parameters.
pub fn tar_builder<T: Write + std::marker::Send>(
    builder: &mut tar::Builder<T>,
    target_dir: &Path,
    archive_files: &[impl AsRef<Path>],
    reproducible: bool,
) -> io::Result<()>
{
    // Only metadata that is directly relevant to the identity of a file will be
    // included. In particular, ownership and mod/access times are excluded.
    builder.mode(tar::HeaderMode::Deterministic);
    let mut archive_files: Vec<PathBuf> =
        archive_files.iter().map(|p| p.as_ref().to_path_buf()).collect();
    archive_files.par_sort_by(|a, b| a.to_string_lossy().cmp(&b.to_string_lossy()));
    archive_files.iter().try_for_each(|f| {
        let f = &Path::new(f);
        debug!(?f);
        if f.exists()
        {
            add_path_to_archive(builder, f, target_dir, reproducible)
        }
        else
        {
            error!("THIS IS A BUG. Unable to proceed. {} does not exist.", f.to_string_lossy());
            Err(io::Error::other(f.to_string_lossy()))
        }
    })?;

    builder.finish()
}

/// Produces a Gz compressed tarball e.g. `source.tar.gz`.
pub fn targz(
    outpath: impl AsRef<Path>,
    target_dir: impl AsRef<Path>,
    archive_files: &[impl AsRef<Path>],
    reproducible: bool,
) -> io::Result<()>
{
    use flate2::{
        Compression,
        write::GzEncoder,
    };
    let outtar = fs::File::create(outpath.as_ref())
        .inspect_err(|_| error!(outpath = ?outpath.as_ref(), "Unable to create outtar"))?;
    let encoder = GzEncoder::new(outtar, Compression::default());
    let mut builder = tar::Builder::new(encoder);
    tar_builder(&mut builder, target_dir.as_ref(), archive_files, reproducible)
}

/// Produces a Zst compressed tarball e.g. `source.tar.zst` or
/// `source.tar.zstd`.
pub fn tarzst(
    outpath: impl AsRef<Path>,
    target_dir: impl AsRef<Path>,
    archive_files: &[impl AsRef<Path>],
    reproducible: bool,
) -> io::Result<()>
{
    use zstd::Encoder;
    let outtar = fs::File::create(outpath.as_ref())
        .inspect_err(|_| error!(outpath = ?outpath.as_ref(), "Unable to create outtar"))?;
    let mut enc_builder = Encoder::new(outtar, 19)?;
    enc_builder.include_checksum(true)?;
    let threads: u32 = std::thread::available_parallelism()?.get() as u32;
    enc_builder.multithread(threads)?;
    let encoder = enc_builder.auto_finish();
    let mut builder = tar::Builder::new(encoder);
    tar_builder(&mut builder, target_dir.as_ref(), archive_files, reproducible)
}

/// Produces a Xz compressed tarball e.g. `source.tar.xz`.
pub fn tarxz(
    outpath: impl AsRef<Path>,
    target_dir: impl AsRef<Path>,
    archive_files: &[impl AsRef<Path>],
    reproducible: bool,
) -> io::Result<()>
{
    // Crc32 is simpler/faster and often hardware accelerated.
    use xz2::{
        stream::{
            Check::Crc32,
            MtStreamBuilder,
        },
        write::XzEncoder,
    };
    let outtar = fs::File::create(outpath.as_ref())
        .inspect_err(|_| error!(outpath = ?outpath.as_ref(), "Unable to create outtar"))?;
    let threads: u32 = std::thread::available_parallelism()?.get() as u32;
    let enc_builder = MtStreamBuilder::new().preset(6).threads(threads).check(Crc32).encoder()?;
    let encoder = XzEncoder::new_stream(outtar, enc_builder);
    let mut builder = tar::Builder::new(encoder);
    tar_builder(&mut builder, target_dir.as_ref(), archive_files, reproducible)
}

/// Produces a Bz compressed tarball e.g. `source.tar.bz`.
pub fn tarbz2(
    outpath: impl AsRef<Path>,
    target_dir: impl AsRef<Path>,
    archive_files: &[impl AsRef<Path>],
    reproducible: bool,
) -> io::Result<()>
{
    use bzip2::{
        Compression,
        write::BzEncoder,
    };
    let outtar = fs::File::create(outpath.as_ref())
        .inspect_err(|_| error!(outpath = ?outpath.as_ref(), "Unable to create outtar"))?;
    let encoder = BzEncoder::new(outtar, Compression::best());
    let mut builder = tar::Builder::new(encoder);
    tar_builder(&mut builder, target_dir.as_ref(), archive_files, reproducible)
}

/// Produces a uncompressed tarball e.g. `source.tar`.
pub fn vanilla(
    outpath: impl AsRef<Path>,
    target_dir: impl AsRef<Path>,
    archive_files: &[impl AsRef<Path>],
    reproducible: bool,
) -> io::Result<()>
{
    let outtar = fs::File::create(outpath.as_ref())
        .inspect_err(|_| error!(outpath = ?outpath.as_ref(), "Unable to create outtar"))?;
    let mut builder = tar::Builder::new(outtar);
    tar_builder(&mut builder, target_dir.as_ref(), archive_files, reproducible)
}
