//! Zero-based static paths inside Lua expressions.
//!
//! A static path such as `rows.0.name` is no Lua: a name followed by a period
//! and a digit is a syntax error in every Lua program, so that each such
//! segment can be rewritten into a call without changing a valid expression.
use std::borrow::Cow;

/// The Lua function a numeric segment is rewritten to. An object is read by the
/// segment as a field name and an array by the segment as a zero-based index,
/// as the static path rules state.
pub(super) const INDEX_FUNCTION: &str = r#"
function __mote_static_index(value, segment)
  if type(value) == "table" then
    local field = rawget(value, segment)
    if field == nil then field = rawget(value, tonumber(segment) + 1) end
    if field ~= nil then return field end
  end
  error("cannot resolve zero-based path segment ." .. segment, 0)
end
"#;

const KEYWORDS: &[&str] = &[
    "and", "break", "do", "else", "elseif", "end", "false", "for", "function", "if", "in", "local",
    "nil", "not", "or", "repeat", "return", "then", "true", "until", "while",
];

pub(super) fn rewrite(source: &str) -> Cow<'_, str> {
    let bytes = source.as_bytes();
    let mut output = String::new();
    let mut copied = 0;
    let mut position = 0;
    // A name after a period or a colon is a field of the preceding expression
    // and cannot be the root of a path.
    let mut previous = b' ';
    while position < bytes.len() {
        let byte = bytes[position];
        if byte.is_ascii_whitespace() {
            position += 1;
            continue;
        }
        if matches!(byte, b'-' | b'[' | b'\'' | b'"')
            && let Some(end) = literal_end(source, position)
        {
            position = end;
            previous = b'"';
            continue;
        }
        if byte.is_ascii_digit() || (byte == b'.' && digit_at(bytes, position + 1)) {
            position = run_end(bytes, position, |byte| is_name_byte(byte) || byte == b'.');
            previous = b'0';
            continue;
        }
        if !is_name_byte(byte) {
            position += 1;
            previous = byte;
            continue;
        }
        let root_start = position;
        position = run_end(bytes, position, is_name_byte);
        let is_root =
            !matches!(previous, b'.' | b':') && !KEYWORDS.contains(&&source[root_start..position]);
        previous = b'a';
        if !is_root {
            continue;
        }
        let mut path = Cow::Borrowed(&source[root_start..position]);
        while position + 1 < bytes.len() && bytes[position] == b'.' {
            let segment_start = position + 1;
            let numeric = bytes[segment_start].is_ascii_digit();
            let segment_end = run_end(bytes, segment_start, |byte| {
                if numeric {
                    byte.is_ascii_digit()
                } else {
                    is_name_byte(byte)
                }
            });
            // A digit run that continues as a name, as in `rows.0x`, is left for
            // Lua to report.
            if segment_end == segment_start
                || (numeric && bytes.get(segment_end).copied().is_some_and(is_name_byte))
            {
                break;
            }
            let segment = &source[segment_start..segment_end];
            path = if numeric {
                Cow::Owned(format!("__mote_static_index({path}, \"{segment}\")"))
            } else {
                Cow::Owned(format!("{path}.{segment}"))
            };
            position = segment_end;
        }
        if let Cow::Owned(path) = path {
            output.push_str(&source[copied..root_start]);
            output.push_str(&path);
            copied = position;
        }
    }
    if copied == 0 {
        return Cow::Borrowed(source);
    }
    output.push_str(&source[copied..]);
    Cow::Owned(output)
}

fn is_name_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

fn digit_at(bytes: &[u8], position: usize) -> bool {
    bytes.get(position).is_some_and(u8::is_ascii_digit)
}

fn run_end(bytes: &[u8], start: usize, accepts: impl Fn(u8) -> bool) -> usize {
    start
        + bytes[start..]
            .iter()
            .take_while(|byte| accepts(**byte))
            .count()
}

/// The end of the string, long string or comment that begins at the position.
/// An unterminated literal extends to the end of the source, where Lua reports it.
fn literal_end(source: &str, start: usize) -> Option<usize> {
    let rest = &source[start..];
    let comment = rest.starts_with("--");
    let body = if comment { &rest[2..] } else { rest };
    let body_start = source.len() - body.len();
    if let Some(after_bracket) = body.strip_prefix('[') {
        let level = after_bracket.len() - after_bracket.trim_start_matches('=').len();
        if after_bracket[level..].starts_with('[') {
            let closing = format!("]{}]", "=".repeat(level));
            return Some(
                body.find(&closing)
                    .map_or(source.len(), |offset| body_start + offset + closing.len()),
            );
        }
    }
    if comment {
        return Some(
            body.find(['\r', '\n'])
                .map_or(source.len(), |offset| body_start + offset),
        );
    }
    let quote = rest
        .chars()
        .next()
        .filter(|quote| matches!(quote, '\'' | '"'))?;
    let mut escaped = false;
    for (offset, character) in rest.char_indices().skip(1) {
        if escaped {
            escaped = false;
        } else if character == '\\' {
            escaped = true;
        } else if character == quote {
            return Some(start + offset + 1);
        }
    }
    Some(source.len())
}

#[cfg(test)]
mod tests {
    use super::rewrite;

    #[test]
    fn numeric_segments_after_a_name_become_index_calls() {
        for (source, expected) in [
            ("rows.0.x", "__mote_static_index(rows, \"0\").x"),
            (
                "inch(report.rows.1.x) + 2.0",
                "inch(__mote_static_index(report.rows, \"1\").x) + 2.0",
            ),
            (
                "grid.0.12",
                "__mote_static_index(__mote_static_index(grid, \"0\"), \"12\")",
            ),
        ] {
            assert_eq!(rewrite(source), expected, "{source}");
        }
    }

    #[test]
    fn valid_lua_is_unchanged() {
        for source in [
            "total .. 0.5",
            "name..0",
            "first and.5 or 1.0.0",
            "'rows.0.x' .. \"rows.0\" .. [[rows.0]] -- rows.0",
            "0x1F + 1e5 + .5 + rows[1].x",
            "lookup(rows).0.x",
            "rows.first:method.0",
            "rows.0x",
        ] {
            assert_eq!(rewrite(source), source, "{source}");
        }
    }
}
