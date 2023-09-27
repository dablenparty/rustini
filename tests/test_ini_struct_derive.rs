use rustini_core::{anyhow, FromIni, ToIni};
use rustini_derive::{FromIni, ToIni};

#[derive(FromIni, ToIni, PartialEq, Eq, Debug)]
struct OptionalConvertMe {
    a: String,
    b: Option<bool>,
    c: Option<u32>,
}

#[test]
fn test_derive_from_ini() -> anyhow::Result<()> {
    let ini_str = r#"
        a = Hello, world!
        c = 42
    "#;
    let data = OptionalConvertMe::from_ini(ini_str)?;
    let expected = OptionalConvertMe {
        a: "Hello, world!".to_string(),
        b: None,
        c: Some(42),
    };
    assert_eq!(data, expected);
    Ok(())
}

#[test]
fn test_derive_to_ini() -> anyhow::Result<()> {
    let data = OptionalConvertMe {
        a: "Hello, world!".to_string(),
        b: None,
        c: Some(42),
    };
    // TODO: preserve order of fields
    let ini_str = data.to_ini();
    let mut ini_lines = ini_str.lines().collect::<Vec<_>>();
    // since field names are at the start of a line, we can sort by them easily
    ini_lines.sort();
    // raw strings add a lot of extra spaces, this is a workaround
    let expected = ["a = Hello, world!", "c = 42"];
    assert_eq!(ini_lines, expected);
    Ok(())
}
