#![warn(clippy::all, clippy::pedantic)]

use std::{collections::HashMap, str::FromStr};

pub use anyhow;
pub use thiserror;

/// Converts an INI string into a Rust struct.
pub trait FromIni<T> {
    type Error;

    /// Converts an INI string into a Rust struct.
    ///
    /// # Arguments
    ///
    /// * `ini` - The INI string to convert.
    ///
    /// # Errors
    ///
    /// Returns an error if the INI string is malformed or cannot be mapped
    /// to the Rust struct.
    fn from_ini<S>(ini: S) -> Result<T, Self::Error>
    where
        S: AsRef<str>;
}

/// Converts a Rust struct into an INI string. All members of the struct must also implement
/// `ToIni` for this to work.
pub trait ToIni {
    fn to_ini(&self) -> String
    where
        Self: Sized;
}

// TODO: make this a derive macro (there's a good guide online)
#[allow(clippy::implicit_hasher)]
impl FromIni<HashMap<String, String>> for HashMap<String, String> {
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
                let value = parts.next().map(|s| s.trim().to_string()).and_then(|s| {
                    if s.is_empty() {
                        None
                    } else {
                        Some(s)
                    }
                });
                Ok((key, value))
            })
            .filter_map(|pair| match pair {
                Ok((k, Some(v))) => Some(Ok((k.to_string(), v))),
                Err(e) => Some(Err(e)),
                _ => None,
            })
            .collect()
    }
}

#[allow(clippy::implicit_hasher)]
impl ToIni for HashMap<String, String> {
    fn to_ini(&self) -> String {
        self.iter()
            .map(|(k, v)| format!("{k} = {v}"))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

macro_rules! from_to_ini_impl {
    ($($t:ty)*) => ($(
        impl FromIni<$t> for $t where $t: FromStr {
            type Error = anyhow::Error;

            fn from_ini<S>(ini: S) -> Result<$t, Self::Error>
            where
                S: AsRef<str>,
            {
                ini.as_ref().parse::<$t>().map_err(|e| anyhow::anyhow!(e))
            }
        }

        impl ToIni for $t where $t: ToString {
            fn to_ini(&self) -> String {
                self.to_string()
            }
        }
    )*)
}

from_to_ini_impl!(bool i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize f32 f64 String);
