// SPDX-License-Identifier: MPL-2.0

// Copyright (C) 2025 Soc Virnyl Estela and contributors

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
use clap::ValueEnum;
use std::{
    fmt::{
        self,
        Display,
    },
    path::PathBuf,
};
#[allow(unused_imports)]
use tracing::{
    debug,
    error,
    info,
    warn,
};

#[derive(ValueEnum, Default, Debug, Clone, Copy)]
/// Compression options to choose. `Compression::Zst` is the default. `Zst` and
/// `Zstd` are the same.
pub enum Compression
{
    Gz,
    Xz,
    #[default]
    Zst,
    Zstd,
    Bz2,
    Not,
}

impl Compression
{
    pub fn to_extension(&self) -> String
    {
        match self
        {
            Compression::Gz => ".tar.gz",
            Compression::Xz => ".tar.xz",
            Compression::Zst | Compression::Zstd => ".tar.zst",
            Compression::Bz2 => ".tar.bz",
            Compression::Not => ".tar",
        }
        .to_string()
    }
}
impl Display for Compression
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        let msg = match self
        {
            Compression::Gz => "gz",
            Compression::Xz => "xz",
            Compression::Zst | Compression::Zstd => "zst",
            Compression::Bz2 => "bz2",
            Compression::Not => "tar (uncompressed)",
        };
        write!(f, "{}", msg)
    }
}

#[derive(Debug)]
/// Representation of supported formats. Either an archive or a directory.
pub enum SupportedFormat
{
    Compressed(Compression, PathBuf),
    Dir(PathBuf),
}

impl std::error::Error for UnsupportedFormat {}

#[derive(Debug)]
/// Representation of an unsupported file format. Used for printing
/// errors.
pub struct UnsupportedFormat
{
    pub ext: String,
}

impl Display for UnsupportedFormat
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "Unsupported archive format: {}", self.ext)
    }
}
