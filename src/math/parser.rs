use super::{
    nodes::{Node, TextKind},
    symbols,
};

pub(super) struct Parser<'a> {
    source: &'a str,
    offset: usize,
    depth: usize,
    pub warnings: Vec<String>,
}
impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source: source.trim(),
            offset: 0,
            depth: 0,
            warnings: Vec::new(),
        }
    }
    pub fn sequence(&mut self, stops: &[&str]) -> Node {
        let mut nodes = Vec::new();
        while self.peek().is_some() {
            self.skip_space();
            if self.peek().is_none() || stops.iter().any(|stop| self.remaining().starts_with(stop))
            {
                break;
            }
            if let Some(atom) = self.atom() {
                nodes.push(self.scripts(atom));
            }
        }
        Node::Sequence(nodes)
    }
    fn atom(&mut self) -> Option<Node> {
        let character = self.peek()?;
        if self.depth >= 128 {
            self.warnings
                .push("math nesting limit reached; remaining source emitted as text".into());
            let remaining = self.remaining().to_owned();
            self.offset = self.source.len();
            return Some(Node::text(remaining));
        }
        self.depth += 1;
        let result = match character {
            '\\' => Some(self.command()),
            '{' => Some(self.group()),
            '}' => {
                self.advance();
                None
            }
            character if decimal_digit(character) => {
                Some(Node::text(self.read_while(decimal_digit)))
            }
            _ => {
                self.advance();
                Some(Node::text(character.to_string()))
            }
        };
        self.depth -= 1;
        result
    }
    fn scripts(&mut self, base: Node) -> Node {
        let mut subscript = None;
        let mut superscript = None;
        loop {
            self.skip_space();
            match self.peek() {
                Some('_') => {
                    self.advance();
                    subscript = Some(Box::new(self.required()));
                }
                Some('^') => {
                    self.advance();
                    superscript = Some(Box::new(self.required()));
                }
                _ => break,
            }
        }
        match base {
            Node::NaryOperator { symbol, location }
                if subscript.is_some() || superscript.is_some() =>
            {
                Node::Nary {
                    symbol,
                    location,
                    subscript,
                    superscript,
                }
            }
            Node::LimitOperator(name) if subscript.is_some() || superscript.is_some() => {
                let upper = superscript.is_some();
                Node::Limit {
                    body: Box::new(Node::Text(name, TextKind::Roman)),
                    limit: superscript.or(subscript).unwrap(),
                    upper,
                }
            }
            Node::GroupCharacter {
                position: "top", ..
            } if superscript.is_some() => Node::Limit {
                body: Box::new(base),
                limit: superscript.unwrap(),
                upper: true,
            },
            Node::GroupCharacter {
                position: "bot", ..
            } if subscript.is_some() => Node::Limit {
                body: Box::new(base),
                limit: subscript.unwrap(),
                upper: false,
            },
            _ => Node::Script {
                base: Box::new(base),
                subscript,
                superscript,
            },
        }
    }
    fn group(&mut self) -> Node {
        self.advance();
        let body = self.sequence(&["}"]);
        if self.peek() == Some('}') {
            self.advance();
        }
        body
    }
    fn required(&mut self) -> Node {
        self.skip_space();
        self.atom().unwrap_or_else(|| Node::text(""))
    }
    fn command(&mut self) -> Node {
        self.advance();
        if self.peek().is_none() {
            return Node::text("\\");
        }
        if self.peek() == Some('\\') {
            self.advance();
            return Node::text(" ");
        }
        let name = self.command_name();
        match name.as_str() {
            "!" | "" | "displaystyle" => Node::text(""),
            "," | ";" | ":" => Node::text(" "),
            "quad" | "qquad" => Node::text("  "),
            "frac" | "dfrac" | "tfrac" | "cfrac" => Node::Fraction {
                numerator: Box::new(self.required()),
                denominator: Box::new(self.required()),
                bar: true,
            },
            "binom" => Node::Delimiter {
                begin: "(".into(),
                end: ")".into(),
                body: Box::new(Node::Fraction {
                    numerator: Box::new(self.required()),
                    denominator: Box::new(self.required()),
                    bar: false,
                }),
            },
            "sqrt" => {
                let degree = self.optional();
                Node::Radical {
                    degree,
                    radicand: Box::new(self.required()),
                }
            }
            "hat" | "vec" | "dot" | "ddot" | "tilde" => Node::Accent {
                character: match name.as_str() {
                    "hat" => "\u{0302}",
                    "vec" => "\u{20d7}",
                    "dot" => "\u{0307}",
                    "ddot" => "\u{0308}",
                    _ => "\u{0303}",
                },
                body: Box::new(self.required()),
            },
            "bar" | "overline" | "underline" => Node::Bar {
                position: if name == "underline" { "bot" } else { "top" },
                body: Box::new(self.required()),
            },
            "overbrace" | "underbrace" => Node::GroupCharacter {
                character: if name == "overbrace" { "⏞" } else { "⏟" },
                position: if name == "overbrace" { "top" } else { "bot" },
                body: Box::new(self.required()),
            },
            "left" => {
                let begin = self.delimiter();
                let body = self.sequence(&["\\right"]);
                if self.remaining().starts_with("\\right") {
                    self.offset += "\\right".len();
                }
                let end = self.delimiter();
                Node::Delimiter {
                    begin,
                    end,
                    body: Box::new(body),
                }
            }
            "begin" => self.environment(),
            "text" => Node::Text(self.group_source(), TextKind::Normal),
            "operatorname" | "mathrm" => Node::Text(self.group_source(), TextKind::Roman),
            "mathbf" | "mathcal" | "mathbb" | "mathfrak" | "mathit" | "mathsf" | "mathtt" => {
                Node::text(symbols::alphabet(&self.group_source(), &name))
            }
            "arg" | "arccos" | "arcsin" | "arctan" | "cos" | "cosh" | "cot" | "coth" | "csc"
            | "deg" | "det" | "dim" | "exp" | "gcd" | "hom" | "inf" | "ker" | "lg" | "ln"
            | "log" | "max" | "min" | "Pr" | "sec" | "sin" | "sinh" | "sup" | "tan" | "tanh" => {
                let text = if self.peek().is_some_and(whitespace) {
                    format!("{name} ")
                } else {
                    name
                };
                Node::Text(text, TextKind::Roman)
            }
            "pmod" => Node::Sequence(vec![
                Node::text(" ("),
                Node::Text("mod".into(), TextKind::Roman),
                Node::text(" "),
                self.required(),
                Node::text(")"),
            ]),
            "lim" => Node::LimitOperator(name),
            "bigcap" | "bigcup" | "prod" | "sum" | "int" | "iint" | "iiint" | "oint" => {
                let (symbol, location) = match name.as_str() {
                    "bigcap" => ("⋂", "undOvr"),
                    "bigcup" => ("⋃", "undOvr"),
                    "prod" => ("∏", "undOvr"),
                    "sum" => ("∑", "undOvr"),
                    "int" => ("∫", "subSup"),
                    "iint" => ("∫∫", "subSup"),
                    "iiint" => ("∫∫∫", "subSup"),
                    _ => ("∮", "subSup"),
                };
                Node::NaryOperator { symbol, location }
            }
            _ => {
                if let Some(symbol) = symbols::symbol(&name) {
                    Node::text(symbol)
                } else {
                    self.warnings
                        .push(format!("unsupported math command \\{name} emitted as text"));
                    Node::text(name)
                }
            }
        }
    }
    fn environment(&mut self) -> Node {
        let name = self.group_source();
        let ending = format!("\\end{{{name}}}");
        let Some(length) = self.remaining().find(&ending) else {
            return Node::text(format!("\\begin{{{name}}}"));
        };
        let mut content = &self.source[self.offset..self.offset + length];
        self.offset += length + ending.len();
        match name.as_str() {
            "pmatrix" | "bmatrix" | "matrix" | "smallmatrix" | "vmatrix" | "Vmatrix" | "cases"
            | "array" => {
                if name == "array" {
                    content = strip_leading_group(content);
                }
                let rows = split(content, "\\\\")
                    .into_iter()
                    .map(|row| {
                        split(row, "&")
                            .into_iter()
                            .map(|cell| self.nonempty_fragment(cell))
                            .collect()
                    })
                    .collect();
                let matrix = Node::Matrix(rows);
                let (begin, end) = match name.as_str() {
                    "pmatrix" => ("(", ")"),
                    "bmatrix" => ("[", "]"),
                    "vmatrix" => ("|", "|"),
                    "Vmatrix" => ("‖", "‖"),
                    "cases" => ("{", ""),
                    _ => ("", ""),
                };
                if begin.is_empty() && end.is_empty() {
                    matrix
                } else {
                    Node::Delimiter {
                        begin: begin.into(),
                        end: end.into(),
                        body: Box::new(matrix),
                    }
                }
            }
            "aligned" | "align" | "split" => Node::EquationArray(
                split(content, "\\\\")
                    .into_iter()
                    .map(|row| {
                        let mut nodes = Vec::new();
                        for (index, fragment) in split(row, "&").iter().enumerate() {
                            if index > 0 {
                                nodes.push(Node::text("&"));
                            }
                            if !fragment.trim().is_empty() {
                                nodes.push(self.fragment(fragment));
                            }
                        }
                        Node::Sequence(nodes)
                    })
                    .collect(),
            ),
            "gather" => Node::EquationArray(
                split(content, "\\\\")
                    .into_iter()
                    .map(|row| self.nonempty_fragment(row))
                    .collect(),
            ),
            _ => {
                self.warnings
                    .push(format!("unsupported math environment {name} flattened"));
                self.fragment(content)
            }
        }
    }
    fn nonempty_fragment(&mut self, source: &str) -> Node {
        if source.trim().is_empty() {
            Node::text(" ")
        } else {
            self.fragment(source)
        }
    }
    fn fragment(&mut self, source: &str) -> Node {
        let mut parser = Parser::new(source);
        parser.depth = self.depth;
        let result = parser.sequence(&[]);
        self.warnings.extend(parser.warnings);
        result
    }
    fn optional(&mut self) -> Option<Box<Node>> {
        self.skip_space();
        if self.peek() != Some('[') {
            return None;
        }
        self.advance();
        let start = self.offset;
        let mut depth = 0;
        while let Some(character) = self.advance() {
            match character {
                '[' => depth += 1,
                ']' if depth == 0 => {
                    return Some(Box::new(
                        self.fragment(&self.source[start..self.offset - 1]),
                    ));
                }
                ']' => depth -= 1,
                _ => {}
            }
        }
        self.warnings
            .push("unterminated optional math argument".into());
        Some(Box::new(self.fragment(&self.source[start..self.offset])))
    }
    fn group_source(&mut self) -> String {
        self.skip_space();
        if self.peek() != Some('{') {
            return String::new();
        }
        self.advance();
        let start = self.offset;
        let mut depth = 1;
        while let Some(character) = self.advance() {
            match character {
                '{' => depth += 1,
                '}' => {
                    depth -= 1;
                    if depth == 0 {
                        return self.source[start..self.offset - 1].into();
                    }
                }
                _ => {}
            }
        }
        self.source[start..self.offset].into()
    }
    fn delimiter(&mut self) -> String {
        self.skip_space();
        if self.peek() == Some('\\') {
            self.advance();
            let name = self.command_name();
            return symbols::symbol(&name).unwrap_or(&name).into();
        }
        match self.advance() {
            Some('.') => String::new(),
            None => {
                self.warnings
                    .push("unterminated math delimiter emitted as a replacement character".into());
                "�".into()
            }
            Some(character) => character.to_string(),
        }
    }
    fn command_name(&mut self) -> String {
        if self.peek().is_some_and(char::is_alphabetic) {
            self.read_while(char::is_alphabetic)
        } else {
            self.advance()
                .map(|character| character.to_string())
                .unwrap_or_default()
        }
    }
    fn read_while(&mut self, predicate: impl Fn(char) -> bool) -> String {
        let start = self.offset;
        while self.peek().is_some_and(&predicate) {
            self.advance();
        }
        self.source[start..self.offset].into()
    }
    fn remaining(&self) -> &str {
        &self.source[self.offset..]
    }
    fn peek(&self) -> Option<char> {
        self.remaining().chars().next()
    }
    fn advance(&mut self) -> Option<char> {
        let character = self.peek()?;
        self.offset += character.len_utf8();
        Some(character)
    }
    fn skip_space(&mut self) {
        while self.peek().is_some_and(whitespace) {
            self.advance();
        }
    }
}
fn whitespace(character: char) -> bool {
    matches!(character, ' ' | '\t' | '\r' | '\n')
}
fn decimal_digit(character: char) -> bool {
    unicode_general_category::get_general_category(character)
        == unicode_general_category::GeneralCategory::DecimalNumber
}
fn split<'a>(source: &'a str, marker: &str) -> Vec<&'a str> {
    let mut parts = Vec::new();
    let mut start = 0;
    let mut offset = 0;
    let mut depth = 0usize;
    while offset < source.len() {
        if depth == 0 && source[offset..].starts_with(marker) {
            parts.push(&source[start..offset]);
            offset += marker.len();
            start = offset;
            continue;
        }
        let character = source[offset..].chars().next().unwrap();
        match character {
            '{' => depth += 1,
            '}' => depth = depth.saturating_sub(1),
            _ => {}
        }
        offset += character.len_utf8();
    }
    parts.push(&source[start..]);
    parts
}
fn strip_leading_group(source: &str) -> &str {
    let source = source.trim();
    if !source.starts_with('{') {
        return source;
    }
    let mut depth = 0;
    for (offset, character) in source.char_indices() {
        match character {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return source[offset + 1..].trim();
                }
            }
            _ => {}
        }
    }
    source
}
