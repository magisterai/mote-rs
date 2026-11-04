use serde_json::Value;

pub fn compare(expected: &Value, actual: &Value, path: &str, differences: &mut Vec<String>) {
    match (expected, actual) {
        (Value::Number(expected), Value::Number(actual))
            if (expected.as_f64().unwrap() - actual.as_f64().unwrap()).abs() <= 1e-9 => {}
        (Value::Array(expected), Value::Array(actual)) if expected.len() == actual.len() => {
            for (index, (expected, actual)) in expected.iter().zip(actual).enumerate() {
                compare(expected, actual, &format!("{path}[{index}]"), differences);
            }
        }
        (Value::Object(expected), Value::Object(actual)) if expected.keys().eq(actual.keys()) => {
            let name = expected.get("id").and_then(Value::as_str).unwrap_or(path);
            for (key, expected) in expected {
                compare(
                    expected,
                    &actual[key],
                    &format!("{name}.{key}"),
                    differences,
                );
            }
        }
        _ if expected == actual => {}
        (Value::Array(expected), Value::Array(actual)) => differences.push(format!(
            "{path}: expected {} items, actual {}",
            expected.len(),
            actual.len()
        )),
        _ => differences.push(format!(
            "{path}: expected {}, actual {}",
            abbreviated(expected),
            abbreviated(actual)
        )),
    }
}

fn abbreviated(value: &Value) -> String {
    let text = value.to_string();
    let mut characters = text.chars();
    let preview: String = characters.by_ref().take(300).collect();
    if characters.next().is_some() {
        format!("{preview}…")
    } else {
        preview
    }
}

/// Copies the stored text of a capture and replaces only the values that no
/// longer agree within the tolerance, so that a refreshed capture keeps its field
/// order and number spelling and differs only where the layout changed.
#[allow(dead_code)]
struct Rewriter<'text> {
    text: &'text str,
    position: usize,
    output: String,
}

#[allow(dead_code)]
impl Rewriter<'_> {
    fn whitespace(&mut self) {
        let rest = &self.text[self.position..];
        let length = rest.len() - rest.trim_start().len();
        self.output.push_str(&rest[..length]);
        self.position += length;
    }

    fn copy(&mut self, character: char) {
        assert!(self.text[self.position..].starts_with(character));
        self.position += 1;
        self.output.push(character);
    }

    /// Advances past the value that begins at the current position.
    fn skip(&mut self) -> &str {
        let start = self.position;
        let bytes = self.text.as_bytes();
        let mut depth = 0usize;
        let mut in_string = false;
        let mut escaped = false;
        while self.position < bytes.len() {
            let byte = bytes[self.position];
            self.position += 1;
            if in_string {
                if escaped {
                    escaped = false;
                } else if byte == b'\\' {
                    escaped = true;
                } else if byte == b'"' {
                    in_string = false;
                    if depth == 0 {
                        break;
                    }
                }
                continue;
            }
            match byte {
                b'"' => in_string = true,
                b'{' | b'[' => depth += 1,
                b'}' | b']' => {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                }
                _ if depth == 0
                    && bytes
                        .get(self.position)
                        .is_none_or(|next| b",]} \t\r\n".contains(next)) =>
                {
                    break;
                }
                _ => {}
            }
        }
        &self.text[start..self.position]
    }

    fn replace(&mut self, actual: &Value) {
        let line_start = self.output.rfind('\n').map_or(0, |index| index + 1);
        let indent: String = self.output[line_start..]
            .chars()
            .take_while(|character| *character == ' ')
            .collect();
        self.skip();
        let text = serde_json::to_string_pretty(actual).unwrap();
        self.output
            .push_str(&text.replace('\n', &format!("\n{indent}")));
    }

    fn value(&mut self, actual: &Value) {
        self.whitespace();
        let start = self.position;
        let text = self.text;
        self.skip();
        let raw = &text[start..self.position];
        let stored: Value = serde_json::from_str(raw).unwrap();
        match (&stored, actual) {
            (Value::Object(stored), Value::Object(fields)) if stored.keys().eq(fields.keys()) => {
                self.position = start;
                self.copy('{');
                loop {
                    self.whitespace();
                    if self.text[self.position..].starts_with('}') {
                        break;
                    }
                    let key = self.skip().to_owned();
                    self.output.push_str(&key);
                    self.whitespace();
                    self.copy(':');
                    let name: String = serde_json::from_str(&key).unwrap();
                    self.value(&fields[&name]);
                    self.whitespace();
                    if self.text[self.position..].starts_with(',') {
                        self.copy(',');
                    }
                }
                self.copy('}');
            }
            (Value::Array(stored), Value::Array(values)) if stored.len() == values.len() => {
                self.position = start;
                self.copy('[');
                for (index, value) in values.iter().enumerate() {
                    self.value(value);
                    self.whitespace();
                    if index + 1 < values.len() {
                        self.copy(',');
                    }
                }
                self.whitespace();
                self.copy(']');
            }
            (Value::Number(stored), Value::Number(current))
                if (stored.as_f64().unwrap() - current.as_f64().unwrap()).abs() <= 1e-9 =>
            {
                self.output.push_str(raw);
            }
            _ if stored == *actual => self.output.push_str(raw),
            _ => {
                self.position = start;
                self.replace(actual);
            }
        }
    }
}

/// Returns the stored JSON text with only the values that disagree with `actual`
/// replaced.
#[allow(dead_code)]
pub fn rewrite(stored: &str, actual: &Value) -> String {
    let mut rewriter = Rewriter {
        text: stored,
        position: 0,
        output: String::new(),
    };
    rewriter.value(actual);
    rewriter.output + &stored[rewriter.position..]
}

/// Rewrites a collection's expected results and their fingerprint in its capture
/// metadata when `MOTE_REFRESH_CAPTURES` is set, and reports whether it did so.
#[allow(dead_code)]
pub fn refresh(folder: &std::path::Path, actual: &Value) -> bool {
    use sha2::{Digest, Sha256};
    if std::env::var_os("MOTE_REFRESH_CAPTURES").is_none() {
        return false;
    }
    let path = folder.join("expected.json");
    let stored = std::fs::read_to_string(&path).unwrap();
    let text = rewrite(&stored, actual);
    std::fs::write(&path, &text).unwrap();
    let metadata_path = folder.join("capture.json");
    let mut metadata = std::fs::read_to_string(&metadata_path).unwrap();
    let previous = format!("{:x}", Sha256::digest(stored.as_bytes()));
    let current = format!("{:x}", Sha256::digest(text.as_bytes()));
    let position = metadata.find(&previous).expect("expected.json fingerprint");
    metadata.replace_range(position..position + previous.len(), &current);
    let previous_bytes = format!("\"bytes\": {}", stored.len());
    let offset = position + metadata[position..].find(&previous_bytes).unwrap();
    metadata.replace_range(
        offset..offset + previous_bytes.len(),
        &format!("\"bytes\": {}", text.len()),
    );
    std::fs::write(metadata_path, metadata).unwrap();
    true
}
