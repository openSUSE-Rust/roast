use crate::{
    decompress,
    operations::cli,
    utils::{
        is_supported_format,
        process_globs,
        start_tracing,
    },
};
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

pub fn raw_opts(raw_args: cli::RawArgs, start_trace: bool) -> io::Result<()>
{
    if start_trace
    {
        start_tracing();
    }

    info!("🥩 Starting Raw.");

    let target_path = process_globs(&raw_args.target)?;
    let target_path = target_path.canonicalize().unwrap_or(target_path);
    if target_path.is_file()
    {
        match is_supported_format(&raw_args.target)
        {
            Ok(target) => match target
            {
                crate::common::SupportedFormat::Compressed(mime_type, src) =>
                {
                    info!(?mime_type);
                    let outpath =
                        raw_args.outdir.unwrap_or(std::env::current_dir().inspect_err(|e| {
                            error!(?e, "Unable to determine current directory!");
                        })?);
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
                    unreachable!("This should never be a directory since we already checked it!")
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
