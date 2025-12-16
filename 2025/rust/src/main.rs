mod cli;
mod core;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod parsers;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let _ = dotenvy::dotenv();

    cli::main()
}
