use std::{fs, path::Path};

use clap::Parser;
use clap_stdin::FileOrStdin;
use kdl::KdlDocument;
use miette::{bail, IntoDiagnostic, Result};

use super::kdl::KdlVersion;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(
        long,
        default_value_t = false,
        help = "Output the KDL document as v1 spec"
    )]
    to_v1: bool,
    #[arg(
        long,
        default_value_t = false,
        help = "Output the KDL document as v2 spec"
    )]
    to_v2: bool,

    #[arg(
        long,
        default_value_t = false,
        help = "Format a given file in place, instead of printing the formatted document"
    )]
    in_place: bool,

    #[arg(short, long, default_value_t = false, help = "Remove all comments")]
    strip_comments: bool,

    #[arg(short, long, help = "Number of spaces to indent")]
    indent_level: Option<usize>,

    #[arg(
        short,
        long,
        default_value_t = false,
        help = "Just parse and validate the document"
    )]
    no_format: bool,

    #[arg(long, default_value_t = false, help = "Force to use the v1 parser")]
    from_v1: bool,
    #[arg(long, default_value_t = false, help = "Force to use the v2 parser")]
    from_v2: bool,

    #[arg(help = "Filename or - to fetch input from STDIN")]
    input: FileOrStdin,
}

#[derive(Default, Clone)]
pub struct Config {
    // version constraints
    pub assume_format: Option<KdlVersion>,
    pub ensure_format: Option<KdlVersion>,

    // input
    pub content: String,
    pub filename: Option<String>,

    // format config
    pub strip_comments: bool,
    pub indent_level: usize,

    // misc. config
    pub no_format: bool,
    pub in_place: bool,
}

impl Config {
    pub fn from_config_file_or_default() -> Result<Self> {
        let mut conf = Config::default();

        if let Some(config_file_config) = get_config_file()? {
            conf.assume_format = config_file_config.assume_version;
            conf.ensure_format = config_file_config.ensure_version;

            if let Some(indent_level) = config_file_config.indent_level {
                conf.indent_level = indent_level;
            }

            if let Some(strip_comments) = config_file_config.strip_comments {
                conf.strip_comments = strip_comments;
            }
        }

        Ok(conf)
    }

    pub fn merge_args(&mut self, args: &Args) -> &mut Self {
        if args.from_v1 || args.from_v2 {
            self.assume_format = get_version_from_flags(args.from_v1, args.from_v2);
        }

        if args.to_v1 || args.to_v2 {
            self.ensure_format = get_version_from_flags(args.to_v1, args.to_v2);
        }

        if args.strip_comments {
            self.strip_comments = true;
        }

        if let Some(level) = args.indent_level {
            self.indent_level = level;
        }

        self.no_format = args.no_format;
        self.in_place = args.in_place;
        self
    }
}

pub fn get_config() -> Result<Config> {
    let args = Args::parse();

    if args.to_v1 && args.to_v2 {
        bail!("Cannot output in v1 and v2 spec at the same time");
    }

    if args.input.is_stdin() && args.in_place {
        bail!("Cannot format STDIN with --in-place, due to missing filename");
    }

    let mut conf = Config::from_config_file_or_default()?;
    conf.merge_args(&args);

    conf.content = args.input.clone().contents().into_diagnostic()?;
    conf.filename = match args.input.is_file() {
        true => Some(args.input.filename().to_owned()),
        false => None,
    };

    Ok(conf)
}

fn get_version_from_flags(v1: bool, v2: bool) -> Option<KdlVersion> {
    if v1 {
        return Some(KdlVersion::V1);
    }

    if v2 {
        return Some(KdlVersion::V2);
    }

    None
}

struct ConfigFileConfig {
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

fn get_config_file() -> Result<Option<ConfigFileConfig>> {
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
