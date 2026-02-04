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

use anyhow::{Context, Result};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

use crate::data::NoogleResponse;

const API_URL: &str = "https://noogle.dev/api/v1/data";
const CACHE_DIR_NAME: &str = "noogle-search";
const DATA_FILE: &str = "data.json";
const METADATA_FILE: &str = "metadata.json";
const DISCLAIMER_FLAG: &str = ".disclaimer_shown";
const TTL_HOURS: i64 = 24;

#[derive(Debug, Serialize, Deserialize)]
struct Metadata {
    last_fetched: DateTime<Utc>,
    data_version: String,
}

pub fn load_data() -> Result<NoogleResponse> {
    let cache_dir = get_cache_dir()?;
    fs::create_dir_all(&cache_dir)?;

    let data_path = cache_dir.join(DATA_FILE);
    let metadata_path = cache_dir.join(METADATA_FILE);

    let needs_fetch = if metadata_path.exists() {
        let metadata_content = fs::read_to_string(&metadata_path)?;
        let metadata: Metadata = serde_json::from_str(&metadata_content)?;

        let age = Utc::now() - metadata.last_fetched;
        age > Duration::hours(TTL_HOURS)
    } else {
        true
    };

    if needs_fetch || !data_path.exists() {
        fetch_and_cache(&data_path, &metadata_path)?;
    }

    let data_content = fs::read_to_string(&data_path).context("Failed to read cached data")?;

    let response: NoogleResponse =
        serde_json::from_str(&data_content).context("Failed to parse cached data")?;

    Ok(response)
}

fn fetch_and_cache(data_path: &PathBuf, metadata_path: &PathBuf) -> Result<()> {
    let response = reqwest::blocking::get(API_URL).context("Failed to fetch from Noogle API")?;

    let body = response.text().context("Failed to read API response")?;

    fs::write(data_path, &body).context("Failed to write data cache")?;

    let response_data: NoogleResponse = serde_json::from_str(&body)?;

    let metadata = Metadata {
        last_fetched: Utc::now(),
        data_version: response_data.upstream_info.rev.clone(),
    };

    let metadata_json = serde_json::to_string_pretty(&metadata)?;
    fs::write(metadata_path, metadata_json).context("Failed to write metadata")?;

    Ok(())
}

fn get_cache_dir() -> Result<PathBuf> {
    let cache_base = dirs::cache_dir().context("Could not determine cache directory")?;
    Ok(cache_base.join(CACHE_DIR_NAME))
}

pub fn should_show_disclaimer() -> Result<bool> {
    let cache_dir = get_cache_dir()?;
    fs::create_dir_all(&cache_dir)?;
    let flag_path = cache_dir.join(DISCLAIMER_FLAG);
    Ok(!flag_path.exists())
}

pub fn mark_disclaimer_shown() -> Result<()> {
    let cache_dir = get_cache_dir()?;
    fs::create_dir_all(&cache_dir)?;
    let flag_path = cache_dir.join(DISCLAIMER_FLAG);
    fs::write(flag_path, "")?;
    Ok(())
}

