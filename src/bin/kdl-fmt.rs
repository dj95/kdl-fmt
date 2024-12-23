use std::fs;

use kdl_fmt::{
    config,
    kdl::{self, FormatOptions},
};
use miette::{IntoDiagnostic, Result};
use tracing_subscriber::EnvFilter;

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let conf = config::get_config()?;

    let formatted_content = kdl::format_document(
        &conf.content,
        &FormatOptions {
            assume_version: conf.assume_format,
            ensure_version: conf.ensure_format,
            no_format: conf.no_format,
            strip_comments: conf.strip_comments,
            indent_level: conf.indent_level,
        },
    )?;

    if conf.in_place {
        if let Some(filename) = conf.filename {
            fs::write(filename, formatted_content).into_diagnostic()?;
        }
    } else {
        print!("{formatted_content}");
    }

    Ok(())
}
