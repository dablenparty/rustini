use rustini_core::{anyhow, IniStruct};
use rustini_derive::IniStruct;

#[derive(IniStruct)]
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
    assert_eq!(data.a, "Hello, world!");
    assert!(data.b);
    assert_eq!(data.c, 42);
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
    assert_eq!(
        ini_str,
        r#"a = Hello, world!
        b = true
        c = 42"#
            .trim()
    );
    Ok(())
}
