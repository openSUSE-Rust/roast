use crate::{
    cli::{RawArgs, RecomprizzArgs, RoastArgs},
    raw::raw_opts,
    roast::roast_opts,
};
use clap::Parser;
use std::{
    io,
    path::{Path, PathBuf},
};
use terminfo::capability as cap;
use terminfo::Database;
#[allow(unused_imports)]
use tracing::{debug, error, info, trace, warn, Level};
use tracing_subscriber::EnvFilter;

pub fn recomprizz_cli_stub() -> io::Result<()> {
    let recomprizz_args = RecomprizzArgs::parse();
    recomprizz_opts(recomprizz_args)
}

pub(crate) fn recomprizz_opts(recomprizz_args: RecomprizzArgs) -> io::Result<()> {
    let terminfodb = Database::from_env().map_err(|e| {
        error!(err = ?e, "Unable to access terminfo db. This is a bug!");
        io::Error::new(
            io::ErrorKind::Other,
            "Unable to access terminfo db. This is a bug! Setting color option to false!",
        )
    });

    let is_termcolorsupported = match terminfodb {
        Ok(hasterminfodb) => hasterminfodb.get::<cap::MaxColors>().is_some(),
        Err(_) => false,
    };
    let filter_layer = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let builder = tracing_subscriber::fmt()
        .with_level(true)
        .with_ansi(is_termcolorsupported)
        .with_env_filter(filter_layer)
        .with_level(true);

    let builder = if cfg!(debug_assertions) {
        builder.with_file(true).with_line_number(true)
    } else {
        builder
    };

    builder.init();

    info!("📤 Starting Recomprizz.");
    debug!(?recomprizz_args);
    let tmp_binding_for_raw = tempfile::TempDir::new().map_err(|err| {
        error!(?err, "Failed to create temporary directory");
        err
    })?;
    let outpath_for_raw = &tmp_binding_for_raw.path();

    let tmp_binding_for_roast = tempfile::TempDir::new().map_err(|err| {
        error!(?err, "Failed to create temporary directory");
        err
    })?;
    let outpath_for_roast = &tmp_binding_for_roast.path();

    let raw_args =
        RawArgs { target: recomprizz_args.target, outpath: Some(outpath_for_raw.to_path_buf()) };

    raw_opts(raw_args)?;

    let mut out_filename = match recomprizz_args.rename {
        Some(v) => PathBuf::from(v),
        None => {
            let mut filename = recomprizz_args.target.clone();
            while let Some(file_prefix) = filename.file_stem() {
                filename = PathBuf::from(file_prefix);
            }
            filename
        }
    };

    let file_extension = match recomprizz_args.compression {
        libroast::common::Compression::Gz => "tar.gz",
        libroast::common::Compression::Xz => "tar.xz",
        libroast::common::Compression::Zst => "tar.zst",
        libroast::common::Compression::Bz2 => "tar.bz",
        libroast::common::Compression::Not => "tar",
    };

    out_filename = out_filename.with_extension(file_extension);

    let roast_args = RoastArgs {
        target: outpath_for_raw.to_path_buf(),
        additional_paths: None,
        outpath: outpath_for_roast.to_path_buf(),
        preserve_root: false,
        reproducible: recomprizz_args.reproducible,
    };

    roast_opts(roast_args)?;

    info!("📥 Finished Recomprizz.");
    Ok(())
}
