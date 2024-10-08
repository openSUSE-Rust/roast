use clap::ValueEnum;
use std::path::PathBuf;

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
