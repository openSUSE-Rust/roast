use crate::{
    cli,
    raw_opts,
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

pub fn raw_cli_stub() -> io::Result<()>
{
    let raw_args = cli::RawArgs::parse();
    raw_opts(raw_args, true)
}
