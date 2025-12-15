mod cli;
mod core;
mod day01;
mod parsers;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let _ = dotenvy::dotenv();

    cli::main()
}
