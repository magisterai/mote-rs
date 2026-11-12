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
const PRESENTATION_UPRIGHT_PROPERTIES: &str = r#"<a:rPr lang="en-US" b="0" i="0" smtClean="0"><a:latin typeface="Cambria Math" panose="02040503050406030204" pitchFamily="18" charset="0"/></a:rPr>"#;
/// PowerPoint ignores `m:nor` and takes the style of normal text from these
/// properties, which leave its font to the paragraph as Word does.
const PRESENTATION_NORMAL_PROPERTIES: &str = r#"<a:rPr lang="en-US" b="0" i="0" smtClean="0"/>"#;
/// The colour and size of an equation, as Word run properties.
fn word_properties(options: &Options) -> String {
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
    if word.is_empty() {
        word
    } else {
        format!("<w:rPr>{word}</w:rPr>")
    }
}
/// Word draws the bar, sign or delimiters of an object in the paragraph's
/// colour and size unless the object states its own, whatever its runs state.
fn control(options: &Options) -> String {
    let properties = if options.presentation {
        String::new()
    } else {
        word_properties(options)
    };
    if properties.is_empty() {
        properties
    } else {
        format!("<m:ctrlPr>{properties}</m:ctrlPr>")
    }
}
/// The properties element of an object, or nothing if it would be empty.
fn object_properties(name: &str, properties: &str, options: &Options) -> String {
    let control = control(options);
    if properties.is_empty() && control.is_empty() {
        String::new()
    } else {
        format!("<m:{name}>{properties}{control}</m:{name}>")
    }
}
impl Node {
    pub fn text(text: impl Into<String>) -> Self {
        Self::Text(text.into(), TextKind::Ordinary)
    }
    /// The columns of `cases` align left; Word centres matrix columns unless
    /// told otherwise.
    fn matrix_xml(rows: &[Vec<Node>], left: bool, options: &Options) -> String {
        let properties = if left {
            let columns = rows.iter().map(Vec::len).max().unwrap_or(0);
            format!(
                r#"<m:mcs><m:mc><m:mcPr><m:count m:val="{columns}"/><m:mcJc m:val="left"/></m:mcPr></m:mc></m:mcs>"#
            )
        } else {
            String::new()
        };
        let properties = object_properties("mPr", &properties, options);
        let rows: String = rows
            .iter()
            .map(|row| {
                let cells: String = row
                    .iter()
                    .map(|cell| format!("<m:e>{}</m:e>", cell.xml(options)))
                    .collect();
                format!("<m:mr>{cells}</m:mr>")
            })
            .collect();
        format!("<m:m>{properties}{rows}</m:m>")
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
                properties.push_str(&word_properties(options));
                if options.presentation {
                    properties.push_str(match kind {
                        TextKind::Ordinary => PRESENTATION_PROPERTIES,
                        TextKind::Normal => PRESENTATION_NORMAL_PROPERTIES,
                        TextKind::Roman => PRESENTATION_UPRIGHT_PROPERTIES,
                    });
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
                object_properties(
                    "fPr",
                    if *bar {
                        ""
                    } else {
                        r#"<m:type m:val="noBar"/>"#
                    },
                    options
                ),
                part("num", numerator),
                part("den", denominator)
            ),
            Self::Radical { radicand, degree } => format!(
                "<m:rad>{}{}{}</m:rad>",
                object_properties(
                    "radPr",
                    if degree.is_some() {
                        ""
                    } else {
                        r#"<m:degHide m:val="1"/>"#
                    },
                    options
                ),
                degree
                    .as_ref()
                    .map(|degree| part("deg", degree))
                    .unwrap_or_default(),
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
                    "<m:{name}>{}{}{}{}</m:{name}>",
                    object_properties(&format!("{name}Pr"), "", options),
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
                    properties.push_str(&format!(r#"<m:{name} m:val="{}"/>"#, escape(value)));
                }
                let body = match body.as_ref() {
                    Self::Matrix(rows) if begin == "{" && end.is_empty() => {
                        format!("<m:e>{}</m:e>", Self::matrix_xml(rows, true, options))
                    }
                    body => part("e", body),
                };
                format!(
                    "<m:d>{}{body}</m:d>",
                    object_properties("dPr", &properties, options)
                )
            }
            Self::Accent { character, body } => format!(
                r#"<m:acc><m:accPr><m:chr m:val="{}"/>{}</m:accPr>{}</m:acc>"#,
                escape(character),
                control(options),
                part("e", body)
            ),
            Self::Bar { position, body } => format!(
                r#"<m:bar><m:barPr><m:pos m:val="{position}"/>{}</m:barPr>{}</m:bar>"#,
                control(options),
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
                    control(options)
                };
                format!(
                    r#"<m:groupChr><m:groupChrPr><m:chr m:val="{character}"/>{placement}{control}</m:groupChrPr>{}</m:groupChr>"#,
                    part("e", body)
                )
            }
            Self::Limit { body, limit, upper } => {
                let name = if *upper { "limUpp" } else { "limLow" };
                format!(
                    "<m:{name}>{}{}{}</m:{name}>",
                    object_properties(&format!("{name}Pr"), "", options),
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
                r#"<m:nary><m:naryPr><m:chr m:val="{symbol}"/><m:limLoc m:val="{location}"/>{}{}{}</m:naryPr>{}{}{}</m:nary>"#,
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
                control(options),
                optional("sub", subscript),
                optional("sup", superscript),
                part("e", &Self::text(" "))
            ),
            Self::LimitOperator(name) => Self::Text(name.clone(), TextKind::Roman).xml(options),
            Self::Matrix(rows) => Self::matrix_xml(rows, false, options),
            Self::EquationArray(rows) => format!(
                "<m:eqArr>{}{}</m:eqArr>",
                object_properties("eqArrPr", "", options),
                rows.iter().map(|row| part("e", row)).collect::<String>()
            ),
        }
    }
}
