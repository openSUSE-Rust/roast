use crate::{
    cli::{
        RawArgs,
        RecomprizzArgs,
        RoastArgs,
    },
    raw::raw_opts,
    roast::roast_opts,
    start_tracing,
};
use clap::Parser;
use std::{
    io,
    path::{
        Path,
        PathBuf,
    },
};
#[allow(unused_imports)]
use tracing::{
    debug,
    error,
    info,
    trace,
    warn,
    Level,
};

pub fn recomprizz_cli_stub() -> io::Result<()>
{
    let recomprizz_args = RecomprizzArgs::parse();
    recomprizz_opts(recomprizz_args)
}

pub(crate) fn recomprizz_opts(recomprizz_args: RecomprizzArgs) -> io::Result<()>
{
    let start_trace = false;
    start_tracing();

    info!("📤 Starting Recomprizz.");
    debug!(?recomprizz_args);
    let tmp_binding_for_raw = tempfile::TempDir::new().map_err(|err| {
        error!(?err, "Failed to create temporary directory");
        err
    })?;
    let outpath_for_raw = &tmp_binding_for_raw.path();

    let raw_args = RawArgs {
        target: recomprizz_args.target.clone(),
        outpath: Some(outpath_for_raw.to_path_buf()),
    };

    raw_opts(raw_args, start_trace)?;

    // Yuck!
    let out_filename = match recomprizz_args.rename
    {
        Some(ref v) => &Path::new(v).to_path_buf(),
        None => &{
            let mut filename = recomprizz_args.target.clone();
            while let Some(file_prefix) = &mut filename.file_stem()
            {
                filename = PathBuf::from(&file_prefix);
            }
            filename
        },
    };

    let file_extension = match recomprizz_args.compression
    {
        libroast::common::Compression::Gz => "tar.gz",
        libroast::common::Compression::Xz => "tar.xz",
        libroast::common::Compression::Zst => "tar.zst",
        libroast::common::Compression::Bz2 => "tar.bz",
        libroast::common::Compression::Not => "tar",
    };

    let roast_outpath = match recomprizz_args.outpath
    {
        Some(v) => v,
        None => std::env::current_dir()?,
    };

    let roast_args = RoastArgs {
        target: outpath_for_raw.to_path_buf(),
        additional_paths: None,
        outpath: roast_outpath.join(out_filename.with_extension(file_extension)),
        preserve_root: false,
        reproducible: recomprizz_args.reproducible,
    };

    roast_opts(roast_args, start_trace)?;

    info!("📥 Finished Recomprizz.");
    Ok(())
}
