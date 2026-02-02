use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct NoogleResponse {
    pub data: Vec<Doc>,
    #[serde(rename = "builtinTypes")]
    pub builtin_types: HashMap<String, serde_json::Value>,
    #[serde(rename = "upstreamInfo")]
    pub upstream_info: RepoInfo,
    #[serde(rename = "nixInfo")]
    pub nix_info: RepoInfo,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Doc {
    pub meta: DocMeta,
    pub content: Option<Content>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DocMeta {
    pub title: String,
    pub path: Vec<String>,
    #[serde(default)]
    pub aliases: Option<Vec<Vec<String>>>,
    pub signature: Option<String>,
    pub is_primop: bool,
    pub primop_meta: Option<serde_json::Value>,
    pub is_functor: Option<bool>,
    pub attr_position: Option<Position>,
    pub attr_expr: Option<String>,
    pub lambda_position: Option<Position>,
    pub lambda_expr: Option<String>,
    pub count_applied: Option<u32>,
    pub content_meta: Option<ContentMeta>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Content {
    pub content: Option<String>,
    pub source: Option<SourceInfo>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SourceInfo {
    pub position: Option<Position>,
    pub path: Vec<String>,
    #[serde(default)]
    pub pos_type: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Position {
    pub file: String,
    pub line: u32,
    pub column: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ContentMeta {
    pub position: Option<Position>,
    pub path: Vec<String>,
    #[serde(default)]
    pub pos_type: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RepoInfo {
    pub rev: String,
    #[serde(rename = "lastModified")]
    pub last_modified: u64,
}

impl Doc {
    pub fn all_names(&self) -> Vec<String> {
        let mut names = vec![self.meta.title.clone()];
        if let Some(aliases) = &self.meta.aliases {
            for alias in aliases {
                names.push(alias.join("."));
            }
        }
        names
    }

    pub fn matches_name(&self, name: &str) -> bool {
        if self.meta.title == name {
            return true;
        }

        if let Some(aliases) = &self.meta.aliases {
            aliases.iter().any(|alias| alias.join(".") == name)
        } else {
            false
        }
    }
}
