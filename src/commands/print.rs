// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 argos_nothing

use crate::data::NoogleResponse;

pub fn execute(response: &NoogleResponse, filter: Option<&str>) {
    for doc in &response.data {
        if let Some(namespace) = filter {
            let prefix = format!("{}.", namespace);
            if let Some(stripped) = doc.meta.title.strip_prefix(&prefix) {
                println!("{}\t{}", stripped, namespace);
            }
        } else {
            for name in doc.all_names() {
                println!("{}", name);
            }
        }
    }
}
