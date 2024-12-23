use clap::Parser;
use clap_stdin::FileOrStdin;

use crate::kdl::KdlVersion;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(
        long,
        default_value_t = false,
        help = "Output the KDL document as v1 spec"
    )]
    pub to_v1: bool,
    #[arg(
        long,
        default_value_t = false,
        help = "Output the KDL document as v2 spec"
    )]
    pub to_v2: bool,

    #[arg(
        long,
        default_value_t = false,
        help = "Format a given file in place, instead of printing the formatted document"
    )]
    pub in_place: bool,

    #[arg(short, long, default_value_t = false, help = "Remove all comments")]
    pub strip_comments: bool,

    #[arg(short, long, help = "Number of spaces to indent")]
    pub indent_level: Option<usize>,

    #[arg(
        short,
        long,
        default_value_t = false,
        help = "Just parse and validate the document"
    )]
    pub no_format: bool,

    #[arg(long, default_value_t = false, help = "Force to use the v1 parser")]
    pub from_v1: bool,
    #[arg(long, default_value_t = false, help = "Force to use the v2 parser")]
    pub from_v2: bool,

    #[arg(help = "Filename or - to fetch input from STDIN")]
    pub input: FileOrStdin,
}

pub fn get_version_from_flags(v1: bool, v2: bool) -> Option<KdlVersion> {
    if v1 {
        return Some(KdlVersion::V1);
    }

    if v2 {
        return Some(KdlVersion::V2);
    }

    None
}
