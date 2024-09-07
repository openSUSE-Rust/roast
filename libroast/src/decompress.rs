// SPDX-License-Identifier: MPL-2.0

// Copyright (C) 2024 Soc Virnyl Estela and contributors

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::fs;
use std::io;
use std::io::Seek;
use std::path::Path;
use tar;

#[allow(unused_imports)]
use tracing::{debug, error, info, warn};

pub fn targz(outdir: impl AsRef<Path>, srcpath: impl AsRef<Path>) -> io::Result<()> {
    use flate2::bufread::GzDecoder;
    let mut src = io::BufReader::new(fs::File::open(srcpath.as_ref())?);
    src.seek(io::SeekFrom::Start(0))?;
    let enc = GzDecoder::new(src);
    let mut ar = tar::Archive::new(enc);
    Ok({
        ar.unpack(outdir.as_ref())?;
        debug!(
            "Successfully decompressed Gz archive from {} to {}",
            srcpath.as_ref().to_string_lossy(),
            outdir.as_ref().to_string_lossy(),
        );
    })
}

pub fn tarzst(outdir: impl AsRef<Path>, srcpath: impl AsRef<Path>) -> io::Result<()> {
    use zstd::Decoder;
    let mut src = io::BufReader::new(fs::File::open(srcpath.as_ref())?);
    src.seek(io::SeekFrom::Start(0))?;
    let enc = Decoder::new(src)?;
    let mut ar = tar::Archive::new(enc);
    Ok({
        ar.unpack(outdir.as_ref())?;
        debug!(
            "Successfully decompressed Zst archive from {} to {}",
            srcpath.as_ref().to_string_lossy(),
            outdir.as_ref().to_string_lossy(),
        );
    })
}

pub fn tarxz(outdir: impl AsRef<Path>, srcpath: impl AsRef<Path>) -> io::Result<()> {
    use xz2::read::XzDecoder;
    let mut src = io::BufReader::new(fs::File::open(srcpath.as_ref())?);
    src.seek(io::SeekFrom::Start(0))?;
    let enc = XzDecoder::new(src);
    let mut ar = tar::Archive::new(enc);
    Ok({
        ar.unpack(outdir.as_ref())?;
        debug!(
            "Successfully decompressed Xz archive from {} to {}",
            srcpath.as_ref().to_string_lossy(),
            outdir.as_ref().to_string_lossy(),
        );
    })
}

pub fn tarbz2(outdir: impl AsRef<Path>, srcpath: impl AsRef<Path>) -> io::Result<()> {
    use bzip2::bufread::MultiBzDecoder;

    let mut src = io::BufReader::new(fs::File::open(srcpath.as_ref())?);
    src.seek(io::SeekFrom::Start(0))?;
    let enc = MultiBzDecoder::new(src);
    let mut ar = tar::Archive::new(enc);
    Ok({
        ar.unpack(outdir.as_ref())?;
        debug!(
            "Successfully decompressed Bz2 archive from {} to {}",
            srcpath.as_ref().to_string_lossy(),
            outdir.as_ref().to_string_lossy(),
        );
    })
}
