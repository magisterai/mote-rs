//! Typesets parsed equations as positioned glyphs and rules, following the
//! OpenType MATH table of the math font, for PDF and PNG output.
mod constructs;
mod glyphs;

use super::{
    nodes::{Node, TextKind},
    parser::Parser,
};
use crate::measurement::FontSelection;
use std::collections::BTreeSet;
use ttf_parser::{Face, GlyphId, math};

/// An equation laid out for painting. Distances are in points; `x` runs from
/// the left edge of the equation and `rise` upwards from its baseline.
#[derive(Debug, Clone, PartialEq)]
pub struct Formula {
    pub width: f64,
    pub ascent: f64,
    pub descent: f64,
    pub display: bool,
    /// Whether the equation opens with ordinary text rather than a structure.
    /// Word keeps more room around a display equation that does.
    pub leading_text: bool,
    pub font: FontSelection,
    /// The paragraph's font, in which Word sets normal text such as `\text`.
    pub text_font: Option<FontSelection>,
    pub glyphs: Vec<FormulaGlyph>,
    pub rules: Vec<FormulaRule>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FormulaGlyph {
    pub glyph: u16,
    /// The characters the glyph represents, for text extraction.
    pub text: String,
    pub x: f64,
    pub rise: f64,
    pub size: f64,
    /// Whether the glyph comes from the text font rather than the math font.
    pub in_text_font: bool,
}

/// A filled rectangle, such as a fraction bar, whose `rise` is its lower edge.
#[derive(Debug, Clone, PartialEq)]
pub struct FormulaRule {
    pub x: f64,
    pub rise: f64,
    pub width: f64,
    pub height: f64,
}

impl Formula {
    pub fn text(&self) -> String {
        let mut glyphs: Vec<_> = self.glyphs.iter().collect();
        glyphs.sort_by(|first, second| first.x.total_cmp(&second.x));
        glyphs.iter().map(|glyph| glyph.text.as_str()).collect()
    }
}

/// Returns `None` if the font lacks a MATH table.
pub(crate) fn typeset(
    source: &str,
    display: bool,
    lone: bool,
    word: bool,
    size: f64,
    font: &FontSelection,
    text_font: Option<&FontSelection>,
) -> Option<(Formula, Vec<String>)> {
    let bytes = font.bytes.as_ref()?;
    let face = Face::parse(bytes, 0).ok()?;
    let text_font = text_font.and_then(|text_font| {
        let face = Face::parse(text_font.bytes.as_ref()?, 0).ok()?;
        Some((text_font, face))
    });
    let table = face.tables().math?;
    let mut typesetter = Typesetter {
        constants: table.constants?,
        glyph_info: table.glyph_info,
        variants: table.variants,
        units: f64::from(face.units_per_em()),
        face,
        text_face: text_font.as_ref().map(|(_, face)| face.clone()),
        size,
        missing: BTreeSet::new(),
        word,
    };
    let mut parser = Parser::new(source);
    let mut root = parser.sequence(&[]);
    let mut warnings = parser.warnings;
    if parser.incomplete_delimiters {
        warnings.push("incomplete math delimiters; equation emitted as source text".into());
        root = Node::Text(source.trim().into(), TextKind::Normal);
    }
    let style = Style {
        level: if display { Level::Display } else { Level::Text },
        cramped: false,
        full_fractions: display,
        display_gaps: display && !lone,
        enclosed: false,
        argument: false,
    };
    let mut first = &root;
    loop {
        first = match first {
            Node::Sequence(nodes) if !nodes.is_empty() => &nodes[0],
            Node::Script {
                base,
                subscript: None,
                superscript: None,
            } => base,
            _ => break,
        };
    }
    let leading_text = !lone && matches!(first, Node::Text(..));
    let layout = match &root {
        Node::Sequence(nodes) => typesetter.list(nodes, style, false),
        node => typesetter.node(node, style),
    };
    warnings.extend(typesetter.missing.iter().map(|character| {
        format!(
            "math font {:?} lacks U+{:04X}",
            font.family, *character as u32
        )
    }));
    Some((
        Formula {
            width: layout.width,
            ascent: layout.ascent,
            descent: layout.descent,
            display,
            leading_text,
            font: font.clone(),
            text_font: text_font
                .filter(|_| layout.glyphs.iter().any(|glyph| glyph.in_text_font))
                .map(|(text_font, _)| text_font.clone()),
            glyphs: layout.glyphs,
            rules: layout.rules,
        },
        warnings,
    ))
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Level {
    Display,
    Text,
    Script,
    ScriptScript,
}

/// A cramped style raises superscripts less, as TeX does under a bar. Word
/// sets every fraction of a display equation at full size, however deeply it
/// is nested, though only the outermost and those within delimiters take the
/// display shifts; the inner ones keep the display gaps unless the equation is
/// displayed only for standing alone.
#[derive(Clone, Copy)]
struct Style {
    level: Level,
    cramped: bool,
    full_fractions: bool,
    display_gaps: bool,
    enclosed: bool,
    /// Whether the style belongs to a part of a display fraction or to a row
    /// of aligned equations, where Word measures a single character beneath a
    /// script as it measures any other base.
    argument: bool,
}
impl Style {
    fn script(self) -> Self {
        Self {
            level: match self.level {
                Level::Display | Level::Text => Level::Script,
                _ => Level::ScriptScript,
            },
            ..self
        }
    }
    fn subscript(self) -> Self {
        Self {
            cramped: true,
            ..self.script()
        }
    }
    fn numerator(self) -> Self {
        Self {
            level: match self.level {
                Level::Display => Level::Text,
                Level::Text if self.full_fractions => Level::Text,
                Level::Text => Level::Script,
                _ => Level::ScriptScript,
            },
            argument: self.level == Level::Display || self.argument,
            ..self
        }
    }
    fn denominator(self) -> Self {
        Self {
            cramped: true,
            ..self.numerator()
        }
    }
    /// PowerPoint sets a fraction within delimiters of a display equation
    /// with the spacing of a nested fraction.
    fn enclosed(self) -> Self {
        Self {
            enclosed: true,
            ..self
        }
    }
    fn cramped(self) -> Self {
        Self {
            cramped: true,
            ..self
        }
    }
    fn display(self) -> bool {
        self.level == Level::Display
    }
    fn tight(self) -> bool {
        matches!(self.level, Level::Script | Level::ScriptScript)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Class {
    Ordinary,
    Operator,
    Binary,
    Relation,
    Opening,
    Closing,
    Punctuation,
    Inner,
}

/// The TeX spacing table in thirds of the thin, medium and thick spaces; a
/// negative entry applies only in display and text styles.
const SPACING: [[i8; 8]; 8] = [
    [0, 1, -2, -3, 0, 0, 0, -1],
    [1, 1, 0, -3, 0, 0, 0, -1],
    [-2, -2, 0, 0, -2, 0, 0, -2],
    [-3, -3, 0, 0, -3, 0, 0, -3],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 1, -2, -3, 0, 0, 0, -1],
    [-1, -1, 0, -1, -1, -1, -1, -1],
    [-1, 1, -2, -3, -1, 0, -1, -1],
];

#[derive(Default, Clone)]
struct Layout {
    width: f64,
    ascent: f64,
    descent: f64,
    glyphs: Vec<FormulaGlyph>,
    rules: Vec<FormulaRule>,
    italic: f64,
    accent: Option<f64>,
    /// Whether the layout is a single glyph, whose scripts TeX positions from
    /// the font's shifts alone.
    single: bool,
}
impl Layout {
    fn place(&mut self, child: Layout, x: f64, rise: f64) {
        self.ascent = self.ascent.max(child.ascent + rise);
        self.descent = self.descent.max(child.descent - rise);
        self.glyphs
            .extend(child.glyphs.into_iter().map(|glyph| FormulaGlyph {
                x: glyph.x + x,
                rise: glyph.rise + rise,
                ..glyph
            }));
        self.rules
            .extend(child.rules.into_iter().map(|rule| FormulaRule {
                x: rule.x + x,
                rise: rule.rise + rise,
                ..rule
            }));
    }
    fn rule(&mut self, x: f64, rise: f64, width: f64, height: f64) {
        self.ascent = self.ascent.max(rise + height);
        self.descent = self.descent.max(-rise);
        self.rules.push(FormulaRule {
            x,
            rise,
            width,
            height,
        });
    }
    fn centred(self, axis: f64) -> Layout {
        let rise = axis - (self.ascent - self.descent) / 2.0;
        let mut layout = Layout {
            width: self.width,
            italic: self.italic,
            ..Layout::default()
        };
        layout.place(self, 0.0, rise);
        layout
    }
}

struct Atom {
    class: Option<Class>,
    layout: Layout,
}

struct Typesetter<'a> {
    face: Face<'a>,
    text_face: Option<Face<'a>>,
    constants: math::Constants<'a>,
    glyph_info: Option<math::GlyphInfo<'a>>,
    variants: Option<math::Variants<'a>>,
    units: f64,
    size: f64,
    missing: BTreeSet<char>,
    word: bool,
}
impl Typesetter<'_> {
    fn size(&self, style: Style) -> f64 {
        let percent = |value: i16, fallback: f64| {
            if value > 0 {
                f64::from(value) / 100.0
            } else {
                fallback
            }
        };
        self.size
            * match style.level {
                Level::Display | Level::Text => 1.0,
                Level::Script => percent(self.constants.script_percent_scale_down(), 0.7),
                Level::ScriptScript => {
                    percent(self.constants.script_script_percent_scale_down(), 0.5)
                }
            }
    }
    fn scale(&self, style: Style) -> f64 {
        self.size(style) / self.units
    }
    fn value(&self, value: math::MathValue<'_>, style: Style) -> f64 {
        f64::from(value.value) * self.scale(style)
    }
    fn axis(&self, style: Style) -> f64 {
        self.value(self.constants.axis_height(), style)
    }
    /// One eighteenth of an em, the unit of TeX's mathematical spacing.
    fn mu(&self, style: Style) -> f64 {
        self.size(style) / 18.0
    }

    /// A leading empty atom gives an aligned relation its space on the left.
    fn list(&mut self, nodes: &[Node], style: Style, leading: bool) -> Layout {
        let mut atoms = Vec::new();
        if leading {
            atoms.push(Atom {
                class: Some(Class::Ordinary),
                layout: Layout::default(),
            });
        }
        for node in nodes {
            self.atoms(node, style, &mut atoms);
        }
        self.join(atoms, style)
    }
    fn node(&mut self, node: &Node, style: Style) -> Layout {
        self.nucleus(node, style).1
    }
    fn nucleus(&mut self, node: &Node, style: Style) -> (Class, Layout) {
        let mut atoms = Vec::new();
        self.atoms(node, style, &mut atoms);
        if let [
            Atom {
                class: Some(class), ..
            },
        ] = atoms.as_slice()
        {
            let class = *class;
            return (class, atoms.pop().unwrap().layout);
        }
        (Class::Ordinary, self.join(atoms, style))
    }
    fn join(&mut self, mut atoms: Vec<Atom>, style: Style) -> Layout {
        let mut previous: Option<usize> = None;
        for index in 0..atoms.len() {
            let Some(class) = atoms[index].class else {
                continue;
            };
            let before = previous.and_then(|previous| atoms[previous].class);
            if class == Class::Binary
                && before.is_none_or(|before| {
                    matches!(
                        before,
                        Class::Binary
                            | Class::Operator
                            | Class::Relation
                            | Class::Opening
                            | Class::Punctuation
                    )
                })
            {
                atoms[index].class = Some(Class::Ordinary);
            }
            if matches!(class, Class::Relation | Class::Closing | Class::Punctuation)
                && before == Some(Class::Binary)
            {
                atoms[previous.unwrap()].class = Some(Class::Ordinary);
            }
            previous = Some(index);
        }
        if let Some(previous) = previous
            && atoms[previous].class == Some(Class::Binary)
        {
            atoms[previous].class = Some(Class::Ordinary);
        }
        if let [atom] = atoms.as_mut_slice() {
            return std::mem::take(&mut atom.layout);
        }
        let mut layout = Layout::default();
        let mut x = 0.0;
        let mut previous = None;
        for atom in atoms {
            if let Some(class) = atom.class {
                if let Some(before) = previous {
                    x += self.space(before, class, style);
                }
                previous = Some(class);
            }
            let width = atom.layout.width;
            layout.italic = atom.layout.italic;
            layout.place(atom.layout, x, 0.0);
            x += width;
        }
        layout.width = x;
        layout
    }
    fn space(&self, before: Class, after: Class, style: Style) -> f64 {
        let entry = SPACING[before as usize][after as usize];
        if entry < 0 && style.tight() {
            return 0.0;
        }
        let mu = match entry.abs() {
            1 => 3.0,
            2 => 4.0,
            3 => 5.0,
            _ => 0.0,
        };
        mu * self.mu(style)
    }

    fn atoms(&mut self, node: &Node, style: Style, atoms: &mut Vec<Atom>) {
        let atom = |class, layout| Atom {
            class: Some(class),
            layout,
        };
        match node {
            Node::Sequence(nodes) => {
                let layout = self.list(nodes, style, false);
                atoms.push(atom(Class::Ordinary, layout));
            }
            Node::Text(text, kind) => self.text(text, *kind, style, atoms),
            Node::Fraction {
                numerator,
                denominator,
                bar,
            } => {
                let layout = self.fraction(numerator, denominator, *bar, style);
                atoms.push(atom(Class::Inner, layout));
            }
            Node::Radical { radicand, degree } => {
                let layout = self.radical(radicand, degree.as_deref(), style);
                atoms.push(atom(Class::Ordinary, layout));
            }
            Node::Script {
                base,
                subscript: None,
                superscript: None,
            } => self.atoms(base, style, atoms),
            Node::Script {
                base,
                subscript,
                superscript,
            } => {
                let (class, base) = self.nucleus(base, style);
                let layout =
                    self.scripts(base, subscript.as_deref(), superscript.as_deref(), style);
                atoms.push(atom(class, layout));
            }
            Node::Delimiter { begin, end, body } => {
                let layout = self.fenced(begin, end, body, style);
                atoms.push(atom(Class::Inner, layout));
            }
            Node::Accent { character, body } => {
                let layout = self.accent(character, body, style);
                atoms.push(atom(Class::Ordinary, layout));
            }
            Node::Bar { position, body } => {
                let layout = self.bar(*position == "top", body, style);
                atoms.push(atom(Class::Ordinary, layout));
            }
            Node::GroupCharacter {
                character,
                position,
                body,
            } => {
                let layout = self.brace(character, *position == "top", body, style);
                atoms.push(atom(Class::Ordinary, layout));
            }
            Node::Limit { body, limit, upper } => {
                let (class, base) = self.nucleus(body, style);
                let limit = self.node(limit, style.subscript());
                let layout = if *upper {
                    self.limits(base, Some(limit), None, style)
                } else {
                    self.limits(base, None, Some(limit), style)
                };
                atoms.push(atom(class, layout));
            }
            Node::NaryOperator { symbol, .. } => {
                let layout = self.operator(symbol, style);
                atoms.push(atom(Class::Operator, layout));
            }
            Node::Nary {
                symbol,
                location,
                subscript,
                superscript,
            } => {
                let operator = self.operator(symbol, style);
                let layout = if *location == "undOvr" && style.display() {
                    let over = superscript
                        .as_deref()
                        .map(|node| self.node(node, style.script()));
                    let under = subscript
                        .as_deref()
                        .map(|node| self.node(node, style.subscript()));
                    self.limits(operator, over, under, style)
                } else {
                    self.scripts(
                        operator,
                        subscript.as_deref(),
                        superscript.as_deref(),
                        style,
                    )
                };
                atoms.push(atom(Class::Operator, layout));
                // Office output gives the operator a space as its argument,
                // which Word draws and PowerPoint does not.
                if self.word {
                    atoms.push(Atom {
                        class: None,
                        layout: Layout {
                            width: self.space_width(style),
                            ..Layout::default()
                        },
                    });
                }
            }
            Node::LimitOperator(name) => {
                let layout = self.word(name, style);
                atoms.push(atom(Class::Operator, layout));
            }
            Node::Matrix(rows) => {
                let layout = self.matrix(rows, style, false);
                atoms.push(atom(Class::Ordinary, layout));
            }
            Node::EquationArray(rows) => {
                let layout = self.equations(rows, style);
                atoms.push(atom(Class::Ordinary, layout));
            }
        }
    }

    fn text(&mut self, text: &str, kind: TextKind, style: Style, atoms: &mut Vec<Atom>) {
        match kind {
            TextKind::Ordinary => {
                for character in text.chars() {
                    if character == ' ' || character == '\u{2003}' {
                        let width = if character == ' ' {
                            self.space_width(style)
                        } else {
                            self.size(style)
                        };
                        atoms.push(Atom {
                            class: None,
                            layout: Layout {
                                width,
                                ..Layout::default()
                            },
                        });
                        continue;
                    }
                    let layout = self.character(math_character(character), character, style);
                    atoms.push(Atom {
                        class: Some(classify(character)),
                        layout,
                    });
                }
            }
            TextKind::Roman => {
                let word = text.trim();
                if word.is_empty() {
                    return;
                }
                let class = if word.chars().count() > 1 && word.chars().all(char::is_alphabetic) {
                    Class::Operator
                } else {
                    Class::Ordinary
                };
                let layout = self.word(word, style);
                atoms.push(Atom {
                    class: Some(class),
                    layout,
                });
            }
            TextKind::Normal => {
                if text.is_empty() {
                    return;
                }
                let layout = match &self.text_face {
                    Some(face) => self.text_word(face, text, style),
                    None => self.word(text, style),
                };
                atoms.push(Atom {
                    class: Some(Class::Ordinary),
                    layout,
                });
            }
        }
    }
    fn word(&mut self, text: &str, style: Style) -> Layout {
        let mut layout = Layout::default();
        for character in text.chars() {
            if character == ' ' {
                layout.width += self.space_width(style);
                continue;
            }
            let glyph = self.character(character, character, style);
            let width = glyph.width;
            layout.italic = glyph.italic;
            layout.place(glyph, layout.width, 0.0);
            layout.width += width;
        }
        layout
    }
    fn text_word(&self, face: &Face<'_>, text: &str, style: Style) -> Layout {
        let size = self.size(style);
        let scale = size / f64::from(face.units_per_em());
        let mut layout = Layout::default();
        for character in text.chars() {
            let glyph = face.glyph_index(character).unwrap_or(GlyphId(0));
            if let Some(bounds) = face.glyph_bounding_box(glyph) {
                layout.ascent = layout.ascent.max(f64::from(bounds.y_max) * scale);
                layout.descent = layout.descent.max(-f64::from(bounds.y_min) * scale);
            }
            if character != ' ' {
                layout.glyphs.push(FormulaGlyph {
                    glyph: glyph.0,
                    text: character.to_string(),
                    x: layout.width,
                    rise: 0.0,
                    size,
                    in_text_font: true,
                });
            }
            layout.width += f64::from(face.glyph_hor_advance(glyph).unwrap_or_default()) * scale;
        }
        layout
    }
    fn space_width(&self, style: Style) -> f64 {
        self.face
            .glyph_index(' ')
            .and_then(|glyph| self.face.glyph_hor_advance(glyph))
            .map(|advance| f64::from(advance) * self.scale(style))
            .unwrap_or(self.size(style) / 4.0)
    }
}

/// Word and TeX set Latin letters and lowercase Greek letters in mathematical
/// italic, and draw a hyphen as a minus sign.
fn math_character(character: char) -> char {
    let shifted = |base: u32, offset: u32| char::from_u32(base + offset).unwrap_or(character);
    match character {
        'h' => 'ℎ',
        'a'..='z' => shifted(0x1d44e, character as u32 - 'a' as u32),
        'A'..='Z' => shifted(0x1d434, character as u32 - 'A' as u32),
        'α'..='ω' => shifted(0x1d6fc, character as u32 - 'α' as u32),
        '-' => '−',
        '*' => '∗',
        '\'' => '′',
        _ => character,
    }
}

fn classify(character: char) -> Class {
    match character {
        '+' | '-' | '−' | '±' | '∓' | '×' | '÷' | '⋅' | '·' | '∗' | '∘' | '∙' | '∩' | '∪' | '∧'
        | '∨' | '⊕' | '⊗' | '⊖' | '⊙' | '∖' | '†' | '‡' | '⋆' | '⊎' | '⊓' | '⊔' | '•' => {
            Class::Binary
        }
        '=' | '<' | '>' | '≤' | '≥' | '≠' | '≈' | '≡' | '∼' | '≃' | '≅' | '∝' | '∈' | '∉' | '∋'
        | '⊂' | '⊃' | '⊆' | '⊇' | '⊊' | '⊋' | '→' | '←' | '↔' | '⇒' | '⇐' | '⇔' | '↦' | '⟶'
        | '⟵' | '⟹' | '⟸' | '⟺' | '↑' | '↓' | '⇑' | '⇓' | '≪' | '≫' | '⊥' | '∥' | '∣' | '⊢'
        | '⊣' | '⊨' | '≺' | '≻' | '⪯' | '⪰' | '≐' | ':' | '≔' | '≜' | '∶' => {
            Class::Relation
        }
        '(' | '[' | '{' | '⟨' | '⌊' | '⌈' => Class::Opening,
        ')' | ']' | '}' | '⟩' | '⌋' | '⌉' | '!' => Class::Closing,
        ',' | ';' => Class::Punctuation,
        _ => Class::Ordinary,
    }
}
