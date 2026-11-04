//! The deterministic subset of Lua's string and math libraries exposed to scripts.
//!
//! The pattern functions (`find`, `match`, `gmatch`, `gsub`) are withheld because
//! they run inside C, where the instruction hook that enforces the time budget is
//! never called. `string.format` and `string.rep` are reimplemented so that their
//! output and cost do not depend on the platform's C library.
use mlua::{Lua, Table, Value, Variadic};

const STRING_FUNCTIONS: &[&str] = &["byte", "char", "len", "lower", "reverse", "sub", "upper"];
const WITHHELD_MATH: &[&str] = &["random", "randomseed"];
/// The largest string `string.rep` may build, in bytes.
const REPETITION_LIMIT: usize = 16 * 1024 * 1024;

pub(super) fn install(lua: &Lua) -> mlua::Result<()> {
    let globals = lua.globals();
    let loaded: Table = globals.get("string")?;
    let string = lua.create_table()?;
    for name in STRING_FUNCTIONS {
        string.set(*name, loaded.get::<Value>(*name)?)?;
    }
    string.set(
        "format",
        lua.create_function(
            |lua, (template, arguments): (mlua::LuaString, Variadic<Value>)| {
                let template = template.to_str()?;
                format(&template, &arguments)
                    .map_err(mlua::Error::RuntimeError)
                    .and_then(|text| lua.create_string(text))
            },
        )?,
    )?;
    string.set(
        "rep",
        lua.create_function(|lua, (text, count): (mlua::LuaString, f64)| {
            let bytes = text.as_bytes();
            let count = if count.is_finite() && count > 0.0 {
                count as usize
            } else {
                0
            };
            if bytes.len().saturating_mul(count) > REPETITION_LIMIT {
                return Err(mlua::Error::RuntimeError(
                    "string.rep result is too large".into(),
                ));
            }
            lua.create_string(bytes.repeat(count))
        })?,
    )?;
    globals.set("string", &string)?;
    let metatable = lua.create_table()?;
    metatable.set("__index", string)?;
    lua.set_type_metatable::<mlua::LuaString>(Some(metatable));
    let loaded: Table = globals.get("math")?;
    let math = lua.create_table()?;
    for pair in loaded.pairs::<String, Value>() {
        let (name, value) = pair?;
        if !WITHHELD_MATH.contains(&name.as_str()) {
            math.set(name, value)?;
        }
    }
    globals.set("math", math)
}

#[derive(Default)]
struct Specification {
    left: bool,
    zero: bool,
    plus: bool,
    space: bool,
    width: usize,
    precision: Option<usize>,
}

/// Formats as Lua 5.1's `string.format` does for the conversions `d`, `i`, `x`,
/// `X`, `e`, `E`, `f`, `g`, `G` and `s`, except that `%s` widths and precisions
/// count characters rather than bytes.
pub(super) fn format(template: &str, arguments: &[Value]) -> Result<String, String> {
    let mut output = String::new();
    let mut characters = template.chars().peekable();
    let mut next = 0;
    while let Some(character) = characters.next() {
        if character != '%' {
            output.push(character);
            continue;
        }
        if characters.next_if_eq(&'%').is_some() {
            output.push('%');
            continue;
        }
        let mut specification = Specification::default();
        let mut flags = 0;
        while let Some(flag) = characters.next_if(|flag| "-0+ #".contains(*flag)) {
            flags += 1;
            match flag {
                '-' => specification.left = true,
                '0' => specification.zero = true,
                '+' => specification.plus = true,
                ' ' => specification.space = true,
                _ => return Err("invalid format (the '#' flag is not supported)".into()),
            }
        }
        if flags > 5 {
            return Err("invalid format (repeated flags)".into());
        }
        specification.width = digits(&mut characters)?.unwrap_or(0);
        if characters.next_if_eq(&'.').is_some() {
            specification.precision = Some(digits(&mut characters)?.unwrap_or(0));
        }
        let conversion = characters.next().ok_or("invalid option '%' to 'format'")?;
        next += 1;
        let position = next + 1;
        let argument = arguments
            .get(next - 1)
            .ok_or_else(|| format!("bad argument #{position} to 'format' (no value)"))?;
        let (sign, body) = match conversion {
            'd' | 'i' | 'x' | 'X' => {
                let value = number(argument, position)?;
                if !value.is_finite() || value.abs() >= 9.2e18 {
                    return Err(format!(
                        "bad argument #{position} to 'format' (number has no integer representation)"
                    ));
                }
                let value = value.trunc() as i64;
                let digits = match conversion {
                    'x' | 'X' if value < 0 => {
                        return Err(format!(
                            "bad argument #{position} to 'format' (non-negative number expected)"
                        ));
                    }
                    'x' => format!("{value:x}"),
                    'X' => format!("{value:X}"),
                    _ => value.unsigned_abs().to_string(),
                };
                let digits = match specification.precision {
                    Some(precision) => format!("{digits:0>precision$}"),
                    None => digits,
                };
                (sign(&specification, value < 0), digits)
            }
            'e' | 'E' | 'f' | 'g' | 'G' => {
                let value = number(argument, position)?;
                let precision = specification.precision.unwrap_or(6);
                let mut body = if !value.is_finite() {
                    specification.zero = false;
                    if value.is_nan() { "nan" } else { "inf" }.to_owned()
                } else {
                    match conversion {
                        'f' => format!("{:.precision$}", value.abs()),
                        'e' | 'E' => exponential(value.abs(), precision),
                        _ => general(value.abs(), precision),
                    }
                };
                if conversion.is_ascii_uppercase() {
                    body = body.to_ascii_uppercase();
                }
                (
                    sign(&specification, value.is_sign_negative() && !value.is_nan()),
                    body,
                )
            }
            's' => {
                let text = match argument {
                    Value::String(text) => {
                        text.to_str().map_err(|error| error.to_string())?.to_owned()
                    }
                    Value::Integer(value) => value.to_string(),
                    Value::Number(value) => {
                        let text = general(value.abs(), 14);
                        if value.is_sign_negative() {
                            format!("-{text}")
                        } else {
                            text
                        }
                    }
                    other => {
                        return Err(format!(
                            "bad argument #{position} to 'format' (string expected, got {})",
                            other.type_name()
                        ));
                    }
                };
                specification.zero = false;
                let text = match specification.precision {
                    Some(precision) => text.chars().take(precision).collect(),
                    None => text,
                };
                ("", text)
            }
            other => return Err(format!("invalid option '%{other}' to 'format'")),
        };
        let length = sign.chars().count() + body.chars().count();
        let padding = specification.width.saturating_sub(length);
        if specification.left {
            output.push_str(sign);
            output.push_str(&body);
            output.extend(std::iter::repeat_n(' ', padding));
        } else if specification.zero
            && !(matches!(conversion, 'd' | 'i' | 'x' | 'X') && specification.precision.is_some())
        {
            output.push_str(sign);
            output.extend(std::iter::repeat_n('0', padding));
            output.push_str(&body);
        } else {
            output.extend(std::iter::repeat_n(' ', padding));
            output.push_str(sign);
            output.push_str(&body);
        }
    }
    Ok(output)
}

/// Reads a width or precision, which Lua limits to two digits.
fn digits(
    characters: &mut std::iter::Peekable<std::str::Chars<'_>>,
) -> Result<Option<usize>, String> {
    let mut value = None;
    let mut count = 0;
    while let Some(digit) = characters.next_if(char::is_ascii_digit) {
        count += 1;
        if count > 2 {
            return Err("invalid format (width or precision too long)".into());
        }
        value = Some(value.unwrap_or(0) * 10 + digit.to_digit(10).unwrap() as usize);
    }
    Ok(value)
}

fn sign(specification: &Specification, negative: bool) -> &'static str {
    if negative {
        "-"
    } else if specification.plus {
        "+"
    } else if specification.space {
        " "
    } else {
        ""
    }
}

/// Lua coerces numeric strings to numbers wherever a number is expected.
fn number(value: &Value, position: usize) -> Result<f64, String> {
    match value {
        Value::Integer(value) => Ok(*value as f64),
        Value::Number(value) => Ok(*value),
        Value::String(text) => text
            .to_str()
            .ok()
            .and_then(|text| text.trim().parse::<f64>().ok())
            .ok_or_else(|| {
                format!("bad argument #{position} to 'format' (number expected, got string)")
            }),
        other => Err(format!(
            "bad argument #{position} to 'format' (number expected, got {})",
            other.type_name()
        )),
    }
}

/// Writes a nonnegative finite value as C's `%e` does, with a signed exponent of
/// at least two digits.
fn exponential(value: f64, precision: usize) -> String {
    let text = format!("{value:.precision$e}");
    let (mantissa, exponent) = text.split_once('e').unwrap();
    let exponent: i32 = exponent.parse().unwrap();
    let sign = if exponent < 0 { '-' } else { '+' };
    format!("{mantissa}e{sign}{:02}", exponent.abs())
}

/// Writes a nonnegative finite value as C's `%g` does: fixed notation when the
/// decimal exponent lies in [-4, precision), exponential otherwise, without
/// trailing zeros.
fn general(value: f64, precision: usize) -> String {
    let precision = precision.max(1);
    let rounded = format!("{value:.*e}", precision - 1);
    let exponent: i32 = rounded.split_once('e').unwrap().1.parse().unwrap();
    if exponent >= -4 && exponent < precision as i32 {
        let decimals = (precision as i32 - 1 - exponent) as usize;
        trim_zeros(&format!("{value:.decimals$}")).to_owned()
    } else {
        let text = exponential(value, precision - 1);
        let (mantissa, exponent) = text.split_once('e').unwrap();
        format!("{}e{exponent}", trim_zeros(mantissa))
    }
}

fn trim_zeros(text: &str) -> &str {
    if text.contains('.') {
        text.trim_end_matches('0').trim_end_matches('.')
    } else {
        text
    }
}

#[cfg(test)]
mod tests {
    use super::format;
    use mlua::Value;

    fn formatted(template: &str, arguments: &[Value]) -> String {
        format(template, arguments).unwrap()
    }

    #[test]
    fn numeric_conversions_match_the_c_library() {
        let number = Value::Number;
        for (template, value, expected) in [
            ("%d", 42.9, "42"),
            ("%5d", -42.0, "  -42"),
            ("%-5d|", 42.0, "42   |"),
            ("%05d", -42.0, "-0042"),
            ("%+d", 7.0, "+7"),
            ("% d", 7.0, " 7"),
            ("%.3d", 7.0, "007"),
            ("%x", 255.0, "ff"),
            ("%04X", 255.0, "00FF"),
            ("%.2f", 2.675, "2.67"),
            ("%.1f", 171.456, "171.5"),
            ("%8.3f", -1.23456, "  -1.235"),
            ("%08.2f", -1.23456, "-0001.23"),
            ("%.0f", 0.5, "0"),
            ("%.0f", 1.5, "2"),
            ("%f", -0.0, "-0.000000"),
            ("%e", 12345.678, "1.234568e+04"),
            ("%.2E", 0.000123, "1.23E-04"),
            ("%g", 100000.0, "100000"),
            ("%g", 1000000.0, "1e+06"),
            ("%g", 0.0001, "0.0001"),
            ("%g", 0.00001, "1e-05"),
            ("%.3g", 1.23456, "1.23"),
            ("%g", 2.5, "2.5"),
            ("%G", 1.5e-10, "1.5E-10"),
            ("%f", f64::INFINITY, "inf"),
            ("%5.1f", f64::NEG_INFINITY, " -inf"),
            ("%G", f64::NAN, "NAN"),
        ] {
            assert_eq!(
                formatted(template, &[number(value)]),
                expected,
                "{template}"
            );
        }
    }

    #[test]
    fn strings_and_literals_follow_lua() {
        let lua = mlua::Lua::new();
        let text = Value::String(lua.create_string("Café").unwrap());
        assert_eq!(formatted("[%6s]", std::slice::from_ref(&text)), "[  Café]");
        assert_eq!(formatted("[%-6s]", std::slice::from_ref(&text)), "[Café  ]");
        assert_eq!(formatted("[%.3s]", std::slice::from_ref(&text)), "[Caf]");
        assert_eq!(formatted("%s", &[Value::Number(0.1 + 0.2)]), "0.3");
        assert_eq!(formatted("%s", &[Value::Number(1e15)]), "1e+15");
        assert_eq!(formatted("%s", &[Value::Integer(12)]), "12");
        assert_eq!(formatted("100%% of %d", &[Value::Number(3.0)]), "100% of 3");
        let numeric = Value::String(lua.create_string(" 12.5 ").unwrap());
        assert_eq!(formatted("%.2f", &[numeric]), "12.50");
    }

    #[test]
    fn unsupported_and_malformed_formats_are_errors() {
        for (template, arguments) in [
            ("%d", vec![]),
            ("%q", vec![Value::Number(1.0)]),
            ("%c", vec![Value::Number(65.0)]),
            ("%#x", vec![Value::Number(1.0)]),
            ("%100d", vec![Value::Number(1.0)]),
            ("%.100f", vec![Value::Number(1.0)]),
            ("%x", vec![Value::Number(-1.0)]),
            ("%d", vec![Value::Number(f64::INFINITY)]),
            ("%d", vec![Value::Boolean(true)]),
            ("%s", vec![Value::Nil]),
            ("100%", vec![]),
        ] {
            assert!(format(template, &arguments).is_err(), "{template}");
        }
    }
}
