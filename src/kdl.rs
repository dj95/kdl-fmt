use kdl::{FormatConfigBuilder, KdlDocument};
use miette::{bail, Result};

#[derive(PartialEq, Eq, Debug)]
enum KdlVersion {
    V1,
    V2,
}

pub fn format_document(
    input: &str,
    strip_comments: bool,
    indent_level: Option<usize>,
    to_v1: bool,
    to_v2: bool,
) -> Result<String> {
    if to_v1 && to_v2 {
        bail!("Cannot output in v1 and v2 spec at the same time");
    }

    let mut version: KdlVersion = KdlVersion::V2;
    let mut doc = match KdlDocument::parse_v1(input).is_ok() {
        true => {
            version = KdlVersion::V1;
            KdlDocument::parse_v1(input)?
        }
        false => KdlDocument::parse_v2(input)?,
    };

    tracing::debug!("{version:?}");

    let indent_level = match indent_level {
        Some(level) => " ".repeat(level),
        None => " ".repeat(4),
    };

    let fmt_config = FormatConfigBuilder::new()
        .no_comments(strip_comments)
        .indent(&indent_level);

    doc.autoformat_config(&fmt_config.build());

    if (version == KdlVersion::V2 && to_v1) || (version == KdlVersion::V1 && !to_v2) {
        tracing::debug!("ensure_v1");
        doc.ensure_v1();
    }

    if (version == KdlVersion::V1 && to_v2) || (version == KdlVersion::V2 && !to_v1) {
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
        let res = format_document(input, false, None, v1, v2);

        tracing::debug!("{res:?}");
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res, exp);
    }
}
