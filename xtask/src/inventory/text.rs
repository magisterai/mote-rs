//! Exact matchers for the few textual patterns in the retained publisher sources.

/// The contents of every fenced `text` block, in order.
pub(super) fn text_blocks(source: &str) -> Vec<&str> {
    const OPENING: &str = "```text\n";
    let mut blocks = Vec::new();
    let mut position = 0;
    while let Some(start) = source[position..].find(OPENING) {
        let content = position + start + OPENING.len();
        let Some(length) = source[content..].find("\n```") else {
            break;
        };
        blocks.push(&source[content..content + length]);
        position = content + length + "\n```".len();
    }
    blocks
}

/// The lines of a text, divided at every boundary that Python's `str.splitlines` recognises.
pub(super) fn lines(text: &str) -> Vec<&str> {
    let mut lines = Vec::new();
    let mut start = 0;
    let mut characters = text.char_indices().peekable();
    while let Some((index, character)) = characters.next() {
        if matches!(
            character,
            '\n' | '\r'
                | '\u{0b}'
                | '\u{0c}'
                | '\u{1c}'
                | '\u{1d}'
                | '\u{1e}'
                | '\u{85}'
                | '\u{2028}'
                | '\u{2029}'
        ) {
            lines.push(&text[start..index]);
            start = index + character.len_utf8();
            if character == '\r' && characters.peek().is_some_and(|(_, next)| *next == '\n') {
                characters.next();
                start += 1;
            }
        }
    }
    if start < text.len() {
        lines.push(&text[start..]);
    }
    lines
}

/// A page footer of the form `12 / 40`, perhaps surrounded by whitespace.
pub(super) fn is_page_number(line: &str) -> bool {
    let digits = |text: &str| !text.is_empty() && text.chars().all(char::is_numeric);
    line.trim()
        .split_once(" / ")
        .is_some_and(|(first, second)| digits(first) && digits(second))
}

/// Every complete `xsd:schema` or `xs:schema` element, ending at the first closing tag.
pub(super) fn schema_fragments(text: &str) -> Vec<&str> {
    let mut fragments = Vec::new();
    let mut position = 0;
    while position < text.len() {
        let opening = ["<xsd:schema", "<xs:schema"]
            .into_iter()
            .filter_map(|opening| {
                let mut from = position;
                while let Some(start) = text[from..].find(opening) {
                    let end = from + start + opening.len();
                    if !text[end..].starts_with(is_word) {
                        return Some((from + start, end));
                    }
                    from += start + 1;
                }
                None
            })
            .min();
        let Some((start, end)) = opening else {
            break;
        };
        let closing = ["</xsd:schema>", "</xs:schema>"]
            .into_iter()
            .filter_map(|closing| text[end..].find(closing).map(|at| end + at + closing.len()))
            .min();
        match closing {
            Some(close) => {
                fragments.push(&text[start..close]);
                position = close;
            }
            None => position = start + 1,
        }
    }
    fragments
}

fn is_word(character: char) -> bool {
    character.is_alphanumeric() || character == '_'
}

/// Every nonempty markup tag, so that prose between the tags is discarded.
pub(super) fn tags(text: &str) -> Vec<&str> {
    let mut tags = Vec::new();
    let mut position = 0;
    while let Some(start) = text[position..].find('<').map(|at| position + at) {
        match text[start + 1..].find('>').map(|at| start + 1 + at) {
            Some(end) if end > start + 1 => {
                tags.push(&text[start..=end]);
                position = end + 1;
            }
            Some(_) => position = start + 1,
            None => break,
        }
    }
    tags
}

/// Every self-closing `xsd:element` declaration with the given name.
pub(super) fn element_declarations<'text>(text: &'text str, name: &str) -> Vec<&'text str> {
    let attribute = format!("name=\"{name}\"");
    let mut declarations = Vec::new();
    let mut position = 0;
    while let Some(start) = text[position..]
        .find("<xsd:element")
        .map(|at| position + at)
    {
        position = start + 1;
        let rest = &text[start + "<xsd:element".len()..];
        let attributes = rest.trim_start();
        if attributes.len() == rest.len() || !attributes.starts_with(&attribute) {
            continue;
        }
        let after_name = text.len() - attributes.len() + attribute.len();
        if let Some(close) = text[after_name..].find('>').map(|at| after_name + at)
            && text[after_name..close].ends_with('/')
        {
            declarations.push(&text[start..=close]);
            position = close + 1;
        }
    }
    declarations
}

pub(super) struct ClauseLink<'text> {
    pub number: &'text str,
    pub name: &'text str,
    pub description: &'text str,
    pub target: &'text str,
}

/// Every index entry of the form `[17.3.1.1 name (description)](target)`.
pub(super) fn clause_links(text: &str) -> Vec<ClauseLink<'_>> {
    fn run(text: &str, accept: impl Fn(char) -> bool) -> usize {
        text.char_indices()
            .find(|(_, character)| !accept(*character))
            .map_or(text.len(), |(index, _)| index)
    }
    fn parse(text: &str) -> Option<(ClauseLink<'_>, usize)> {
        let mut rest = text.strip_prefix('[')?;
        let number = run(rest, |character| character.is_numeric() || character == '.');
        let (number, after) = (
            rest.get(..number).filter(|text| !text.is_empty())?,
            &rest[number..],
        );
        rest = after.strip_prefix(' ')?;
        let name = run(rest, is_word);
        let (name, after) = (
            rest.get(..name).filter(|text| !text.is_empty())?,
            &rest[name..],
        );
        rest = after.strip_prefix(" (")?;
        let description = run(rest, |character| character != ')');
        let (description, after) = (
            rest.get(..description).filter(|text| !text.is_empty())?,
            &rest[description..],
        );
        rest = after.strip_prefix(")](")?;
        let target = run(rest, |character| character != ')');
        let (target, after) = (
            rest.get(..target).filter(|text| !text.is_empty())?,
            &rest[target..],
        );
        rest = after.strip_prefix(')')?;
        Some((
            ClauseLink {
                number,
                name,
                description,
                target,
            },
            text.len() - rest.len(),
        ))
    }
    let mut links = Vec::new();
    let mut position = 0;
    while let Some(start) = text[position..].find('[').map(|at| position + at) {
        match parse(&text[start..]) {
            Some((link, length)) => {
                links.push(link);
                position = start + length;
            }
            None => position = start + 1,
        }
    }
    links
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn patterns_match_as_the_publisher_expressions_do() {
        assert_eq!(
            text_blocks("a```text\none\n```b```text\n\n```"),
            ["one", ""]
        );
        assert_eq!(lines("a\r\nb\x0cc\n"), ["a", "b", "c"]);
        assert!(is_page_number(" 12 / 40 "));
        assert!(!is_page_number("12/40"));
        assert_eq!(
            schema_fragments("<xs:schemas/><xsd:schema a>x</xs:schema>"),
            ["<xsd:schema a>x</xs:schema>"]
        );
        assert_eq!(tags("<>a <b>c<d\n e>"), ["<b>", "<d\n e>"]);
        assert_eq!(
            element_declarations(
                r#"<xsd:element name="glow" type="x"/><xsd:element name="glow">"#,
                "glow"
            ),
            [r#"<xsd:element name="glow" type="x"/>"#]
        );
        let links = clause_links("[x] [17.3.1.1 pStyle (Style)](a.md#b)");
        assert_eq!(
            (
                links[0].number,
                links[0].name,
                links[0].description,
                links[0].target
            ),
            ("17.3.1.1", "pStyle", "Style", "a.md#b")
        );
    }
}
