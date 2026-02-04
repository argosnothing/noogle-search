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

use crate::data::Doc;
use owo_colors::OwoColorize;
use std::io::Write;
use std::process::{Command, Stdio};

pub fn print_preview(doc: &Doc) {
    println!("{}\n", doc.meta.title.bright_cyan().bold());

    if let Some(sig) = &doc.meta.signature {
        println!("{}", "Type Signature:".yellow().bold());
        highlight_code(sig.trim(), "nix");
        println!("\n");
    }

    if let Some(lambda) = &doc.meta.lambda_expr {
        println!("{}", "Definition:".yellow().bold());
        highlight_code(lambda, "nix");
    }

    if let Some(content) = &doc.content {
        if let Some(text) = &content.content {
            print_content(text);
        }
    }

    if let Some(pos) = &doc.meta.lambda_position {
        let file_short = pos.file.split('/').last().unwrap_or(&pos.file);
        println!(
            "\n{} {}:{}:{}",
            "Source:".dimmed(),
            file_short.blue(),
            pos.line,
            pos.column
        );
    }

    if let Some(aliases) = &doc.meta.aliases {
        if !aliases.is_empty() {
            println!("\n{}", "Aliases:".yellow().bold());
            for alias in aliases {
                println!("  {}", alias.join(".").dimmed());
            }
        }
    }
}

fn highlight_code(code: &str, lang: &str) {
    if let Ok(mut child) = Command::new("bat")
        .args(&[
            "--color=always",
            "--style=plain",
            &format!("--language={}", lang),
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
    {
        if let Some(mut stdin) = child.stdin.take() {
            let _ = stdin.write_all(code.as_bytes());
        }
        if let Ok(output) = child.wait_with_output() {
            print!("{}", String::from_utf8_lossy(&output.stdout));
            return;
        }
    }
    print!("{}", code);
}

fn print_content(text: &str) {
    let mut in_code_block = false;
    let mut code_buffer = String::new();
    let mut last_was_empty = false;
    let mut last_was_heading = false;
    let mut skip_next_usage_line = false;

    for line in text.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("```nix") || trimmed.starts_with("```") && trimmed.len() == 3 {
            if !in_code_block {
                in_code_block = true;
                code_buffer.clear();
            } else {
                in_code_block = false;
                if !code_buffer.is_empty() {
                    let trimmed_code = code_buffer.trim();
                    highlight_code(trimmed_code, "nix");
                    println!();
                }
            }
            last_was_heading = false;
            skip_next_usage_line = false;
        } else if in_code_block {
            code_buffer.push_str(line);
            code_buffer.push('\n');
        } else if trimmed.starts_with(":::") {
            continue;
        } else if trimmed.is_empty() {
            if !last_was_empty && !last_was_heading {
                println!();
                last_was_empty = true;
            }
            last_was_heading = false;
        } else if skip_next_usage_line && trimmed.contains("usage example") {
            skip_next_usage_line = false;
            continue;
        } else if trimmed.starts_with(": ") {
            let content = trimmed.trim_start_matches(": ");
            println!("  - {}", strip_inline_code(content));
            last_was_empty = false;
            last_was_heading = false;
            skip_next_usage_line = false;
        } else if trimmed.starts_with("# ") {
            let heading = trimmed.trim_start_matches("# ");
            println!("{}", strip_inline_code(heading).yellow().bold());
            last_was_empty = false;
            last_was_heading = true;
            skip_next_usage_line = heading == "Examples";
        } else if trimmed.starts_with("## ") {
            let heading = trimmed.trim_start_matches("## ");
            println!("{}", strip_inline_code(heading).yellow());
            last_was_empty = false;
            last_was_heading = true;
            skip_next_usage_line = heading == "Examples";
        } else {
            println!("{}", strip_inline_code(line));
            last_was_empty = false;
            last_was_heading = false;
            skip_next_usage_line = false;
        }
    }
}

fn strip_inline_code(text: &str) -> String {
    unescape_markdown(&text.replace('`', ""))
}

fn unescape_markdown(text: &str) -> String {
    text.replace(r"\.", ".")
        .replace(r"\-", "-")
        .replace(r"\*", "*")
        .replace(r"\_", "_")
        .replace(r"\#", "#")
        .replace(r"\[", "[")
        .replace(r"\]", "]")
        .replace(r"\(", "(")
        .replace(r"\)", ")")
        .replace(r"\{", "{")
        .replace(r"\}", "}")
        .replace(r"\|", "|")
        .replace(r"\\", "\\")
}
