// SPDX-License-Identifier: MPL-2.0

// Copyright (C) 2024 Soc Virnyl Estela and contributors

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

pub mod common;
pub mod compress;
pub mod consts;
pub mod decompress;
pub mod operations;

use crate::{
    common::{
        Compression,
        SupportedFormat,
        UnsupportedFormat,
    },
    consts::{
        BZ2_MIME,
        GZ_MIME,
        SUPPORTED_MIME_TYPES,
        TAR_MIME,
        XZ_MIME,
        ZST_MIME,
    },
};
use std::{
    fs,
    io,
    path::Path,
};
use terminfo::{
    capability as cap,
    Database,
};
#[allow(unused_imports)]
use tracing::{
    debug,
    error,
    info,
    trace,
    warn,
};
use tracing_subscriber::EnvFilter;

pub fn copy_dir_all(src: impl AsRef<Path>, dst: &Path) -> Result<(), io::Error>
{
    debug!("Copying sources");
    debug!(?dst);
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)?
    {
        let entry = entry?;
        let ty = entry.file_type()?;
        trace!(?entry);
        trace!(?ty);
        if ty.is_dir()
        {
            trace!(?ty, "Is directory?");
            copy_dir_all(entry.path(), &dst.join(entry.file_name()))?;

        // Should we respect symlinks?
        // } else if ty.is_symlink() {
        //     debug!("Is symlink");
        //     let path = fs::read_link(&entry.path())?;
        //     let path = fs::canonicalize(&path).unwrap();
        //     debug!(?path);
        //     let pathfilename = path.file_name().unwrap_or(OsStr::new("."));
        //     if path.is_dir() {
        //         copy_dir_all(&path, &dst.join(pathfilename))?;
        //     } else {
        //         fs::copy(&path, &mut dst.join(pathfilename))?;
        //     }

        // Be pedantic or you get symlink error
        }
        else if ty.is_file()
        {
            trace!(?ty, "Is file?");
            fs::copy(entry.path(), dst.join(entry.file_name()))?;
        };
    }
    Ok(())
}

pub fn is_supported_format(src: &Path) -> Result<SupportedFormat, UnsupportedFormat>
{
    if let Ok(identified_src) = infer::get_from_path(src)
    {
        if let Some(known) = identified_src
        {
            debug!(?known);
            if SUPPORTED_MIME_TYPES.contains(&known.mime_type())
            {
                return if known.mime_type().eq(GZ_MIME)
                {
                    Ok(SupportedFormat::Compressed(Compression::Gz, src.to_path_buf()))
                }
                else if known.mime_type().eq(XZ_MIME)
                {
                    Ok(SupportedFormat::Compressed(Compression::Xz, src.to_path_buf()))
                }
                else if known.mime_type().eq(ZST_MIME)
                {
                    Ok(SupportedFormat::Compressed(Compression::Zst, src.to_path_buf()))
                }
                else if known.mime_type().eq(BZ2_MIME)
                {
                    Ok(SupportedFormat::Compressed(Compression::Bz2, src.to_path_buf()))
                }
                else if known.mime_type().eq(TAR_MIME)
                {
                    Ok(SupportedFormat::Compressed(Compression::Not, src.to_path_buf()))
                }
                else
                {
                    error!("Should not be able to reach here!");
                    unreachable!()
                };
            }
        }
        else
        {
            let get_ext = match src.extension()
            {
                Some(ext) => ext.to_string_lossy().to_string(),
                None => "unknown format".to_string(),
            };
            return Err(UnsupportedFormat { ext: get_ext });
        }
    }
    Err(UnsupportedFormat { ext: "unknown format".to_string() })
}

pub fn start_tracing()
{
    let terminfodb = Database::from_env().map_err(|e| {
        error!(err = ?e, "Unable to access terminfo db. This is a bug!");
        io::Error::new(
            io::ErrorKind::Other,
            "Unable to access terminfo db. This is a bug! Setting color option to false!",
        )
    });

    let is_termcolorsupported = match terminfodb
    {
        Ok(hasterminfodb) => hasterminfodb.get::<cap::MaxColors>().is_some(),
        Err(_) => false,
    };
    let filter_layer = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let builder = tracing_subscriber::fmt()
        .with_level(true)
        .with_ansi(is_termcolorsupported)
        .with_env_filter(filter_layer)
        .with_level(true);

    let builder = if cfg!(debug_assertions)
    {
        builder.with_file(true).with_line_number(true)
    }
    else
    {
        builder
    };

    builder.init();
}
