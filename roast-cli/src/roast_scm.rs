use crate::{
    cli,
    roast_scm_opts,
};
use clap::Parser;
use std::io;

pub fn roast_scm_cli_stub() -> io::Result<()>
{
    let roast_scm_args = cli::RoastScmArgs::parse();
    roast_scm_opts(&roast_scm_args, true)
}
