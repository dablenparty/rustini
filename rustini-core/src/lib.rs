#![warn(clippy::all, clippy::pedantic)]

use std::collections::HashMap;

pub use anyhow;
pub use thiserror;

/// Trait for converting between an INI file and a struct.
pub trait IniStruct<T> {
    type Error;

    /// Convert an INI string to a struct.
    fn from_ini<S>(ini: S) -> Result<T, Self::Error>
    where
        S: AsRef<str>;

    /// Convert this struct to an INI string.
    fn to_ini(&self) -> String;
}

// TODO: make this a derive macro (there's a good guide online)
impl IniStruct<HashMap<String, String>> for HashMap<String, String> {
    type Error = anyhow::Error;

    fn from_ini<S>(ini: S) -> Result<HashMap<String, String>, Self::Error>
    where
        S: AsRef<str>,
    {
        ini.as_ref()
            .lines()
            .map(|line| {
                let mut parts = line.splitn(2, '=');
                let key = parts.next().ok_or(anyhow::anyhow!("missing key"))?.trim();
                let value = parts.next().map(|s| s.trim().to_string());
                Ok((key, value))
            })
            .filter_map(|pair| match pair {
                Ok((k, Some(v))) => Some(Ok((k.to_string(), v))),
                Err(e) => Some(Err(e)),
                _ => None,
            })
            .collect::<Result<::std::collections::HashMap<_, _>, _>>()
    }

    fn to_ini(&self) -> String {
        self.iter()
            .map(|(k, v)| format!("{k} = {v}"))
            .collect::<Vec<_>>()
            .join("\n")
    }
}
