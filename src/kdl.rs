use kdl::{FormatConfigBuilder, KdlDocument};
use miette::{bail, Result};

#[derive(PartialEq, Eq, Debug)]
enum KdlVersion {
    V1,
    V2,
}

fn parse_document(input: &str, from_v1: bool, from_v2: bool) -> Result<(KdlDocument, KdlVersion)> {
    if from_v1 {
        let doc = KdlDocument::parse_v1(input)?;

        return Ok((doc, KdlVersion::V1));
    }

    if from_v2 {
        let doc = KdlDocument::parse_v2(input)?;

        return Ok((doc, KdlVersion::V2));
    }

    // auto detect
    let mut version: KdlVersion = KdlVersion::V2;
    let doc = match KdlDocument::parse_v1(input).is_ok() {
        true => {
            version = KdlVersion::V1;
            KdlDocument::parse_v1(input)?
        }
        false => KdlDocument::parse_v2(input)?,
    };

    Ok((doc, version))
}

pub struct FormatOptions {
    // input format
    pub from_v1: bool,
    pub from_v2: bool,

    // output format
    pub to_v1: bool,
    pub to_v2: bool,

    // dry-run for validation only
    pub no_format: bool,

    // further formatting options
    pub strip_comments: bool,
    pub indent_level: Option<usize>,
}

pub fn format_document(input: &str, options: &FormatOptions) -> Result<String> {
    if options.to_v1 && options.to_v2 {
        bail!("Cannot output in v1 and v2 spec at the same time");
    }

    let parser_result = parse_document(input, options.from_v1, options.from_v2)?;
    let mut doc = parser_result.0;
    let version = parser_result.1;

    tracing::debug!("{version:?}");

    let indent_level = match options.indent_level {
        Some(level) => " ".repeat(level),
        None => " ".repeat(4),
    };

    let fmt_config = FormatConfigBuilder::new()
        .no_comments(options.strip_comments)
        .indent(&indent_level);

    if !options.no_format {
        doc.autoformat_config(&fmt_config.build());
    }

    if (version == KdlVersion::V2 && options.to_v1) || (version == KdlVersion::V1 && !options.to_v2)
    {
        tracing::debug!("ensure_v1");
        doc.ensure_v1();
    }

    if (version == KdlVersion::V1 && options.to_v2) || (version == KdlVersion::V2 && !options.to_v1)
    {
        tracing::debug!("ensure_v2");
        doc.ensure_v2();
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
        false,
        false,
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
        false,
        false,
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
        false,
        true,
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
        true,
        false,
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
        #[case] v1: bool,
        #[case] v2: bool,
        #[case] exp: &str,
    ) {
        let res = format_document(
            input,
            &FormatOptions {
                from_v1: false,
                from_v2: false,
                to_v1: v1,
                to_v2: v2,
                no_format: false,
                strip_comments: false,
                indent_level: None,
            },
        );

        tracing::debug!("{res:?}");
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res, exp);
    }
}
