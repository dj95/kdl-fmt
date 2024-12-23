use clap::Parser;
use clap_stdin::FileOrStdin;
use miette::{bail, IntoDiagnostic, Result};

use super::kdl::KdlVersion;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
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

pub fn get_config() -> Result<Config> {
    let args = Args::parse();

    if args.to_v1 && args.to_v2 {
        bail!("Cannot output in v1 and v2 spec at the same time");
    }

    if args.input.is_stdin() && args.in_place {
        bail!("Cannot format STDIN with --in-place, due to missing filename");
    }

    let content = args.input.clone().contents().into_diagnostic()?;

    let filename = match args.input.is_file() {
        true => Some(args.input.filename().to_owned()),
        false => None,
    };

    Ok(Config {
        assume_format: get_version_from_flags(args.from_v1, args.from_v2),
        ensure_format: get_version_from_flags(args.to_v1, args.to_v2),
        content,
        filename,
        strip_comments: args.strip_comments,
        indent_level: args.indent_level.unwrap_or(4),
        no_format: args.no_format,
        in_place: args.in_place,
    })
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
