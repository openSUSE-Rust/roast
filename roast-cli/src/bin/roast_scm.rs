use roast_cli::roast_scm::roast_scm_cli_stub;
use std::io;

fn main() -> io::Result<()>
{
    roast_scm_cli_stub()?;
    Ok(())
}
