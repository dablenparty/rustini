use std::collections::HashMap;

use rustini_core::{anyhow, thiserror, IniStruct};

struct ConvertMe {
    a: String,
    b: bool,
    c: u32,
}

#[derive(Debug, thiserror::Error)]
enum ConvertMeError {
    #[error("missing key")]
    MissingKey,
    #[error("invalid value: {0}")]
    InvalidValue(String),
    #[error("missing value for key: {0}")]
    MissingValue(String),
}

impl IniStruct<ConvertMe> for ConvertMe {
    type Error = anyhow::Error;

    fn from_ini<S>(ini: S) -> Result<ConvertMe, Self::Error>
    where
        S: AsRef<str>,
    {
        // TODO: extract convenience methods for parsing values/pairs
        let ini = ini.as_ref();
        let pairs = ini
            .lines()
            .map(|line| {
                let mut parts = line.splitn(2, '=');
                let key = parts.next().ok_or(ConvertMeError::MissingKey)?.trim();
                let value = parts.next().map(|s| s.trim());
                Ok((key, value))
            })
            .collect::<Result<HashMap<_, _>, ConvertMeError>>()?;
        let a = pairs
            .get("a")
            .ok_or(ConvertMeError::MissingKey)?
            .ok_or(ConvertMeError::MissingValue("a".to_string()))?
            .to_string();
        let b = pairs.get("b").ok_or(ConvertMeError::MissingKey)?.map_or(
            Err(ConvertMeError::MissingValue("b".to_string())),
            |s| {
                s.parse::<bool>()
                    .map_err(|_| ConvertMeError::InvalidValue(format!("b: {}", s)))
            },
        )?;
        let c = pairs.get("c").ok_or(ConvertMeError::MissingKey)?.map_or(
            Err(ConvertMeError::MissingValue("c".to_string())),
            |s| {
                s.parse::<u32>()
                    .map_err(|_| ConvertMeError::InvalidValue(format!("c: {}", s)))
            },
        )?;
        Ok(ConvertMe { a, b, c })
    }

    fn to_ini(&self) -> String {
        let Self { a, b, c } = self;
        format!(
            r#"a = {a}
        b = {b}
        c = {c}"#
        )
        .trim()
        .to_string()
    }
}

#[test]
fn test_from_ini() -> anyhow::Result<()> {
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
fn test_to_ini() -> anyhow::Result<()> {
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
