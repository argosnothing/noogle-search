// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 argos_nothing

use crate::data::NoogleResponse;
use anyhow::{Result, anyhow};
use std::process::Command;

pub fn execute(response: &NoogleResponse, input: &str) -> Result<()> {
    let doc = super::util::find_doc(response, input)?;

    let position = doc
        .meta
        .lambda_position
        .as_ref()
        .or(doc.meta.attr_position.as_ref())
        .ok_or_else(|| anyhow!("No source position available"))?;

    let rev = &response.upstream_info.rev;

    let file_path = position
        .file
        .split("-source/")
        .nth(1)
        .unwrap_or(&position.file);

    let url = format!(
        "https://github.com/NixOS/nixpkgs/blob/{}/{}#L{}",
        rev, file_path, position.line
    );

    eprintln!("Opening: {}", url);

    Command::new("xdg-open").arg(&url).spawn()?;

    Ok(())
}
