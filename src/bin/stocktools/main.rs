use anyhow::Result;
use clap::{Parser, Subcommand};

mod adg;
use adg::AdgSubcommand;

mod fcr;
use fcr::FcrSubcommand;

#[derive(Subcommand, Debug)]
enum Commands {
    Adg(AdgSubcommand),
    Fcr(FcrSubcommand),
}

#[derive(Parser)]
#[command(
    author,
    version,
    propagate_version = true,
    about = "Tools for managing and identifying livestock breeds, growth, and health."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    #[arg(long, short, help = "Print extra detail in output logs")]
    verbose: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Adg(subcommand) => subcommand.run(),
        Commands::Fcr(subcommand) => subcommand.run(),
    }
}
