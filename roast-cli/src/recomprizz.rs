use crate::{
    cli::RecomprizzArgs,
    recomprizz_opts,
};
use clap::Parser;
use std::io;

pub fn recomprizz_cli_stub() -> io::Result<()>
{
    let recomprizz_args = RecomprizzArgs::parse();
    recomprizz_opts(recomprizz_args)
}
