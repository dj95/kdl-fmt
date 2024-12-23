use crate::kdl::KdlVersion;
use kdl::KdlDocument;
use miette::{IntoDiagnostic, Result};
use std::{fs, path::Path};

pub struct ConfigFileConfig {
    pub assume_version: Option<KdlVersion>,
    pub ensure_version: Option<KdlVersion>,
    pub strip_comments: Option<bool>,
    pub indent_level: Option<usize>,
}

macro_rules! kdl_first_entry_as_string {
    ( $node:expr ) => {
        $node
            .entries()
            .iter()
            .next()
            .and_then(|s| s.value().as_string())
    };
}

macro_rules! kdl_first_entry_as_bool {
    ( $node:expr ) => {
        $node
            .entries()
            .iter()
            .next()
            .and_then(|s| s.value().as_bool())
    };
}

macro_rules! kdl_first_entry_as_integer {
    ( $node:expr ) => {
        $node
            .entries()
            .iter()
            .next()
            .and_then(|s| s.value().as_integer())
    };
}

fn get_kdl_version_from_node(doc: &KdlDocument, node_name: &str) -> Option<KdlVersion> {
    match doc.get(node_name) {
        Some(version) => match kdl_first_entry_as_string!(version) {
            Some(version) => match version {
                "v1" => Some(KdlVersion::V1),
                "v2" => Some(KdlVersion::V2),
                _ => None,
            },
            None => None,
        },
        None => None,
    }
}

pub fn get_config_file() -> Result<Option<ConfigFileConfig>> {
    if !Path::new(".kdl-fmt.kdl").exists() {
        return Ok(None);
    }

    let content = fs::read_to_string(".kdl-fmt.kdl").into_diagnostic()?;

    let doc = KdlDocument::parse_v2(&content)?;

    let indent_level = match doc.get("indent_level") {
        Some(level) => kdl_first_entry_as_integer!(level).map(|strip| strip as usize),
        None => None,
    };

    let strip_comments = match doc.get("strip_comments") {
        Some(strip) => kdl_first_entry_as_bool!(strip).map(|strip| strip),
        None => None,
    };

    Ok(Some(ConfigFileConfig {
        assume_version: get_kdl_version_from_node(&doc, "assume_version"),
        ensure_version: get_kdl_version_from_node(&doc, "ensure_version"),
        indent_level,
        strip_comments,
    }))
}
