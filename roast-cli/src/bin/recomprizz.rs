use roast_cli::recomprizz::recomprizz_cli_stub;

pub fn main()
{
    if recomprizz_cli_stub().is_err()
    {
        std::process::exit(1)
    }
}
