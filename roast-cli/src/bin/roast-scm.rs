use roast_cli::roast_scm::roast_scm_cli_stub;

fn main()
{
    if roast_scm_cli_stub().is_err()
    {
        std::process::exit(1)
    }
}
