use rustini_core::{anyhow, IniStruct};
use rustini_derive::IniStruct;

#[derive(IniStruct, PartialEq, Eq, Debug)]
struct ConvertMe {
    a: String,
    b: bool,
    c: u32,
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
