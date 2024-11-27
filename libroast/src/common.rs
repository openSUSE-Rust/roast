use clap::ValueEnum;
use std::{
    fmt::{
        self,
        Display,
    },
    path::PathBuf,
};
#[allow(unused_imports)]
use tracing::{
    debug,
    error,
    info,
    warn,
};

#[derive(ValueEnum, Default, Debug, Clone, Copy)]
pub enum Compression
{
    Gz,
    Xz,
    #[default]
    Zst,
    Zstd,
    Bz2,
    Not,
}

impl Display for Compression
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        let msg = match self
        {
            Compression::Gz => "gz",
            Compression::Xz => "xz",
            Compression::Zst | Compression::Zstd => "zst",
            Compression::Bz2 => "bz2",
            Compression::Not => "tar (uncompressed)",
        };
        write!(f, "{}", msg)
    }
}

#[derive(Debug)]
pub enum SupportedFormat
{
    Compressed(Compression, PathBuf),
    Dir(PathBuf),
}

impl std::error::Error for UnsupportedFormat {}

#[derive(Debug)]
pub struct UnsupportedFormat
{
    pub ext: String,
}

impl Display for UnsupportedFormat
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "Unsupported archive format: {}", self.ext)
    }
}
