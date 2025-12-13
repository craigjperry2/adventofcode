mod cli;
mod core;

fn main() -> color_eyre::Result<()> {
    // Better errors and load env (.env, .envrc already handled by direnv)
    color_eyre::install()?;
    let _ = dotenvy::dotenv();

    cli::main()
}
