use crate::{
    cli::RecomprizzArgs,
    recomprizz_opts,
};
use clap::Parser;
use std::io;
#[allow(unused_imports)]
use tracing::{
    debug,
    error,
    info,
    trace,
    warn,
    Level,
};

pub fn recomprizz_cli_stub() -> io::Result<()>
{
    let recomprizz_args = RecomprizzArgs::parse();
    recomprizz_opts(recomprizz_args)
}
