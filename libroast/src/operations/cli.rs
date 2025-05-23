//! Mostly structs that are used for `clap` for CLI arguments.
//! Also useful for just anything else not CLI.

use crate::common::Compression;
use clap::Parser;
use std::path::PathBuf;
#[allow(unused_imports)]
use tracing::{
    Level,
    debug,
    error,
    info,
    trace,
    warn,
};

#[derive(Debug, Parser)]
#[command(
    name = "roast",
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
                archive. Supports globbing."
    )]
    pub target: PathBuf,
    #[arg(
        long,
        short = 'i',
        help = "Additional paths such as files or directories in the target directory to include \
                to the archive. Their parent directory will be put next to the target directory's \
                work directory. The work directory is based on the preserve root option. This is \
                different from `--additional_paths`. Useful to override excluded directories. ⚠️ \
                Careful if the archive has whether preserved root set when it was created."
    )]
    pub include: Option<Vec<PathBuf>>,
    #[arg(
        long,
        short = 'E',
        help = "Additional paths such as files or directories from within target directory's work \
                directory to exclude when generating the archive."
    )]
    pub exclude: Option<Vec<PathBuf>>,
    #[arg(
        long,
        short = 'A',
        help = "Additional paths such as files or directories to add to the archive. Their parent \
                directory will be put next to the target directory. This is different from \
                `--include`. Optionally, one can add a path to a directory inside the archive \
                e.g. `-A some/file/to/archive,put/where/in/archive`. If directory does not exist, \
                it will be created."
    )]
    pub additional_paths: Option<Vec<String>>,
    #[arg(long, short = 'f', help = "Output file of the generated archive with path.")]
    pub outfile: PathBuf,
    #[arg(long, short = 'd', help = "Output path of the generated archive.")]
    pub outdir: Option<PathBuf>,
    #[arg(
        long,
        short = 'p',
        help = "Preserve root directory instead of only archiving relative paths.",
        default_value_t = false,
        action = clap::ArgAction::Set
    )]
    pub preserve_root: bool,
    #[arg(
        long,
        short = 'r',
        help = "Allow reproducibility for Reproducible Builds.",
        default_value_t = false,
        action = clap::ArgAction::Set
    )]
    pub reproducible: bool,
    #[arg(
        long,
        short = 'g',
        help = "Whether to ignore git related metadata, files and directories.",
        default_value_t = true,
        action = clap::ArgAction::Set
    )]
    pub ignore_git: bool,
    #[arg(
        long,
        short = 'I',
        help = "Whether to ignore hidden directories and files or what we call dotfiles. Does not affect `--ignore-git`.",
        default_value_t = false,
        action = clap::ArgAction::Set
    )]
    pub ignore_hidden: bool,
}

#[derive(Debug, Parser)]
#[command(
    name = "raw",
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
    #[arg(
        long,
        short = 't',
        help = "Target tarball file to extract and decompress. Supports globbing."
    )]
    pub target: PathBuf,
    #[arg(long, short = 'd', help = "Output directory of extracted archive.")]
    pub outdir: Option<PathBuf>,
}

#[derive(Debug, Parser)]
#[command(
    name = "recomprizz",
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
    #[arg(
        long,
        short = 't',
        help = "Target tarball file to extract and recompress. Supports globbing."
    )]
    pub target: PathBuf,
    #[arg(
        long,
        short = 'i',
        help = "Additional paths such as files or directories in the target directory to include \
                to the archive. Their parent directory will be put next to the target directory's \
                work directory. The work directory is based on the preserve root option. This is \
                different from `--additional_paths`. Useful to override excluded directories."
    )]
    pub include: Option<Vec<PathBuf>>,
    #[arg(
        long,
        short = 'E',
        help = "Additional paths such as files or directories from within target directory's work \
                directory to exclude when generating the archive. ⚠️ Careful if the archive has \
                whether preserved root set when it was created."
    )]
    pub exclude: Option<Vec<PathBuf>>,
    #[arg(
        long,
        short = 'A',
        help = "Additional paths such as files or directories to add to the archive. Their parent \
                directory will be put next to the target directory. This is different from \
                `--include`. Optionally, one can add a path to a directory inside the archive \
                e.g. `-A some/file/to/archive,put/where/in/archive`. If directory does not exist, \
                it will be created."
    )]
    pub additional_paths: Option<Vec<String>>,
    #[arg(long, short = 'd', help = "Output directory of recompressed archive.")]
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
        help = "Allow reproducibility for Reproducible Builds.",
        default_value_t = false,
        action = clap::ArgAction::Set
    )]
    pub reproducible: bool,
    #[arg(
        long,
        short = 'g',
        help = "Whether to ignore git related metadata, files and directories.",
        default_value_t = true,
        action = clap::ArgAction::Set
    )]
    pub ignore_git: bool,
    #[arg(
        long,
        short = 'I',
        help = "Whether to ignore hidden directories and files or what we call dotfiles. Does not affect `--ignore-git`.",
        default_value_t = false,
        action = clap::ArgAction::Set
    )]
    pub ignore_hidden: bool,
}

#[derive(Debug, Parser)]
#[command(
    name = "roast-scm",
    author = "Soc Virnyl Estela",
    about = "Create archive tarballs from remote git repositories.",
    after_long_help = "Set verbosity and tracing through `RUST_LOG` environmental variable e.g. \
                       `RUST_LOG=trace`",
    help_template = "{name} {version} - \
                     {about}\n\n{usage}\n\n{all-args}\n{after-help}\nMaintained by {author} \
                     <contact@uncomfyhalomacro.pl>.",
    version
)]
pub struct RoastScmArgs
{
    #[arg(long, short = 'g', help = "Remote URL to the git repository.", alias = "url")]
    pub git_repository_url: String,
    #[arg(
        long,
        short = 'E',
        help = "Additional paths such as files or directories from within target repository's \
                work directory to exclude when generating the archive."
    )]
    pub exclude: Option<Vec<PathBuf>>,
    #[arg(long, help = "Revision or tag. It can also be a specific commit hash.")]
    pub revision: String,
    #[arg(
        long, default_value_t = 1,
        action = clap::ArgAction::Set,
        help = "The depth of cloning the repository.")]
    pub depth: i32,
    #[arg(
        long, default_value_t = true,
        action = clap::ArgAction::Set,
        help = "If the cloned repository should be temporary."
    )]
    pub is_temporary: bool,
    #[arg(
        long,
        short = 'f',
        help = "Output file of the generated archive with path. If not provided, attempts to \
                write the filename based on project name and revision."
    )]
    pub outfile: Option<PathBuf>,
    #[arg(long, short = 'd', help = "Output path of the generated archive.")]
    pub outdir: Option<PathBuf>,
    #[arg(
        long,
        short = 'r',
        help = "Allow reproducibility for Reproducible Builds.",
        default_value_t = false,
        action = clap::ArgAction::Set
    )]
    pub reproducible: bool,
    #[arg(
        long,
        short = 'g',
        help = "Whether to ignore git related metadata, files and directories.",
        default_value_t = true,
        action = clap::ArgAction::Set
    )]
    pub ignore_git: bool,
    #[arg(
        long,
        short = 'I',
        help = "Whether to ignore hidden directories and files or what we call dotfiles. Does not affect `--ignore-git`.",
        default_value_t = false,
        action = clap::ArgAction::Set
    )]
    pub ignore_hidden: bool,
    #[arg(long, short = 'c', help = "Compression to use.", default_value_t)]
    pub compression: Compression,
}
