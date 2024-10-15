use clap::Parser;
use libroast::{
    decompress,
    is_supported_format,
};
use roast_cli::cli;
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

fn main() -> io::Result<()>
{
    let raw_args = cli::RawArgs::parse();
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

    info!("🥩 Starting Raw.");
    if raw_args.target.is_file()
    {
        match is_supported_format(&raw_args.target)
        {
            Ok(target) => match target
            {
                libroast::common::SupportedFormat::Compressed(mime_type, src) =>
                {
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
