use std::fs;

use clap::Parser;
use clap_stdin::FileOrStdin;
use kdl_fmt::kdl::{self, FormatOptions};
use miette::{bail, IntoDiagnostic, Result};
use tracing_subscriber::EnvFilter;

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

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let args = Args::parse();

    if args.to_v1 && args.to_v2 {
        bail!("Cannot output in v1 and v2 spec at the same time");
    }

    if args.input.is_stdin() && args.in_place {
        bail!("Cannot format STDIN with --in-place, due to missing filename");
    }

    let content = &args.input.clone().contents().into_diagnostic()?;

    let formatted_content = kdl::format_document(
        &content,
        &FormatOptions {
            from_v1: args.from_v1,
            from_v2: args.from_v2,
            to_v1: args.to_v1,
            to_v2: args.to_v2,
            no_format: args.no_format,
            strip_comments: args.strip_comments,
            indent_level: args.indent_level,
        },
    )?;

    if args.input.is_file() && args.in_place {
        fs::write(args.input.filename(), formatted_content).into_diagnostic()?;
    } else {
        print!("{formatted_content}");
    }

    Ok(())
}
