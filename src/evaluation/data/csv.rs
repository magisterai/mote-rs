//! Strict comma-separated records with literal strings and permissive row widths.
use crate::model::{Properties, Value};
use std::collections::BTreeMap;

pub(super) fn decode(source: &str, skip_heading: bool) -> Result<Value, String> {
    let records = records(source)?;
    let header_index = usize::from(skip_heading);
    let Some(header) = records.get(header_index) else {
        return Ok(Value::Array(Vec::new()));
    };
    let mut seen = BTreeMap::new();
    let headers: Vec<String> = header
        .iter()
        .enumerate()
        .map(|(index, header)| {
            let mut name = header.trim().to_owned();
            if name.is_empty() {
                name = format!("column_{}", index + 1);
            }
            let count = seen.entry(name.clone()).or_insert(0);
            *count += 1;
            if *count > 1 {
                name = format!("{name}_{count}");
            }
            name
        })
        .collect();
    Ok(Value::Array(
        records[header_index + 1..]
            .iter()
            .map(|record| {
                Value::Object(
                    headers
                        .iter()
                        .enumerate()
                        .map(|(index, name)| {
                            (
                                name.clone(),
                                Value::String(record.get(index).cloned().unwrap_or_default()),
                            )
                        })
                        .collect::<Properties>(),
                )
            })
            .collect(),
    ))
}

fn records(source: &str) -> Result<Vec<Vec<String>>, String> {
    let mut normalized = source.replace("\r\n", "\n");
    if normalized.ends_with('\r') {
        normalized.pop();
    }
    let mut parser = Parser {
        source: &normalized,
        position: 0,
    };
    let mut records = Vec::new();
    while parser.position < normalized.len() {
        if parser.peek() == Some(b'\n') {
            parser.position += 1;
            continue;
        }
        let mut fields = Vec::new();
        loop {
            fields.push(parser.field()?);
            match parser.peek() {
                Some(b',') => parser.position += 1,
                Some(b'\n') => {
                    parser.position += 1;
                    break;
                }
                None => break,
                _ => unreachable!("field parser ends at a record boundary"),
            }
        }
        records.push(fields);
    }
    Ok(records)
}
struct Parser<'source> {
    source: &'source str,
    position: usize,
}
impl Parser<'_> {
    fn peek(&self) -> Option<u8> {
        self.source.as_bytes().get(self.position).copied()
    }
    fn error(&self, message: &str) -> String {
        let before = &self.source[..self.position];
        let line = before.bytes().filter(|byte| *byte == b'\n').count() + 1;
        let column = before.rsplit('\n').next().unwrap_or_default().len() + 1;
        format!("line {line}, column {column}: {message}")
    }
    fn field(&mut self) -> Result<String, String> {
        if self.peek() != Some(b'"') {
            let start = self.position;
            while let Some(byte) = self.peek() {
                if matches!(byte, b',' | b'\n') {
                    break;
                }
                if byte == b'"' {
                    return Err(self.error("bare quote in an unquoted field"));
                }
                self.position += 1;
            }
            return Ok(self.source[start..self.position].into());
        }
        self.position += 1;
        let mut field = String::new();
        let mut start = self.position;
        loop {
            match self.peek() {
                None => return Err(self.error("unclosed quoted field")),
                Some(b'"') => {
                    field.push_str(&self.source[start..self.position]);
                    self.position += 1;
                    match self.peek() {
                        Some(b'"') => {
                            field.push('"');
                            self.position += 1;
                            start = self.position;
                        }
                        Some(b',' | b'\n') | None => return Ok(field),
                        _ => return Err(self.error("unexpected content after a closing quote")),
                    }
                }
                _ => self.position += 1,
            }
        }
    }
}
