use crate::{
    cli,
    roast_opts,
};
use clap::Parser;
use std::io;

pub fn roast_cli_stub() -> io::Result<()>
{
    let roast_args = cli::RoastArgs::parse();
    roast_opts(&roast_args, true)
}
