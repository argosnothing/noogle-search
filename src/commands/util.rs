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

use crate::data::{Doc, NoogleResponse};
use anyhow::{Result, anyhow};

pub fn parse_input(input: &str) -> (&str, Option<&str>) {
    if input.contains('\t') {
        let parts: Vec<&str> = input.split('\t').collect();
        (parts[0], Some(parts[1]))
    } else {
        (input, None)
    }
}

pub fn find_doc<'a>(response: &'a NoogleResponse, input: &str) -> Result<&'a Doc> {
    let (name, filter) = parse_input(input);
    
    let full_name = if let Some(ns) = filter {
        format!("{}.{}", ns, name)
    } else {
        name.to_string()
    };

    response
        .data
        .iter()
        .find(|d| d.meta.title == full_name)
        .or_else(|| response.data.iter().find(|d| d.matches_name(&full_name)))
        .ok_or_else(|| anyhow!("Function '{}' not found", full_name))
}
