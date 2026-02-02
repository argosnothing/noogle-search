use crate::data::Doc;
use owo_colors::OwoColorize;
use std::io::Write;
use std::process::{Command, Stdio};

pub fn print_preview(doc: &Doc) {
    println!("{}\n", doc.meta.title.bright_cyan().bold());

    if let Some(sig) = &doc.meta.signature {
        println!("{}", "Type Signature:".yellow().bold());
        highlight_code(sig.trim(), "nix");
        println!();
    }

    if let Some(lambda) = &doc.meta.lambda_expr {
        println!("{}", "Definition:".yellow().bold());
        highlight_code(lambda, "nix");
        println!();
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
    println!("  {}", code);
}

fn print_content(text: &str) {
    let mut in_code_block = false;
    let mut code_buffer = String::new();

    for line in text.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("```nix") {
            in_code_block = true;
            code_buffer.clear();
        } else if trimmed == "```" && in_code_block {
            in_code_block = false;
            if !code_buffer.is_empty() {
                highlight_code(&code_buffer, "nix");
            }
        } else if in_code_block {
            code_buffer.push_str(line);
            code_buffer.push('\n');
        } else if trimmed.starts_with(":::") {
            continue;
        } else if trimmed.starts_with(": ") {
            println!("  - {}", trimmed.trim_start_matches(": "));
        } else if trimmed.starts_with("# ") {
            println!("\n{}", trimmed.trim_start_matches("# ").yellow().bold());
        } else if trimmed.starts_with("## ") {
            println!("\n{}", trimmed.trim_start_matches("## ").yellow());
        } else {
            println!("{}", line);
        }
    }
}
