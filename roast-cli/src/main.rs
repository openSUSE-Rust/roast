use std::io;
use std::path::PathBuf;

use clap::Parser;
use libroast::compress;
use roast_cli::cli;

#[allow(unused_imports)]
use tracing::{debug, error, info, trace, warn, Level};

use terminfo::{capability as cap, Database};
use tracing_subscriber::EnvFilter;
use walkdir::WalkDir;

fn main() -> io::Result<()> {
    let roast_args = cli::RoastArgs::parse();
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

    info!("❤️‍🔥 Starting Roast.");
    debug!(?roast_args);
    let target_path = roast_args.target.canonicalize()?;
    let outpath = roast_args.outpath;
    println!("{}", target_path.display());
    let walker: Vec<PathBuf> = WalkDir::new(&target_path)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .map(|f| {
            debug!(?f);
            PathBuf::from(f.path())
        })
        .filter(|p| *p != target_path)
        .collect();

    let reproducible = roast_args.reproducible;

    match outpath.extension() {
        Some(ext) => match ext.to_string_lossy().to_string().as_str() {
            "gz" => {
                compress::targz(&outpath, &target_path, &walker, reproducible).map_err(|err| {
                    error!(?err);
                    err
                })?;
                info!("Your new tarball is now in {}", &outpath.display());
            }
            "zst" | "zstd" => {
                compress::tarzst(&outpath, &target_path, &walker, reproducible).map_err(|err| {
                    error!(?err);
                    err
                })?;
                info!("Your new tarball is now in {}", &outpath.display());
            }
            "bz" => {
                compress::tarbz2(&outpath, &target_path, &walker, reproducible).map_err(|err| {
                    error!(?err);
                    err
                })?;
                info!("Your new tarball is now in {}", &outpath.display());
            }
            "xz" => {
                compress::tarxz(&outpath, &target_path, &walker, reproducible).map_err(|err| {
                    error!(?err);
                    err
                })?;
                info!("Your new tarball is now in {}", &outpath.display());
            }
            _ => {
                let err = io::Error::new(
                    io::ErrorKind::Unsupported,
                    "Compression type unsupported. Valid options are xz, zst / zstd, bz, and gz.",
                );
                error!(?err);
                return Err(err);
            }
        },
        None => {
            let err = io::Error::new(
                io::ErrorKind::Unsupported,
                "Please provide a file extension. Compression type unsupported. Valid options are xz, zst / zstd, bz, and gz.",
            );
            error!(?err);
            return Err(err);
        }
    }
    Ok(())
}
