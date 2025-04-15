use crate::common::Compression;
use git2::{
    AutotagOption,
    FetchOptions,
    Repository,
    build::RepoBuilder,
};
use rayon::prelude::*;
use std::path::{
    Path,
    PathBuf,
};
