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
        _ => differences.push(format!("{path}: expected {expected}, actual {actual}")),
    }
}
