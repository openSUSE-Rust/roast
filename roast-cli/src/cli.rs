use std::path::PathBuf;

use clap::Parser;

#[allow(unused_imports)]
use tracing::{debug, error, info, trace, warn, Level};

#[derive(Debug, Parser)]
#[command(
    author = "Soc Virnyl Estela",
    about = "Archiver with high-level compression",
    after_long_help = "Set verbosity and tracing through `RUST_LOG` environmental variable e.g. `RUST_LOG=trace`",
    help_template = "{name} {version} - {about}\n\n{usage}\n\n{all-args}\n{after-help}\nMaintained by {author} <contact@uncomfyhalomacro.pl>.",
    version
)]
pub struct RoastArgs {
    #[arg(
        long,
        short = 't',
        help = "Target directory to archive. This will be set as the root directory of the archive."
    )]
    pub target: PathBuf,
    #[arg(
        long,
        short = 'a',
        help = "Additional paths to add to the archive. Their parent directory will be put next to the target directory."
    )]
    pub additional_paths: Option<Vec<PathBuf>>,
    #[arg(long, short = 'o', help = "Output path of tarball.")]
    pub outpath: PathBuf,
    #[arg(
        long,
        short = 'r',
        help = "Allow reproducibility for Reproducible Builds 🥴",
        default_value_t = false
    )]
    pub reproducible: bool,
}
