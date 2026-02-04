// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 argos_nothing

mod cache;
mod commands;
mod data;
mod format;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::io::{self, ErrorKind};

#[derive(Parser)]
#[command(name = "noogle-search")]
#[command(about = "Search Noogle functions for television/fzf")]
struct Cli {
    #[arg(short = 'f', long = "filter")]
    filter: Option<String>,

    query: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
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
        Some(Commands::Print { filter }) => {
            let response = cache::load_data()?;
            commands::print::execute(&response, filter.as_deref());
        }
        Some(Commands::Preview { name }) => {
            let response = cache::load_data()?;
            commands::preview::execute(&response, &name)?;
        }
        Some(Commands::OpenSource { name }) => {
            let response = cache::load_data()?;
            commands::open_source::execute(&response, &name)?;
        }
        Some(Commands::OpenNoogle { name }) => {
            let response = cache::load_data()?;
            commands::open_noogle::execute(&response, &name)?;
        }
        None => {
            commands::search::execute(cli.filter, cli.query)?;
        }
    }

    Ok(())
}
