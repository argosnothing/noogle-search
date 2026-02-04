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
