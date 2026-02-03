mod cache;
mod commands;
mod data;
mod format;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::io::{self, ErrorKind};

#[derive(Parser)]
#[command(name = "noogle-search-tv")]
#[command(about = "Search Noogle functions for television/fzf")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Print {
        #[arg(long)]
        filter: Option<String>,
    },
    Preview {
        name: String,
    },
    Search {
        filter: Option<String>,
    },
    OpenSource {
        name: String,
    },
    OpenNoogle {
        name: String,
    },
}

fn main() -> Result<()> {
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    }

    let result = run();

    if let Err(e) = &result {
        if let Some(io_err) = e.downcast_ref::<io::Error>() {
            if io_err.kind() == ErrorKind::BrokenPipe {
                return Ok(());
            }
        }
    }

    result
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Search { filter } => {
            commands::search::execute(filter)?;
        }
        _ => {
            let response = cache::load_data()?;

            match cli.command {
                Commands::Print { filter } => {
                    commands::print::execute(&response, filter.as_deref());
                }
                Commands::Preview { name } => {
                    commands::preview::execute(&response, &name)?;
                }
                Commands::OpenSource { name } => {
                    commands::open_source::execute(&response, &name)?;
                }
                Commands::OpenNoogle { name } => {
                    commands::open_noogle::execute(&response, &name)?;
                }
                Commands::Search { .. } => unreachable!(),
            }
        }
    }

    Ok(())
}
