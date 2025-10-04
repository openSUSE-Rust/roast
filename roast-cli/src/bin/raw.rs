use roast_cli::raw::raw_cli_stub;

fn main()
{
    if raw_cli_stub().is_err()
    {
        std::process::exit(1)
    }
}
