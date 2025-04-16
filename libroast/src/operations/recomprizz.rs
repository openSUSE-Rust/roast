use crate::{
    operations::{
        cli::{
            RawArgs,
            RecomprizzArgs,
            RoastArgs,
        },
        raw::raw_opts,
        roast::roast_opts,
    },
    utils::{
        process_globs,
        start_tracing,
    },
};
use std::{
    io,
    os::unix::ffi::OsStrExt,
    path::{
        Path,
        PathBuf,
    },
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

pub fn recomprizz_opts(recomprizz_args: RecomprizzArgs) -> io::Result<()>
{
    let start_trace = false;
    start_tracing();

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

    let target = process_globs(&recomprizz_args.target)?;
    let target = target.canonicalize().unwrap_or(target);
    let raw_args = RawArgs { target: target.clone(), outdir: Some(outpath_for_raw.to_path_buf()) };

    raw_opts(raw_args, start_trace)?;

    // Yuck!
    let out_filename = match recomprizz_args.rename
    {
        Some(ref v) => &Path::new(v).to_path_buf(),
        None => &{
            let mut filename = target.clone();
            while let Some(file_prefix) = &mut filename.file_stem()
            {
                let file_prefix_str_bytes = file_prefix.as_bytes();
                if let Some(last_byte) = file_prefix_str_bytes.last()
                {
                    if last_byte.is_ascii_digit()
                    {
                        filename = PathBuf::from(&file_prefix);
                        break;
                    }
                }
                filename = PathBuf::from(&file_prefix);
            }
            filename
        },
    };

    let file_extension = match recomprizz_args.compression
    {
        crate::common::Compression::Gz => "tar.gz",
        crate::common::Compression::Xz => "tar.xz",
        crate::common::Compression::Zst | crate::common::Compression::Zstd => "tar.zst",
        crate::common::Compression::Bz2 => "tar.bz",
        crate::common::Compression::Not => "tar",
    };

    let out_filename = format!("{}.{}", out_filename.display(), file_extension);

    let roast_args = RoastArgs {
        target: outpath_for_raw.to_path_buf(),
        additional_paths: None,
        exclude: recomprizz_args.exclude,
        outfile: PathBuf::from(&out_filename),
        outdir: recomprizz_args.outdir,
        preserve_root: false,
        reproducible: recomprizz_args.reproducible,
        ignore_git: recomprizz_args.ignore_git,
        ignore_hidden: recomprizz_args.ignore_hidden,
        include: recomprizz_args.include,
    };

    roast_opts(&roast_args, start_trace)?;

    info!("📥 Finished Recomprizz.");
    Ok(())
}
