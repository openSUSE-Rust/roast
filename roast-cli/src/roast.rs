use crate::{
    cli,
    roast_opts,
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
pub fn roast_cli_stub() -> io::Result<()>
{
    let roast_args = cli::RoastArgs::parse();
    roast_opts(roast_args, true)
}
