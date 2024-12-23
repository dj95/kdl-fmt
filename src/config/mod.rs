use clap::Parser;
use miette::{bail, IntoDiagnostic, Result};

use super::kdl::KdlVersion;

mod args;
mod file;

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
    pub fn new(args: &args::Args) -> Result<Self> {
        let mut conf = Self::from_config_file_or_default()?;
        conf.merge_args(args);
        Ok(conf)
    }

    fn from_config_file_or_default() -> Result<Self> {
        let mut conf = Config::default();

        if let Some(config_file_config) = file::get_config_file()? {
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

    fn merge_args(&mut self, args: &args::Args) -> &mut Self {
        if args.from_v1 || args.from_v2 {
            self.assume_format = args::get_version_from_flags(args.from_v1, args.from_v2);
        }

        if args.to_v1 || args.to_v2 {
            self.ensure_format = args::get_version_from_flags(args.to_v1, args.to_v2);
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
    let args = args::Args::parse();

    if args.to_v1 && args.to_v2 {
        bail!("Cannot output in v1 and v2 spec at the same time");
    }

    if args.input.is_stdin() && args.in_place {
        bail!("Cannot format STDIN with --in-place, due to missing filename");
    }

    let mut conf = Config::new(&args)?;

    conf.content = args.input.clone().contents().into_diagnostic()?;
    conf.filename = match args.input.is_file() {
        true => Some(args.input.filename().to_owned()),
        false => None,
    };

    Ok(conf)
}
