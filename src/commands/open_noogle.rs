// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 argos_nothing

use crate::data::NoogleResponse;
use anyhow::Result;
use std::process::Command;

pub fn execute(response: &NoogleResponse, input: &str) -> Result<()> {
    let doc = super::util::find_doc(response, input)?;
    let path = doc.meta.path.join("/");
    let url = format!("https://noogle.dev/f/{}", path);
    
    Command::new("xdg-open")
        .arg(&url)
        .spawn()?;

    Ok(())
}
