use super::Options;

#[derive(Clone, Copy)]
pub(super) enum TextKind {
    Ordinary,
    Normal,
    Roman,
}
#[derive(Clone)]
pub(super) enum Node {
    Sequence(Vec<Node>),
    Text(String, TextKind),
    Fraction {
        numerator: Box<Node>,
        denominator: Box<Node>,
        bar: bool,
    },
    Radical {
        radicand: Box<Node>,
        degree: Option<Box<Node>>,
    },
    Script {
        base: Box<Node>,
        subscript: Option<Box<Node>>,
        superscript: Option<Box<Node>>,
    },
    Delimiter {
        begin: String,
        end: String,
        body: Box<Node>,
    },
    Accent {
        character: &'static str,
        body: Box<Node>,
    },
    Bar {
        position: &'static str,
        body: Box<Node>,
    },
    GroupCharacter {
        character: &'static str,
        position: &'static str,
        body: Box<Node>,
    },
    Limit {
        body: Box<Node>,
        limit: Box<Node>,
        upper: bool,
    },
    NaryOperator {
        symbol: &'static str,
        location: &'static str,
    },
    Nary {
        symbol: &'static str,
        location: &'static str,
        subscript: Option<Box<Node>>,
        superscript: Option<Box<Node>>,
    },
    LimitOperator(String),
    Matrix(Vec<Vec<Node>>),
    EquationArray(Vec<Node>),
}
pub(super) fn escape(text: &str) -> String {
    crate::xml::escape(text).expect("equation characters were validated before parsing")
}
const PRESENTATION_PROPERTIES: &str = r#"<a:rPr lang="en-US" b="0" i="1" smtClean="0"><a:latin typeface="Cambria Math" panose="02040503050406030204" pitchFamily="18" charset="0"/></a:rPr>"#;
impl Node {
    pub fn text(text: impl Into<String>) -> Self {
        Self::Text(text.into(), TextKind::Ordinary)
    }
    pub fn xml(&self, options: &Options) -> String {
        let part = |name: &str, node: &Node| format!("<m:{name}>{}</m:{name}>", node.xml(options));
        let optional = |name: &str, node: &Option<Box<Node>>| {
            format!(
                "<m:{name}>{}</m:{name}>",
                node.as_ref()
                    .map(|node| node.xml(options))
                    .unwrap_or_default()
            )
        };
        match self {
            Self::Sequence(nodes) => nodes.iter().map(|node| node.xml(options)).collect(),
            Self::Text(text, kind) => {
                if text.is_empty() {
                    return String::new();
                }
                let mut properties = match kind {
                    TextKind::Ordinary => String::new(),
                    TextKind::Normal => "<m:rPr><m:nor/></m:rPr>".into(),
                    TextKind::Roman => {
                        r#"<m:rPr><m:scr m:val="roman"/><m:sty m:val="p"/></m:rPr>"#.into()
                    }
                };
                let mut word = String::new();
                if !options.color.is_empty() {
                    word.push_str(&format!(r#"<w:color w:val="{}"/>"#, escape(&options.color)));
                }
                if options.size_half_points > 0 {
                    word.push_str(&format!(
                        r#"<w:sz w:val="{0}"/><w:szCs w:val="{0}"/>"#,
                        options.size_half_points
                    ));
                }
                if !word.is_empty() {
                    properties.push_str(&format!("<w:rPr>{word}</w:rPr>"));
                }
                if options.presentation {
                    properties.push_str(PRESENTATION_PROPERTIES);
                }
                format!(
                    r#"<m:r>{properties}<m:t xml:space="preserve">{}</m:t></m:r>"#,
                    escape(text)
                )
            }
            Self::Fraction {
                numerator,
                denominator,
                bar,
            } => format!(
                "<m:f>{}{}{}</m:f>",
                if *bar {
                    ""
                } else {
                    r#"<m:fPr><m:type m:val="noBar"/></m:fPr>"#
                },
                part("num", numerator),
                part("den", denominator)
            ),
            Self::Radical { radicand, degree } => format!(
                "<m:rad>{}{}</m:rad>",
                degree
                    .as_ref()
                    .map(|degree| part("deg", degree))
                    .unwrap_or_else(|| r#"<m:radPr><m:degHide m:val="1"/></m:radPr>"#.into()),
                part("e", radicand)
            ),
            Self::Script {
                base,
                subscript,
                superscript,
            } => {
                let name = match (subscript.is_some(), superscript.is_some()) {
                    (true, true) => "sSubSup",
                    (true, false) => "sSub",
                    (false, true) => "sSup",
                    (false, false) => return base.xml(options),
                };
                format!(
                    "<m:{name}>{}{}{}</m:{name}>",
                    part("e", base),
                    subscript
                        .as_ref()
                        .map(|node| part("sub", node))
                        .unwrap_or_default(),
                    superscript
                        .as_ref()
                        .map(|node| part("sup", node))
                        .unwrap_or_default()
                )
            }
            Self::Delimiter { begin, end, body } => {
                let mut properties = String::new();
                for (name, value) in [("begChr", begin), ("endChr", end)] {
                    if !value.is_empty() {
                        properties.push_str(&format!(r#"<m:{name} m:val="{}"/>"#, escape(value)));
                    }
                }
                format!("<m:d><m:dPr>{properties}</m:dPr>{}</m:d>", part("e", body))
            }
            Self::Accent { character, body } => format!(
                r#"<m:acc><m:accPr><m:chr m:val="{}"/></m:accPr>{}</m:acc>"#,
                escape(character),
                part("e", body)
            ),
            Self::Bar { position, body } => format!(
                r#"<m:bar><m:barPr><m:pos m:val="{position}"/></m:barPr>{}</m:bar>"#,
                part("e", body)
            ),
            Self::GroupCharacter {
                character,
                position,
                body,
            } => {
                // PowerPoint's underbrace uses the character's implicit position.
                let placement = if options.presentation && *position == "bot" {
                    String::new()
                } else {
                    format!(
                        r#"<m:pos m:val="{position}"/><m:vertJc m:val="{}"/>"#,
                        if *position == "top" { "bot" } else { "top" }
                    )
                };
                let control = if options.presentation {
                    format!("<m:ctrlPr>{PRESENTATION_PROPERTIES}</m:ctrlPr>")
                } else {
                    String::new()
                };
                format!(
                    r#"<m:groupChr><m:groupChrPr><m:chr m:val="{character}"/>{placement}{control}</m:groupChrPr>{}</m:groupChr>"#,
                    part("e", body)
                )
            }
            Self::Limit { body, limit, upper } => {
                let name = if *upper { "limUpp" } else { "limLow" };
                format!(
                    "<m:{name}>{}{}</m:{name}>",
                    part("e", body),
                    part("lim", limit)
                )
            }
            Self::NaryOperator { symbol, .. } => Self::text(*symbol).xml(options),
            Self::Nary {
                symbol,
                location,
                subscript,
                superscript,
            } => format!(
                r#"<m:nary><m:naryPr><m:chr m:val="{symbol}"/><m:limLoc m:val="{location}"/>{}{}</m:naryPr>{}{}{}</m:nary>"#,
                if subscript.is_none() {
                    r#"<m:subHide m:val="1"/>"#
                } else {
                    ""
                },
                if superscript.is_none() {
                    r#"<m:supHide m:val="1"/>"#
                } else {
                    ""
                },
                optional("sub", subscript),
                optional("sup", superscript),
                part("e", &Self::text(" "))
            ),
            Self::LimitOperator(name) => Self::Text(name.clone(), TextKind::Roman).xml(options),
            Self::Matrix(rows) => format!(
                "<m:m>{}</m:m>",
                rows.iter()
                    .map(|row| format!(
                        "<m:mr>{}</m:mr>",
                        row.iter().map(|cell| part("e", cell)).collect::<String>()
                    ))
                    .collect::<String>()
            ),
            Self::EquationArray(rows) => format!(
                "<m:eqArr>{}</m:eqArr>",
                rows.iter().map(|row| part("e", row)).collect::<String>()
            ),
        }
    }
}
