use clap::{Parser, Subcommand};

mod commands;
mod drm;
mod types;

#[derive(Parser)]

struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    List,
}


fn main() {
    let cli : Cli = Cli::parse();
    match cli.command {
        Command::List => commands::list::list_monitors(),
    }
}
