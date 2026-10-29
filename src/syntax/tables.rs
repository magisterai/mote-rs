use super::{
    Diagnostic, MarkdownTableCell, MarkdownTableRow, Node, NodeKind, Parser, Span,
    markup::MarkupBoundary,
};
impl Parser<'_> {
    pub(super) fn parse_markdown_table(&mut self) -> Result<Option<Node>, Diagnostic> {
        let start = self.position;
        let Some((header_end, delimiter_start)) = line_at(self.source, start) else {
            return Ok(None);
        };
        let Some(header) = cells(self.source, start, header_end) else {
            return Ok(None);
        };
        let Some((delimiter_end, mut current)) = line_at(self.source, delimiter_start) else {
            return Ok(None);
        };
        let Some(delimiters) = cells(self.source, delimiter_start, delimiter_end) else {
            return Ok(None);
        };
        let Some(alignments) = alignments_for_line(self.source, &delimiters) else {
            return Ok(None);
        };
        if alignments.len() != header.len() {
            return Ok(None);
        }
        let mut rows = vec![self.table_row(header, &alignments, true)?];
        let mut last_end = delimiter_end;
        while let Some((end, next)) = line_at(self.source, current) {
            let Some(row_cells) = cells(self.source, current, end) else {
                break;
            };
            if alignments_for_line(self.source, &row_cells).is_some() {
                break;
            }
            rows.push(self.table_row(row_cells, &alignments, false)?);
            last_end = end;
            current = next;
        }
        self.position = last_end;
        Ok(Some(Node {
            kind: NodeKind::Table(rows),
            span: Span {
                start,
                end: last_end,
            },
        }))
    }
    fn table_row(
        &self,
        cells: Vec<Span>,
        alignments: &[String],
        header: bool,
    ) -> Result<MarkdownTableRow, Diagnostic> {
        let mut result = Vec::new();
        for (index, cell) in cells.into_iter().enumerate() {
            let mut parser = Parser {
                source: &self.source[..cell.end],
                position: cell.start,
                depth: self.depth,
            };
            result.push(MarkdownTableCell {
                alignment: alignments.get(index).cloned().unwrap_or_default(),
                body: parser.parse_markup(MarkupBoundary::TableCell)?,
            });
        }
        Ok(MarkdownTableRow {
            header,
            cells: result,
        })
    }
}
fn line_at(source: &str, start: usize) -> Option<(usize, usize)> {
    if start >= source.len() {
        return None;
    }
    let end = source[start..]
        .find(['\r', '\n'])
        .map_or(source.len(), |offset| start + offset);
    if source[start..end].trim().is_empty() || source[start..end].trim_start().starts_with(']') {
        return None;
    }
    let next = if source[end..].starts_with("\r\n") {
        end + 2
    } else if end < source.len() {
        end + 1
    } else {
        end
    };
    Some((end, next))
}
fn cells(source: &str, mut start: usize, mut end: usize) -> Option<Vec<Span>> {
    let bytes = source.as_bytes();
    while start < end && matches!(bytes[start], b' ' | b'\t') {
        start += 1;
    }
    while end > start && matches!(bytes[end - 1], b' ' | b'\t') {
        end -= 1;
    }
    if !source[start..end].contains('|') {
        return None;
    }
    if bytes[start] == b'|' {
        start += 1;
    }
    if end > start && bytes[end - 1] == b'|' {
        end -= 1;
    }
    let mut result = Vec::new();
    let mut square_depth = 0_usize;
    let mut parenthesis_depth = 0_usize;
    let mut escaped = false;
    let mut cell_start = start;
    for (offset, byte) in bytes.iter().enumerate().take(end).skip(start) {
        if escaped {
            escaped = false;
            continue;
        }
        match *byte {
            b'\\' => escaped = true,
            b'[' => square_depth += 1,
            b']' => square_depth = square_depth.saturating_sub(1),
            b'(' => parenthesis_depth += 1,
            b')' => parenthesis_depth = parenthesis_depth.saturating_sub(1),
            b'|' if square_depth == 0 && parenthesis_depth == 0 => {
                result.push(trimmed_span(source, cell_start, offset));
                cell_start = offset + 1;
            }
            _ => {}
        }
    }
    result.push(trimmed_span(source, cell_start, end));
    (result.len() >= 2).then_some(result)
}
fn trimmed_span(source: &str, start: usize, end: usize) -> Span {
    let text = &source[start..end];
    let start = start + text.len() - text.trim_start_matches([' ', '\t']).len();
    let end = start + text.trim_matches([' ', '\t']).len();
    Span { start, end }
}
fn alignments_for_line(source: &str, cells: &[Span]) -> Option<Vec<String>> {
    cells
        .iter()
        .map(|cell| {
            let text = &source[cell.start..cell.end];
            let marker = text.trim_matches(':');
            if marker.len() < 3 || !marker.bytes().all(|byte| byte == b'-') {
                return None;
            }
            Some(
                match (text.starts_with(':'), text.ends_with(':')) {
                    (true, true) => "center",
                    (true, false) => "left",
                    (false, true) => "right",
                    _ => "",
                }
                .into(),
            )
        })
        .collect()
}
