pub mod cli;
pub mod raw;
pub mod recomprizz;
pub mod roast;

use std::io;
use terminfo::{
    capability as cap,
    Database,
};
#[allow(unused_imports)]
use tracing::{
    debug,
    error,
    info,
    trace,
    warn,
    Level,
};
use tracing_subscriber::EnvFilter;

pub(crate) fn start_tracing()
{
    let terminfodb = Database::from_env().map_err(|e| {
        error!(err = ?e, "Unable to access terminfo db. This is a bug!");
        io::Error::new(
            io::ErrorKind::Other,
            "Unable to access terminfo db. This is a bug! Setting color option to false!",
        )
    });

    let is_termcolorsupported = match terminfodb
    {
        Ok(hasterminfodb) => hasterminfodb.get::<cap::MaxColors>().is_some(),
        Err(_) => false,
    };
    let filter_layer = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let builder = tracing_subscriber::fmt()
        .with_level(true)
        .with_ansi(is_termcolorsupported)
        .with_env_filter(filter_layer)
        .with_level(true);

    let builder = if cfg!(debug_assertions)
    {
        builder.with_file(true).with_line_number(true)
    }
    else
    {
        builder
    };

    builder.init();
}
