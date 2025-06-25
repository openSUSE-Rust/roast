//! Mostly structs that are used for `clap` for CLI arguments.
//! Also useful for just anything else not CLI.

use crate::common::Compression;
use clap::{
    Command,
    Parser,
    Subcommand,
};
use clap_complete::{
    Generator,
    Shell,
    generate,
};
use std::{
    io,
    path::PathBuf,
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

pub(crate) fn print_completions<G: Generator>(generator: G, cmd: &mut Command)
{
    generate(generator, cmd, cmd.get_name().to_string(), &mut io::stdout());
}

#[derive(Debug, Subcommand)]
pub enum Subcommands
{
    #[command(about = "Generate shell completions")]
    GenerateCompletionsFor
    {
        shell: Shell
    },
}

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
    pub target: Option<PathBuf>,
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
    pub outfile: Option<PathBuf>,
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
    #[arg(long, short = 'S', help = "Whether to silence the output or not.", default_value_t = true, action = clap::ArgAction::Set)]
    pub silent: bool,
    #[command(subcommand)]
    pub subcommands: Option<Subcommands>,
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
    pub target: Option<PathBuf>,
    #[arg(long, short = 'd', help = "Output directory of extracted archive.")]
    pub outdir: Option<PathBuf>,
    #[arg(long, short = 'S', help = "Whether to silence the output or not.", default_value_t = true, action = clap::ArgAction::Set)]
    pub silent: bool,
    #[command(subcommand)]
    pub subcommands: Option<Subcommands>,
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
    pub target: Option<PathBuf>,
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
        help = "Pass a string or regex value into this flag. This will be used with the \
                `--renamepattern` flag. If no value is provided to `--renamepattern`, this is \
                assumed to be a hard-coded name."
    )]
    pub rename: Option<String>,
    #[arg(
        long,
        requires = "rename",
        help = "Pass a replacement pattern using the capture groups into this flag. This will be \
                used with the `--rename` flag."
    )]
    pub renamepattern: Option<String>,
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
    #[arg(long, short = 'S', help = "Whether to silence the output or not.", default_value_t = true, action = clap::ArgAction::Set)]
    pub silent: bool,
    #[command(subcommand)]
    pub subcommands: Option<Subcommands>,
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
    #[arg(long, requires_if("true", "changesauthor"), short = 'C', action = clap::ArgAction::Set, default_value_t = false, help = "Whether to generate or update a changelog file or not.")]
    pub changesgenerate: bool,
    #[arg(
        long,
        short = 'A',
        requires = "changesgenerate",
        help = "Author to include during the changelog generation."
    )]
    pub changesauthor: Option<String>,
    #[arg(
        long,
        requires = "changesauthor",
        short = 'e',
        help = "Email of author to include during the changelog generation."
    )]
    pub changesemail: Option<String>,
    #[arg(
        long,
        alias = "caof",
        help = "Whether to specify a path to the changes file. Otherwise, it is the current \
                directory and the filename is the same filename prefix of the generated tarball \
                e.g. `source.tar.xz` will have `source.changes` file. If file exists, it prepends \
                the newest changes to the top-most part of the text file."
    )]
    pub changesoutfile: Option<PathBuf>,
    #[arg(
        long,
        help = "Whether to hard code the version or not. Set it to hard code one, otherwise, it \
                will use the generated version internally."
    )]
    pub set_version: Option<String>,
    #[arg(
        long,
        help = "Whether to hard code the name or not. Set it to hard code one, otherwise, it will \
                use the generated name internally."
    )]
    pub set_name: Option<String>,
    #[arg(long, short = 'U', help = "Remote URL to the git repository.", alias = "url")]
    pub git_repository_url: Option<String>,
    #[arg(
        long,
        short = 'E',
        help = "Additional paths such as files or directories from within target repository's \
                work directory to exclude when generating the archive."
    )]
    pub exclude: Option<Vec<PathBuf>>,
    #[arg(
        long,
        help = "Revision or tag. It can also be a specific commit hash or branch. Supports <https://git-scm.com/docs/git-rev-parse.html#_specifying_revisions>."
    )]
    pub revision: Option<String>,
    #[arg(
        long,
        help = "Pass a regex with capture groups. Required by `versionrewritepattern` flag. Each \
                capture group is labelled through increments of 1.",
        requires = "versionrewritepattern"
    )]
    pub versionrewriteregex: Option<String>,
    #[arg(long, help = "Pass a pattern from the capture groups from `versionrewriteregex` flag.")]
    pub versionrewritepattern: Option<String>,
    #[arg(
        long, default_value_t = 0,
        action = clap::ArgAction::Set,
        help = "The depth of cloning the repository.")]
    pub depth: i32,
    #[arg(
        long, default_value_t = true,
        action = clap::ArgAction::Set,
        help = "Whether the cloned repository should be deleted or not after the operation."
    )]
    pub is_temporary: bool,
    #[arg(
        long,
        short = 'f',
        help = "Output file of the generated archive with path. If not provided, attempts to \
                write the filename based on project name and revision based on <https://en.opensuse.org/openSUSE:Package_versioning_guidelines>."
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
    #[arg(long, short = 'S', help = "Whether to silence the output or not.", default_value_t = true, action = clap::ArgAction::Set)]
    pub silent: bool,
    #[command(subcommand)]
    pub subcommands: Option<Subcommands>,
}
