#![warn(clippy::all, clippy::pedantic)]

use std::{collections::HashMap, fmt::Display, str::FromStr};

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

/// Represents a value in an INI file. Idea credit to `serde_json`.
#[derive(Debug, PartialEq, Clone, PartialOrd)]
pub enum IniValue {
    Bool(bool),
    Float(f64),
    PosInt(u64),
    NegInt(i64),
    String(String),
}

#[derive(Debug, thiserror::Error)]
pub enum IniValueError {
    #[error("failed to parse boolean: {0}")]
    BadBooleanFormat(#[from] std::str::ParseBoolError),
    #[error("misquoted string: {0}")]
    MisquotedString(String),
    #[error("tried to parse wrong type for enum variant: {0}")]
    WrongType(IniValue),
}

// TODO: consider making these TryFrom impls a derive macro iterating over the variants
impl TryFrom<IniValue> for bool {
    type Error = IniValueError;

    fn try_from(value: IniValue) -> Result<Self, Self::Error> {
        match value {
            IniValue::Bool(b) => Ok(b),
            _ => Err(IniValueError::WrongType(value)),
        }
    }
}

impl TryFrom<IniValue> for f64 {
    type Error = IniValueError;

    fn try_from(value: IniValue) -> Result<Self, Self::Error> {
        match value {
            IniValue::Float(f) => Ok(f),
            _ => Err(IniValueError::WrongType(value)),
        }
    }
}

impl TryFrom<IniValue> for u64 {
    type Error = IniValueError;

    fn try_from(value: IniValue) -> Result<Self, Self::Error> {
        match value {
            IniValue::PosInt(i) => Ok(i),
            _ => Err(IniValueError::WrongType(value)),
        }
    }
}

impl TryFrom<IniValue> for i64 {
    type Error = IniValueError;

    fn try_from(value: IniValue) -> Result<Self, Self::Error> {
        match value {
            IniValue::NegInt(i) => Ok(i),
            _ => Err(IniValueError::WrongType(value)),
        }
    }
}

impl TryFrom<IniValue> for String {
    type Error = IniValueError;

    fn try_from(value: IniValue) -> Result<Self, Self::Error> {
        match value {
            IniValue::String(s) => Ok(s),
            _ => Err(IniValueError::WrongType(value)),
        }
    }
}

impl FromStr for IniValue {
    // TODO: custom error type with thiserror
    type Err = IniValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let chars = s.chars().flat_map(|c| c.to_lowercase()).collect::<Vec<_>>();
        match chars[..] {
            ['"', .., '"'] | ['\'', .., '\''] => {
                let s = &s[1..s.len() - 1];
                Ok(IniValue::String(s.to_string()))
            }
            ['"', ..] | ['\'', ..] | [.., '"'] | [.., '\''] => {
                // TODO: custom error type with thiserror
                Err(IniValueError::MisquotedString(s.to_string()))
            }
            ['t', 'r', 'u', 'e'] | ['f', 'a', 'l', 's', 'e'] => s
                .parse::<bool>()
                .map(|b| IniValue::Bool(b))
                .map_err(Into::into),
            _ => {
                if let Ok(f) = s.parse::<f64>() {
                    if f.is_normal() && f.fract() == 0.0 {
                        if f.is_sign_positive() {
                            Ok(IniValue::PosInt(f as u64))
                        } else {
                            Ok(IniValue::NegInt(f as i64))
                        }
                    } else {
                        Ok(IniValue::Float(f))
                    }
                } else {
                    Ok(IniValue::String(s.to_string()))
                }
            }
        }
    }
}

impl Display for IniValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // just write the wrapped value no matter the branch
        match self {
            IniValue::String(s) => write!(f, "{s}"),
            IniValue::Bool(b) => write!(f, "{b}"),
            IniValue::PosInt(i) => write!(f, "{i}"),
            IniValue::NegInt(i) => write!(f, "{i}"),
            IniValue::Float(ff) => write!(f, "{ff}"),
        }
    }
}
