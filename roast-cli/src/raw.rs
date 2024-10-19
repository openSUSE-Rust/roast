use crate::{
    cli,
    start_tracing,
};
use clap::Parser;
use libroast::{
    decompress,
    is_supported_format,
};
use std::io;
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
    Level,
};
use tracing_subscriber::EnvFilter;

pub fn raw_cli_stub() -> io::Result<()>
{
    let raw_args = cli::RawArgs::parse();
    raw_opts(raw_args, true)
}

pub(crate) fn raw_opts(raw_args: cli::RawArgs, start_trace: bool) -> io::Result<()>
{
    if start_trace
    {
        start_tracing();
    }

    info!("🥩 Starting Raw.");
    if raw_args.target.is_file()
    {
        match is_supported_format(&raw_args.target)
        {
            Ok(target) => match target
            {
                libroast::common::SupportedFormat::Compressed(mime_type, src) =>
                {
                    info!(?mime_type);
                    let outpath =
                        raw_args.outpath.unwrap_or(std::env::current_dir().inspect_err(|e| {
                            error!(?e, "Unable to determine current directory!");
                        })?);
                    match mime_type
                    {
                        libroast::common::Compression::Gz =>
                        {
                            decompress::targz(&outpath, &src)?;
                        }
                        libroast::common::Compression::Xz =>
                        {
                            decompress::tarxz(&outpath, &src)?;
                        }
                        libroast::common::Compression::Zst =>
                        {
                            decompress::tarzst(&outpath, &src)?;
                        }
                        libroast::common::Compression::Bz2 =>
                        {
                            decompress::tarbz2(&outpath, &src)?;
                        }
                        libroast::common::Compression::Not =>
                        {
                            decompress::vanilla(&outpath, &src)?;
                        }
                    }
                    info!("🥩 You have extracted your source at {}", outpath.display());
                    Ok(())
                }
                libroast::common::SupportedFormat::Dir(_) =>
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
