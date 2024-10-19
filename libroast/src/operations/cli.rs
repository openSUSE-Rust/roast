use crate::common::Compression;
use clap::Parser;
use std::path::PathBuf;
#[allow(unused_imports)]
use tracing::{
    debug,
    error,
    info,
    trace,
    warn,
    Level,
};

#[derive(Debug, Parser)]
#[command(
    author = "Soc Virnyl Estela",
    about = "Archiver with high-level compression",
    after_long_help = "Set verbosity and tracing through `RUST_LOG` environmental variable e.g. \
                       `RUST_LOG=trace`",
    help_template = "{name} {version} - \
                     {about}\n\n{usage}\n\n{all-args}\n{after-help}\nMaintained by {author} \
                     <contact@uncomfyhalomacro.pl>.",
    version
)]
pub struct RoastArgs
{
    #[arg(
        long,
        short = 't',
        help = "Target directory to archive. This will be set as the root directory of the \
                archive."
    )]
    pub target: PathBuf,
    #[arg(
        long,
        short = 'a',
        help = "Additional paths such as files or directories to add to the archive. Their parent \
                directory will be put next to the target directory."
    )]
    pub additional_paths: Option<Vec<PathBuf>>,
    #[arg(long, short = 'o', help = "Output file of the generated archive with path.")]
    pub outfile: PathBuf,
    #[arg(
        long,
        short = 'p',
        help = "Preserve root directory instead of only archiving relative paths. DEFAULT: false.",
        default_value_t = false
    )]
    pub preserve_root: bool,
    #[arg(
        long,
        short = 'r',
        help = "Allow reproducibility for Reproducible Builds. DEFAULT: false.",
        default_value_t = false
    )]
    pub reproducible: bool,
}

#[derive(Debug, Parser)]
#[command(
    author = "Soc Virnyl Estela",
    about = "Raw extractor and decompressor",
    after_long_help = "Set verbosity and tracing through `RUST_LOG` environmental variable e.g. \
                       `RUST_LOG=trace`",
    help_template = "{name} {version} - \
                     {about}\n\n{usage}\n\n{all-args}\n{after-help}\nMaintained by {author} \
                     <contact@uncomfyhalomacro.pl>.",
    version
)]
pub struct RawArgs
{
    #[arg(long, short = 't', help = "Target tarball file to extract and decompress.")]
    pub target: PathBuf,
    #[arg(
        long,
        short = 'o',
        help = "Output path of extracted archive. DEFAULT: current directory if omitted."
    )]
    pub outdir: Option<PathBuf>,
}

#[derive(Debug, Parser)]
#[command(
    author = "Soc Virnyl Estela",
    about = "Recompress to other compression formats",
    after_long_help = "Set verbosity and tracing through `RUST_LOG` environmental variable e.g. \
                       `RUST_LOG=trace`",
    help_template = "{name} {version} - \
                     {about}\n\n{usage}\n\n{all-args}\n{after-help}\nMaintained by {author} \
                     <contact@uncomfyhalomacro.pl>.",
    version
)]
pub struct RecomprizzArgs
{
    #[arg(long, short = 't', help = "Target tarball file to extract and recompress.")]
    pub target: PathBuf,
    #[arg(
        long,
        short = 'o',
        help = "Output directory of recompressed archive. DEFAULT: current directory if omitted."
    )]
    pub outdir: Option<PathBuf>,
    #[arg(long, short = 'c', help = "Compression to use.", default_value_t)]
    pub compression: Compression,
    #[arg(
        long,
        short = 'R',
        help = "Use this flag if you want a new filename to use ignoring the new file extension. \
                Omitting this flag will just fallback to basename."
    )]
    pub rename: Option<String>,
    #[arg(
        long,
        short = 'r',
        help = "Allow reproducibility for Reproducible Builds. DEFAULT: false.",
        default_value_t = false
    )]
    pub reproducible: bool,
}
