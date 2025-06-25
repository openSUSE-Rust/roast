// SPDX-License-Identifier: MPL-2.0

// Copyright (C) 2025 Soc Virnyl Estela and contributors

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::{
    decompress,
    operations::cli::{
        RawArgs,
        print_completions,
    },
    utils::{
        is_supported_format,
        process_globs,
        start_tracing,
    },
};
use clap::CommandFactory;
use std::io;
#[allow(unused_imports)]
use tracing::{
    Level,
    debug,
    error,
    info,
    trace,
    warn,
};

/// Extracts a tarball as long as it is a supported mime-type. Arguments
/// passed are based on `crate::cli::RawArgs`.
pub fn raw_opts(raw_args: RawArgs, start_trace: bool) -> io::Result<()>
{
    if let Some(ref subcommand) = raw_args.subcommands
    {
        let mut cmd = RawArgs::command();
        match subcommand
        {
            crate::operations::cli::Subcommands::GenerateCompletionsFor { shell } =>
            {
                print_completions(*shell, &mut cmd);
            }
        }
        Ok(())
    }
    else
    {
        if cfg!(feature = "obs")
        {
            if start_trace
            {
                start_tracing();
            }
        }
        else if !raw_args.silent && start_trace
        {
            start_tracing();
        }
        info!("🥩 Starting Raw.");
        debug!(?raw_args);

        let target_path =
            process_globs(&raw_args.target.ok_or("No target directory set").map_err(|err| {
                error!(err);
                io::Error::new(io::ErrorKind::InvalidInput, err)
            })?)?;
        let target_path = target_path.canonicalize().unwrap_or(target_path);
        if target_path.is_file()
        {
            match is_supported_format(&target_path)
            {
                Ok(target) => match target
                {
                    crate::common::SupportedFormat::Compressed(mime_type, src) =>
                    {
                        info!(?mime_type);
                        let outpath = raw_args.outdir.unwrap_or(
                            std::env::current_dir().inspect_err(|e| {
                                error!(?e, "Unable to determine current directory!");
                            })?,
                        );
                        match mime_type
                        {
                            crate::common::Compression::Gz =>
                            {
                                decompress::targz(&outpath, &src)?;
                            }
                            crate::common::Compression::Xz =>
                            {
                                decompress::tarxz(&outpath, &src)?;
                            }
                            crate::common::Compression::Zst | crate::common::Compression::Zstd =>
                            {
                                decompress::tarzst(&outpath, &src)?;
                            }
                            crate::common::Compression::Bz2 =>
                            {
                                decompress::tarbz2(&outpath, &src)?;
                            }
                            crate::common::Compression::Not =>
                            {
                                decompress::vanilla(&outpath, &src)?;
                            }
                        }
                        info!("🥩 You have extracted your source at {}", outpath.display());
                        Ok(())
                    }
                    crate::common::SupportedFormat::Dir(_) =>
                    {
                        unreachable!(
                            "This should never be a directory since we already checked it!"
                        )
                    }
                },
                Err(err) =>
                {
                    eprintln!("{}", err);
                    error!(?err);
                    Err(io::Error::new(io::ErrorKind::Unsupported, err.to_string()))
                }
            }
        }
        else
        {
            let err = io::Error::new(io::ErrorKind::Unsupported, "Directory detected.");
            error!(?err);
            Err(err)
        }
    }
}
