use rustini_core::{anyhow, ToIni};
use rustini_derive::ToIni;

#[derive(ToIni, PartialEq, Debug)]
struct ConvertMeStruct {
    a: String,
    b: Option<f64>,
    c: Option<usize>,
}

#[test]
fn test_derive_to_ini() -> anyhow::Result<()> {
    let data = ConvertMeStruct {
        a: "Hello, world!".to_string(),
        b: Some(1.23),
        c: None,
    };
    // TODO: preserve order of fields
    let ini_str = data.to_ini();
    let mut ini_lines = ini_str.lines().collect::<Vec<_>>();
    // since field names are at the start of a line, we can sort by them easily
    ini_lines.sort();
    // raw strings add a lot of extra spaces, this is a workaround
    let expected = ["a = Hello, world!", "b = 1.23"];
    assert_eq!(ini_lines, expected);
    Ok(())
}

#[derive(PartialEq, Eq, Debug)]
enum ConvertMeEnum {
    A,
}

impl ToIni for ConvertMeEnum {
    fn to_ini(&self) -> String
    where
        Self: Sized,
    {
        match self {
            ConvertMeEnum::A => "A",
        }
        .to_string()
    }
}

#[test]
fn test_impl_to_ini_enum() -> anyhow::Result<()> {
    let data = ConvertMeEnum::A;
    let ini_str = data.to_ini();
    let expected = "A";
    assert_eq!(ini_str, expected);
    Ok(())
}
