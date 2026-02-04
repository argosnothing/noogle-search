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

use anyhow::Result;
use std::env;
use std::process::Command;

pub fn execute(initial_filter: Option<String>, initial_query: Option<String>) -> Result<()> {
    let exe_path = env::current_exe()?;

    let initial_cmd = if let Some(filter) = &initial_filter {
        format!("{} print --filter {}", exe_path.display(), filter)
    } else {
        format!("{} print", exe_path.display())
    };

    let mut fzf_args = vec![
        "--preview".to_string(),
        format!("{} preview {{}}", exe_path.display()),
        "--preview-window=wrap".to_string(),
        "--scheme=history".to_string(),
        "--layout=reverse".to_string(),
        "--with-nth=1".to_string(),
        "--delimiter=\t".to_string(),
        "--bind".to_string(),
        format!("ctrl-o:execute({} open-source {{}})", exe_path.display()),
        "--bind".to_string(),
        format!("ctrl-n:execute({} open-noogle {{}})", exe_path.display()),
        "--bind".to_string(),
        format!("ctrl-l:reload({} print --filter lib)", exe_path.display()),
        "--bind".to_string(),
        format!("ctrl-b:reload({} print --filter builtins)", exe_path.display()),
        "--bind".to_string(),
        format!("ctrl-p:reload({} print --filter pkgs)", exe_path.display()),
        "--bind".to_string(),
        format!("ctrl-a:reload({} print)", exe_path.display()),
        "--header".to_string(),
        "Ctrl-L: lib | Ctrl-B: builtins | Ctrl-P: pkgs | Ctrl-A: all\nCtrl-O: source | Ctrl-N: noogle | Ctrl-/: preview".to_string(),
    ];

    if let Some(query) = initial_query {
        fzf_args.push("--query".to_string());
        fzf_args.push(query);
    }

    let mut child = Command::new("fzf")
        .args(&fzf_args)
        .stdin(
            Command::new("sh")
                .arg("-c")
                .arg(&initial_cmd)
                .stdout(std::process::Stdio::piped())
                .spawn()?
                .stdout
                .take()
                .unwrap(),
        )
        .spawn()?;

    child.wait()?;
    Ok(())
}
