use rustini_core::{anyhow, IniStruct};
use rustini_derive::IniStruct;

#[derive(IniStruct, PartialEq, Eq, Debug)]
struct ConvertMe {
    a: String,
    b: bool,
    c: u32,
}

#[derive(IniStruct, PartialEq, Eq, Debug)]
struct OptionalConvertMe {
    a: String,
    b: Option<bool>,
    c: Option<u32>,
}

#[test]
fn test_derive_from_ini() -> anyhow::Result<()> {
    let ini_str = r#"
        a = Hello, world!
        b = true
        c = 42
    "#;
    let data = ConvertMe::from_ini(ini_str)?;
    let expected = ConvertMe {
        a: "Hello, world!".to_string(),
        b: true,
        c: 42,
    };
    assert_eq!(data, expected);
    Ok(())
}

#[test]
fn test_derive_optional_from_ini() -> anyhow::Result<()> {
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
    let data = ConvertMe {
        a: "Hello, world!".to_string(),
        b: true,
        c: 42,
    };
    let ini_str = data.to_ini();
    // raw strings add a lot of extra space, this is a workaround
    let expected = ["a = Hello, world!", "b = true", "c = 42"].join("\n");
    assert_eq!(ini_str, expected);
    Ok(())
}

#[test]
fn test_derive_optional_to_ini() -> anyhow::Result<()> {
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
