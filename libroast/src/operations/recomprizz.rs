// SPDX-License-Identifier: MPL-2.0

// Copyright (C) 2025 Soc Virnyl Estela and contributors

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::{
    operations::{
        cli::{
            RawArgs,
            RecomprizzArgs,
            RoastArgs,
            print_completions,
        },
        raw::raw_opts,
        roast::roast_opts,
    },
    utils::{
        is_supported_format,
        process_globs,
        start_tracing,
    },
};
use clap::CommandFactory;
use regex::Regex;
use std::{
    io,
    path::PathBuf,
};
#[allow(unused_imports)]
use tracing::{
    Level,
    debug,
    error,
    info,
    trace,
    warn,
};

/// A combination of `raw` and `roast`. It extracts a tarball of a supported
/// mime-type and reproduces another tarball that might be of a different
/// filename or compression option e.g. `source.tar.gz` -> `source.tar.zst`.
///
/// This function relies on the arguments provided by
/// `crate::cli::RecomprizzArgs`.
pub fn recomprizz_opts(recomprizz_args: RecomprizzArgs) -> io::Result<()>
{
    if let Some(ref subcommand) = recomprizz_args.subcommands
    {
        let mut cmd = RecomprizzArgs::command();
        match subcommand
        {
            crate::operations::cli::Subcommands::GenerateCompletionsFor { shell } =>
            {
                print_completions(*shell, &mut cmd);
            }
        }
    }
    else
    {
        #[allow(clippy::if_same_then_else)] // It's not actually the same
        if cfg!(feature = "obs")
        {
            start_tracing();
        }
        else if !recomprizz_args.silent
        {
            start_tracing();
        }
        info!("📤 Starting Recomprizz.");
        debug!(?recomprizz_args);
        let tmp_binding_for_raw = tempfile::Builder::new()
            .prefix(".raaaaaaaaaaaaaaaaawwwwww")
            .rand_bytes(8)
            .tempdir()
            .inspect_err(|err| {
                error!(?err, "Failed to create temporary directory");
            })?;
        let outpath_for_raw = &tmp_binding_for_raw.path();

        let target = process_globs(&recomprizz_args.target.ok_or("No target provided.").map_err(
            |err| {
                error!(err);
                io::Error::new(io::ErrorKind::InvalidInput, err)
            },
        )?)?;
        let target = target.canonicalize().unwrap_or(target);
        let raw_args = RawArgs {
            target: Some(target.clone()),
            outdir: Some(outpath_for_raw.to_path_buf()),
            silent: recomprizz_args.silent,
            subcommands: None,
        };

        raw_opts(raw_args, false)?;

        let file_extension = recomprizz_args.compression.to_extension();

        let out_filename = match recomprizz_args.rename
        {
            Some(rename_string) => match recomprizz_args.renamepattern
            {
                Some(pattern) =>
                {
                    info!("Renaming with regex `{}` with pattern `{}`.", rename_string, pattern);
                    let re = Regex::new(&rename_string).map_err(|err| {
                        error!(?err);
                        io::Error::other(err)
                    })?;
                    let filename = target.file_name().unwrap_or_default().to_string_lossy();
                    let after = re.replace_all(&filename, pattern);
                    after.to_string()
                }
                None =>
                {
                    info!("Setting hard-coded name: {}", rename_string);
                    rename_string.to_string()
                }
            },
            None =>
            {
                let supported_format = is_supported_format(&target).map_err(|err| {
                    error!(?err);
                    io::Error::other(err)
                })?;
                match supported_format
                {
                    crate::common::SupportedFormat::Compressed(compression, path_buf) =>
                    {
                        let filename = path_buf.file_name().unwrap_or_default().to_string_lossy();
                        match filename.rsplit_once(&compression.to_extension())
                        {
                            Some((name, _)) => name.to_string(),
                            None =>
                            {
                                warn!("Not able to remove extension.");
                                warn!(
                                    "The file might be a supported format but is using a \
                                     different file extension."
                                );
                                warn!(
                                    "Not removing old file extension. This will result to an \
                                     undesirable rename of the file."
                                );
                                filename.to_string()
                            }
                        }
                    }
                    crate::common::SupportedFormat::Dir(_) =>
                    {
                        unreachable!("Only files are supported in `recomprizz`.")
                    }
                }
            }
        };

        let out_filename = format!("{}{}", out_filename, file_extension);

        let roast_args = RoastArgs {
            target: Some(outpath_for_raw.to_path_buf()),
            additional_paths: None,
            exclude: recomprizz_args.exclude,
            outfile: Some(PathBuf::from(&out_filename)),
            outdir: recomprizz_args.outdir,
            preserve_root: false,
            reproducible: recomprizz_args.reproducible,
            ignore_git: recomprizz_args.ignore_git,
            ignore_hidden: recomprizz_args.ignore_hidden,
            include: recomprizz_args.include,
            silent: recomprizz_args.silent,
            subcommands: None,
        };

        roast_opts(&roast_args, false)?;

        info!("📥 Finished Recomprizz.");
    }
    Ok(())
}
