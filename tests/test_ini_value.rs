use std::str::FromStr;

use rustini_core::{
    anyhow::{self, Context},
    IniValue,
};

#[test]
fn test_ini_value_bool() -> anyhow::Result<()> {
    let t_string = "true";
    let t = IniValue::from_str(t_string)
        .context(format!("failed to parse {t_string} into IniValue"))?;
    assert_eq!(t, IniValue::Bool(true));
    // implicitly tests case insensitivity
    // TODO: eventually, that will be an option
    let f_string = "false";
    let f = IniValue::from_str(f_string)
        .context(format!("failed to parse {f_string} into IniValue"))?;
    assert_eq!(f, IniValue::Bool(false));
    Ok(())
}

#[test]
fn test_ini_value_case_sensitive() {
    let t_string = "TrUe";
    let r = IniValue::from_str(t_string);
    assert!(
        r.is_err(),
        "expected case-sensitive parse to fail, got {:?}",
        r
    );
}

#[test]
fn test_ini_value_float() -> anyhow::Result<()> {
    let f_string = "1.2345";
    let f = IniValue::from_str(f_string)
        .context(format!("failed to parse {f_string} into IniValue"))?;
    assert_eq!(f, IniValue::Float(1.2345));
    Ok(())
}

#[test]
fn test_ini_value_pos_int() -> anyhow::Result<()> {
    let i_string = "12345";
    let i = IniValue::from_str(i_string)
        .context(format!("failed to parse {i_string} into IniValue"))?;
    assert_eq!(i, IniValue::PosInt(12345));
    Ok(())
}

#[test]
fn test_ini_value_neg_int() -> anyhow::Result<()> {
    let i_string = "-12345";
    let i = IniValue::from_str(i_string)
        .context(format!("failed to parse {i_string} into IniValue"))?;
    assert_eq!(i, IniValue::NegInt(-12345));
    Ok(())
}

#[test]
fn test_ini_value_unquoted_string() -> anyhow::Result<()> {
    let s_string = "Hello, world!";
    let s = IniValue::from_str(s_string)
        .context(format!("failed to parse {s_string} into IniValue"))?;
    assert_eq!(s, IniValue::String("Hello, world!".to_string()));
    Ok(())
}

#[test]
fn test_ini_value_quoted_string() -> anyhow::Result<()> {
    let s_string = r#""Hello, world!""#;
    let s = IniValue::from_str(s_string)
        .context(format!("failed to parse {s_string} into IniValue"))?;
    assert_eq!(s, IniValue::String("Hello, world!".to_string()));
    Ok(())
}

#[test]
fn test_ini_value_bad_quotes() {
    let right_quote = "Hello, world!\"";
    let right_res = IniValue::from_str(right_quote);
    assert!(
        right_res.is_err(),
        "expected right-quoted string to fail, got {:?}",
        right_res
    );

    let left_quote = "\"Hello, world!";
    let left_res = IniValue::from_str(left_quote);
    assert!(
        left_res.is_err(),
        "expected left-quoted string to fail, got {:?}",
        left_res
    );
}

#[test]
fn test_ini_value_bad_float() {
    let nan = "NaN";
    let nan_res = IniValue::from_str(nan);
    assert!(nan_res.is_err(), "expected NaN to fail, got {:?}", nan_res);

    let inf = "inf";
    let inf_res = IniValue::from_str(inf);
    assert!(
        inf_res.is_err(),
        "expected Infinity to fail, got {:?}",
        inf_res
    );
}
