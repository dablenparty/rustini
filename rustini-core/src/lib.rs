#![warn(clippy::all, clippy::pedantic)]

use std::{fmt::Display, num::FpCategory, str::FromStr};

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

/// Represents a value in an INI file. Idea credit to `serde_json`.
#[derive(Debug, PartialEq, Clone, PartialOrd)]
pub enum IniValue {
    Bool(bool),
    Float(f64),
    PosInt(u64),
    NegInt(i64),
    String(String),
}

impl FromStr for IniValue {
    // TODO: custom error type with thiserror
    type Err = anyhow::Error;

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
                Err(anyhow::anyhow!("unterminated string"))
            }
            ['t', 'r', 'u', 'e'] | ['f', 'a', 'l', 's', 'e'] => s
                .parse::<bool>()
                .map(|b| IniValue::Bool(b))
                .map_err(Into::into),
            _ => {
                if let Ok(f) = s.parse::<f64>() {
                    if let FpCategory::Nan | FpCategory::Infinite = f.classify() {
                        todo!("custom error handling for NaN/inf");
                    }
                    if f.fract() == 0.0 {
                        if f >= 0.0 {
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
