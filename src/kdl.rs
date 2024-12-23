use kdl::{FormatConfigBuilder, KdlDocument};
use miette::Result;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum KdlVersion {
    V1,
    V2,
}

fn parse_document(
    input: &str,
    assume_version: &Option<KdlVersion>,
) -> Result<(KdlDocument, KdlVersion)> {
    let version = match assume_version {
        Some(version) => *version,
        None => match KdlDocument::parse_v1(input) {
            Ok(_) => KdlVersion::V1,
            Err(_) => KdlVersion::V2,
        },
    };

    match version {
        KdlVersion::V1 => Ok((KdlDocument::parse_v1(input)?, KdlVersion::V1)),
        KdlVersion::V2 => Ok((KdlDocument::parse_v2(input)?, KdlVersion::V2)),
    }
}

pub struct FormatOptions {
    // input format
    pub assume_version: Option<KdlVersion>,
    pub ensure_version: Option<KdlVersion>,

    // dry-run for validation only
    pub no_format: bool,

    // further formatting options
    pub strip_comments: bool,
    pub indent_level: usize,
}

pub fn format_document(input: &str, options: &FormatOptions) -> Result<String> {
    let parser_result = parse_document(input, &options.assume_version)?;
    let mut doc = parser_result.0;
    let version = parser_result.1;

    tracing::debug!("{version:?}");

    let indent_level = " ".repeat(options.indent_level);

    let fmt_config = FormatConfigBuilder::new()
        .no_comments(options.strip_comments)
        .indent(&indent_level);

    if !options.no_format {
        doc.autoformat_config(&fmt_config.build());
    }

    match options.ensure_version {
        Some(KdlVersion::V1) => {
            doc.ensure_v1();
        }
        Some(KdlVersion::V2) => {
            doc.ensure_v2();
        }
        _ => match version {
            // use the detected format, if no conversion is required
            KdlVersion::V1 => doc.ensure_v1(),
            KdlVersion::V2 => doc.ensure_v2(),
        },
    }

    Ok(doc.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(
        r#"
world prop="value" {
child 1
child 2
}"#,
        None,
        r#"world prop="value" {
    child 1
    child 2
}
"#
    )]
    #[case(
        r#"
world prop="value" {
child 1
// some comment
child 2
}"#,
        None,
        r#"world prop="value" {
    child 1
    // some comment
    child 2
}
"#
    )]
    #[case(
        r#"
world prop="value" {
child null
// some comment
child 2
}"#,
        Some(KdlVersion::V2),
        r#"world prop=value {
    child #null
    // some comment
    child 2
}
"#
    )]
    #[case(
        r#"
world prop="value" {
child #null
// some comment
child #true
}"#,
        Some(KdlVersion::V1),
        r#"world prop="value" {
    child null
    // some comment
    child true
}
"#
    )]
    #[test_log::test]
    fn test_format_document(
        #[case] input: &str,
        #[case] ensure_version: Option<KdlVersion>,
        #[case] exp: &str,
    ) {
        let res = format_document(
            input,
            &FormatOptions {
                ensure_version,
                assume_version: None,
                no_format: false,
                strip_comments: false,
                indent_level: 4,
            },
        );

        tracing::debug!("{res:?}");
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res, exp);
    }
}
