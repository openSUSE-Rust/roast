use roast_cli::roast::roast_cli_stub;

fn main()
{
    if roast_cli_stub().is_err()
    {
        std::process::exit(1)
    }
}
