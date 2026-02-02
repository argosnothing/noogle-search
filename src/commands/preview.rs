use anyhow::{anyhow, Result};
use crate::data::NoogleResponse;
use crate::format;

pub fn execute(response: &NoogleResponse, name: &str) -> Result<()> {
    let doc = response.data.iter()
        .find(|d| d.matches_name(name))
        .ok_or_else(|| anyhow!("Function '{}' not found", name))?;

    format::print_preview(doc);
    Ok(())
}
