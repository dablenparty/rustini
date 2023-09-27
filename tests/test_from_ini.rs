use rustini_core::{anyhow, FromIni};
use rustini_derive::FromIni;

#[derive(FromIni, PartialEq, Debug)]
struct ConvertMeStruct {
    a: String,
    b: Option<f64>,
    c: Option<usize>,
}

#[test]
fn test_derive_from_ini_struct() -> anyhow::Result<()> {
    let ini_str = r#"
        a = Hello, world!
        c = 42
    "#;
    let data = ConvertMeStruct::from_ini(ini_str)?;
    let expected = ConvertMeStruct {
        a: "Hello, world!".to_string(),
        b: None,
        c: Some(42),
    };
    assert_eq!(data, expected);
    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
enum ConvertMeEnum {
    A,
    B,
    C,
}

impl FromIni<ConvertMeEnum> for ConvertMeEnum {
    type Error = anyhow::Error;

    fn from_ini<S>(ini: S) -> Result<ConvertMeEnum, Self::Error>
    where
        S: AsRef<str>,
    {
        let ini = ini.as_ref();
        match ini {
            "A" => Ok(ConvertMeEnum::A),
            "B" => Ok(ConvertMeEnum::B),
            "C" => Ok(ConvertMeEnum::C),
            _ => Err(anyhow::anyhow!("invalid value for ConvertMe: {}", ini)),
        }
    }
}

#[test]
fn test_impl_from_ini_enum() -> anyhow::Result<()> {
    let ini_str = "A";
    let data = ConvertMeEnum::from_ini(ini_str)?;
    let expected = ConvertMeEnum::A;
    assert_eq!(data, expected);
    Ok(())
}

#[derive(FromIni, PartialEq, Debug)]
struct ConvertMeStructWithEnum {
    a: ConvertMeEnum,
}

#[test]
fn test_derive_nested_impl_from_ini() {
    let ini_str = r#"
        a = A
    "#;
    let data = ConvertMeStructWithEnum::from_ini(ini_str).unwrap();
    let expected = ConvertMeStructWithEnum {
        a: ConvertMeEnum::A,
    };
    assert_eq!(data, expected);
}
