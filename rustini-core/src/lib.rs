#![warn(clippy::all, clippy::pedantic)]

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
