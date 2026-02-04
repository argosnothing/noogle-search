// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 argos_nothing <argosnothing@gmail.com>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

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

    // Show GPL disclaimer only once, first time any command is run
    if cache::should_show_disclaimer()? {
        eprintln!("noogle-search  Copyright (C) 2026  argos_nothing");
        eprintln!("This program comes with ABSOLUTELY NO WARRANTY.");
        eprintln!("This is free software, and you are welcome to redistribute it");
        eprintln!("under certain conditions. See LICENSE for details.");
        eprintln!("\nPress Enter to continue...");
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        
        cache::mark_disclaimer_shown()?;
    }

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
