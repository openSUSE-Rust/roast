use clap::ValueEnum;
use std::{
    fmt::{
        self,
        Display,
    },
    path::PathBuf,
};

#[derive(ValueEnum, Default, Debug, Clone, Copy)]
pub enum Compression
{
    Gz,
    Xz,
    #[default]
    Zst,
    Bz2,
}

impl Display for Compression
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        let msg = match self
        {
            Compression::Gz => "gz",
            Compression::Xz => "xz",
            Compression::Zst => "zst",
            Compression::Bz2 => "bz2",
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
