// SPDX-License-Identifier: MPL-2.0

// Copyright (C) 2025 Soc Virnyl Estela and contributors

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Decompress and extract core utility functions.

use std::{
    fs,
    io,
    io::Seek,
    path::Path,
};
use tar;
#[allow(unused_imports)]
use tracing::{
    debug,
    error,
    info,
    warn,
};

/// Decompresses and extracts an archive with Gz.
pub fn targz(outdir: impl AsRef<Path>, srcpath: impl AsRef<Path>) -> io::Result<()>
{
    use flate2::bufread::GzDecoder;
    let mut src = io::BufReader::new(fs::File::open(srcpath.as_ref())?);
    src.seek(io::SeekFrom::Start(0))?;
    let enc = GzDecoder::new(src);
    let mut ar = tar::Archive::new(enc);
    ar.unpack(outdir.as_ref())?;
    debug!(
        "Successfully decompressed and extracted tape gz-compressed archive from {} to {}",
        srcpath.as_ref().to_string_lossy(),
        outdir.as_ref().to_string_lossy(),
    );
    Ok(())
}

/// Decompresses and extracts an archive with Zstd.
pub fn tarzst(outdir: impl AsRef<Path>, srcpath: impl AsRef<Path>) -> io::Result<()>
{
    use zstd::Decoder;
    let mut src = io::BufReader::new(fs::File::open(srcpath.as_ref())?);
    src.seek(io::SeekFrom::Start(0))?;
    let enc = Decoder::new(src)?;
    let mut ar = tar::Archive::new(enc);
    ar.unpack(outdir.as_ref())?;
    debug!(
        "Successfully decompressed and extracted tape zstd-compressed archive from {} to {}",
        srcpath.as_ref().to_string_lossy(),
        outdir.as_ref().to_string_lossy(),
    );
    Ok(())
}

/// Decompresses and extracts an archive with Xz/Lzma.
pub fn tarxz(outdir: impl AsRef<Path>, srcpath: impl AsRef<Path>) -> io::Result<()>
{
    use xz2::read::XzDecoder;
    let mut src = io::BufReader::new(fs::File::open(srcpath.as_ref())?);
    src.seek(io::SeekFrom::Start(0))?;
    let enc = XzDecoder::new(src);
    let mut ar = tar::Archive::new(enc);
    ar.unpack(outdir.as_ref())?;
    debug!(
        "Successfully decompressed and extracted tape xz-compressed archive from {} to {}",
        srcpath.as_ref().to_string_lossy(),
        outdir.as_ref().to_string_lossy(),
    );
    Ok(())
}

/// Decompresses and extracts an archive with Bz2.
pub fn tarbz2(outdir: impl AsRef<Path>, srcpath: impl AsRef<Path>) -> io::Result<()>
{
    use bzip2::bufread::MultiBzDecoder;

    let mut src = io::BufReader::new(fs::File::open(srcpath.as_ref())?);
    src.seek(io::SeekFrom::Start(0))?;
    let enc = MultiBzDecoder::new(src);
    let mut ar = tar::Archive::new(enc);
    ar.unpack(outdir.as_ref())?;
    debug!(
        "Successfully decompressed and extracted tape bz2-compressed archive from {} to {}",
        srcpath.as_ref().to_string_lossy(),
        outdir.as_ref().to_string_lossy(),
    );
    Ok(())
}

/// Extracts an uncompressed archive.
pub fn vanilla(outdir: impl AsRef<Path>, srcpath: impl AsRef<Path>) -> io::Result<()>
{
    let mut src = io::BufReader::new(fs::File::open(srcpath.as_ref())?);
    src.seek(io::SeekFrom::Start(0))?;
    let mut ar = tar::Archive::new(src);
    ar.unpack(outdir.as_ref())?;
    debug!(
        "Successfully extracted tape archive from {} to {}",
        srcpath.as_ref().to_string_lossy(),
        outdir.as_ref().to_string_lossy(),
    );
    Ok(())
}
