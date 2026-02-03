use crate::data::NoogleResponse;
use anyhow::{Result, anyhow};
use std::process::Command;

pub fn execute(response: &NoogleResponse, name: &str) -> Result<()> {
    let doc = response
        .data
        .iter()
        .find(|d| d.matches_name(name))
        .ok_or_else(|| anyhow!("Function '{}' not found", name))?;

    let path = doc.meta.path.join("/");
    let url = format!("https://noogle.dev/f/{}", path);
    
    Command::new("xdg-open")
        .arg(&url)
        .spawn()?;

    Ok(())
}
