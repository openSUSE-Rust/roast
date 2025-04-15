// SPDX-License-Identifier: MPL-2.0

// Copyright (C) 2024 Soc Virnyl Estela and contributors

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
use std::{
    fs,
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

/// Create a deterministic tar-header for creating reproducible tarballs
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

fn add_path_to_archive<T: Write>(
    builder: &mut tar::Builder<T>,
    additional_path: &Path,
    target_dir: &Path,
    reproducible: bool,
) -> io::Result<()>
{
    let mut h = if reproducible
    {
        create_deterministic_header(additional_path)?
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
    let subpath = additional_path.strip_prefix(target_dir).map_err(|err| {
        error!(
            ?err,
            "THIS IS A BUG. Unable to proceed. {} is not within {}.",
            additional_path.to_string_lossy(),
            target_dir.to_string_lossy()
        );
        io::Error::new(io::ErrorKind::Other, additional_path.to_string_lossy())
    })?;

    if additional_path.is_file()
    {
        let src = std::fs::File::open(additional_path).map(std::io::BufReader::new)?;
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

pub fn tar_builder<T: Write>(
    builder: &mut tar::Builder<T>,
    target_dir: impl AsRef<Path>,
    archive_files: &[impl AsRef<Path>],
    reproducible: bool,
) -> io::Result<()>
{
    // Only metadata that is directly relevant to the identity of a file will be
    // included. In particular, ownership and mod/access times are excluded.
    builder.mode(tar::HeaderMode::Deterministic);
    let mut archive_files: Vec<PathBuf> =
        archive_files.iter().map(|p| p.as_ref().to_path_buf()).collect();
    archive_files.sort();
    archive_files.iter().try_for_each(|f| {
        let f = PathBuf::from(f);
        debug!(?f);
        if f.exists()
        {
            // Using walkdir for deterministic ordering of the files
            for entry in walkdir::WalkDir::new(f).sort_by_file_name()
            {
                let entry = entry?;
                add_path_to_archive(builder, entry.path(), target_dir.as_ref(), reproducible)?;
            }
            Ok(())
        }
        else
        {
            error!("THIS IS A BUG. Unable to proceed. {} does not exist.", f.to_string_lossy());
            Err(io::Error::new(io::ErrorKind::Other, f.to_string_lossy()))
        }
    })?;

    builder.finish()
}

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
    tar_builder(&mut builder, target_dir, archive_files, reproducible)
}

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
    tar_builder(&mut builder, target_dir, archive_files, reproducible)
}

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
    tar_builder(&mut builder, target_dir, archive_files, reproducible)
}

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
    tar_builder(&mut builder, target_dir, archive_files, reproducible)
}

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
    tar_builder(&mut builder, target_dir, archive_files, reproducible)
}
