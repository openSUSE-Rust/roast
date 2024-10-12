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

#[derive(Debug)]
pub enum SupportedFormat
{
    Compressed(Compression, PathBuf),
    Dir(PathBuf),
}

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
