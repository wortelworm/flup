use anyhow::Result;
use clap::{Parser, Subcommand};
use inputs::*;

mod inputs;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Execute command to update my nixos config
    Update,
    /// Default command, show latest input date for my nixos config
    Show,
}

fn main() -> Result<()> {
    let home_dir = simple_home_dir::home_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();
    let lock_file = home_dir.clone() + "/.dotfiles/flake.lock";
    let update_script = home_dir + "/.dotfiles/scripts/update.sh";

    let cli: Cli = Cli::parse();
    match cli.command.unwrap_or(Commands::Show) {
        Commands::Show => {
            let latest = Inputs::from_file(&lock_file)?.latest();
            let num_days = (chrono::Utc::now() - latest).num_days();
            let date = format_datetime(latest.into());
            println!("Latest input is from {num_days} days ago ({date}).",);
        }
        Commands::Update => {
            let status = std::process::Command::new(update_script).status()?;
            println!("Exited with status {}!", status);
        }
    }

    Ok(())
}

