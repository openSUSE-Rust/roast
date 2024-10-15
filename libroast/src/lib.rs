// SPDX-License-Identifier: MPL-2.0

// Copyright (C) 2024 Soc Virnyl Estela and contributors

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

pub mod common;
pub mod compress;
pub mod consts;
pub mod decompress;

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
use std::path::Path;
#[allow(unused_imports)]
use tracing::{
    debug,
    error,
    info,
    warn,
};

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
