use crate::charts::support as charts;
use crate::model::{Content, Document, ElementKind, Properties, StyleKind, Value};
use std::collections::BTreeSet;

const RUN: &[&str] = &[
    "font",
    "size",
    "color",
    "highlight",
    "fill",
    "bold",
    "italic",
    "underline",
    "strike",
    "double_strike",
    "small_caps",
];
const PARAGRAPH: &[&str] = &[
    "style",
    "alignment",
    "spacing",
    "spacing_before",
    "spacing_after",
    "line_spacing",
    "indent",
    "border",
    "fill",
    "keep_with_next",
    "keep_together",
    "page_break_before",
    "widow_control",
];

/// Report retained authoring properties that have no native painting effect.
/// This describes the resolved behavior, independently of model acceptance.
pub(super) fn warnings(document: &Document) -> BTreeSet<String> {
    let mut warnings = BTreeSet::new();
    unknown(
        &document.properties,
        &[
            "page",
            "margin",
            "columns",
            "default_font",
            "default_size",
            "language",
        ],
        "document",
        &mut warnings,
    );
    unknown(
        &document.page,
        &["size", "width", "height", "orientation"],
        "document.page",
        &mut warnings,
    );
    unknown(
        &document.margin,
        &["top", "right", "bottom", "left", "header", "footer"],
        "document.margin",
        &mut warnings,
    );
    unknown(
        &document.columns.properties,
        &["count", "gap", "separator", "equal_width"],
        "document.columns",
        &mut warnings,
    );
    if document
        .columns
        .properties
        .get("equal_width")
        .is_some_and(|value| value.to_text() == "false")
    {
        warnings.insert("document.columns.equal_width=false uses equal-width native columns; individual column widths are unsupported".into());
    }
    if document.defaults.contains_key("language") {
        warnings.insert("document.language is retained; native text uses character metrics without language-specific shaping".into());
    }
    for (name, font) in &document.fonts {
        unknown(
            &font.properties,
            &["name", "latin", "files", "word_line_height"],
            &format!("font[{name:?}]"),
            &mut warnings,
        );
    }
    for (name, style) in &document.styles {
        let path = format!("style[{name:?}]");
        if style.kind == StyleKind::Table {
            let properties = crate::tables::styles::declared(style);
            table_style(&properties, &path, &mut warnings);
            continue;
        }
        let allowed: Vec<_> = RUN
            .iter()
            .chain(PARAGRAPH)
            .copied()
            .chain([
                "name",
                "type",
                "based_on",
                "next",
                "link",
                "paragraph",
                "run",
            ])
            .collect();
        unknown(&style.properties, &allowed, &path, &mut warnings);
        nested(&style.properties, &path, false, false, &mut warnings);
        for (field, allowed) in [("paragraph", PARAGRAPH), ("run", RUN)] {
            if let Some(Value::Object(properties)) = style.properties.get(field) {
                let path = format!("{path}.{field}");
                unknown(properties, allowed, &path, &mut warnings);
                nested(properties, &path, false, false, &mut warnings);
                if field == "paragraph" && properties.contains_key("fill") {
                    warnings.insert(format!("{path}.fill does not paint a native paragraph band; apply fill directly to the block"));
                }
            }
        }
    }
    content(
        &document.content,
        "content",
        false,
        false,
        false,
        &mut warnings,
    );
    for (index, section) in document.sections.iter().enumerate() {
        let path = format!("section[{index}]");
        unknown(
            &section.section_properties,
            &["start"],
            &format!("{path}.properties"),
            &mut warnings,
        );
        unknown(
            &section.page_numbering,
            &["restart", "start"],
            &format!("{path}.page_numbering"),
            &mut warnings,
        );
        if matches!(
            section.start.as_str(),
            "even_page" | "odd_page" | "next_column"
        ) {
            warnings.insert(format!("{path}.properties.start={} begins a new native page; parity and next-column section starts are unsupported", section.start));
        }
        unknown(
            &section.page,
            &["size", "width", "height", "orientation"],
            &format!("{path}.page"),
            &mut warnings,
        );
        unknown(
            &section.margin,
            &["top", "bottom", "left", "right", "header", "footer"],
            &format!("{path}.margin"),
            &mut warnings,
        );
        unknown(
            &section.columns.properties,
            &["count", "gap", "separator", "equal_width"],
            &format!("{path}.columns"),
            &mut warnings,
        );
        if section
            .columns
            .properties
            .get("equal_width")
            .is_some_and(|value| value.to_text() == "false")
        {
            warnings.insert(format!("{path}.columns.equal_width=false uses equal-width native columns; individual column widths are unsupported"));
        }
        for (kind, story) in [("header", &section.header), ("footer", &section.footer)] {
            let path = format!("{path}.{kind}");
            unknown(
                &story.properties,
                &["height", "distance"],
                &path,
                &mut warnings,
            );
            content(&story.content, &path, false, true, false, &mut warnings);
            story_fallbacks(&story.content, &path, &mut warnings);
        }
    }
    warnings
}
fn story_fallbacks(nodes: &[Content], path: &str, warnings: &mut BTreeSet<String>) {
    for node in nodes {
        if let Content::Element(element) = node {
            if element.kind == ElementKind::Table {
                warnings.insert(format!("{path} table structure is flattened into visible text; cell geometry is not preserved"));
            }
            story_fallbacks(&element.children, path, warnings);
        }
    }
}
fn content(
    nodes: &[Content],
    path: &str,
    in_cell: bool,
    flattened: bool,
    drawing: bool,
    warnings: &mut BTreeSet<String>,
) {
    for (index, node) in nodes.iter().enumerate() {
        let Content::Element(element) = node else {
            continue;
        };
        let path = format!("{path}[{index}].{}", element.kind.as_text());
        let attributes = &element.attributes;
        if drawing
            && matches!(
                element.kind,
                ElementKind::Table
                    | ElementKind::Row
                    | ElementKind::Cell
                    | ElementKind::Image
                    | ElementKind::Shape
                    | ElementKind::Chart
                    | ElementKind::Line
                    | ElementKind::Figure
                    | ElementKind::Caption
                    | ElementKind::Break
            )
        {
            warnings.insert(format!("{path} is omitted from native drawing text"));
            continue;
        }
        if drawing
            && matches!(
                element.kind,
                ElementKind::PageNumber
                    | ElementKind::Label
                    | ElementKind::Reference
                    | ElementKind::Bookmark
                    | ElementKind::Footnote
                    | ElementKind::Endnote
            )
        {
            warnings.insert(format!("{path} contributes only child text inside native drawings; field, anchor and note semantics are omitted"));
        }
        if drawing {
            for field in [
                "border",
                "keep_with_next",
                "keep_together",
                "page_break_before",
                "widow_control",
            ] {
                if attributes.contains_key(field) {
                    warnings.insert(format!("{path}.{field} has no native drawing text effect"));
                }
            }
        }
        let list = matches!(element.kind, ElementKind::List | ElementKind::OrderedList);
        let item = element.kind == ElementKind::ListItem;
        let mut allowed = Vec::new();
        match element.kind {
            ElementKind::Slide | ElementKind::Box => {
                warnings.insert(format!("{path} presentation output is not yet supported"));
            }
            ElementKind::Shape => {
                allowed.extend([
                    "position",
                    "kind",
                    "fill",
                    "border",
                    "rotation",
                    "flip_horizontal",
                    "flip_vertical",
                    "opacity",
                    "shadow",
                    "name",
                    "text",
                    "margin",
                    "alignment",
                    "vertical_alignment",
                ]);
                allowed.extend_from_slice(crate::drawings::TEXT_ATTRIBUTES);
                if crate::properties::number(attributes, "rotation").unwrap_or(0.0) != 0.0
                    && contains_link(&element.children)
                {
                    warnings.insert(format!("{path} rotates visible text, but PDF link hit areas remain at unrotated positions"));
                }
            }
            ElementKind::Chart => {
                allowed.extend_from_slice(charts::warnings(
                    attributes, &path, flattened, true, warnings,
                ));
            }
            ElementKind::Line => {
                warnings.insert(format!(
                    "{path} drawing geometry is not yet supported in native output"
                ));
            }
            ElementKind::Paragraph
            | ElementKind::Heading
            | ElementKind::Blockquote
            | ElementKind::ListItem => {
                allowed.extend_from_slice(RUN);
                allowed.extend_from_slice(PARAGRAPH);
                if element.kind == ElementKind::Heading {
                    allowed.extend(["level", "numbering", "marker"]);
                }
                if item {
                    allowed.extend(["level", "tight", "marker", "checked", "checkbox"]);
                }
            }
            ElementKind::Span | ElementKind::Strong | ElementKind::Emphasis | ElementKind::Code => {
                allowed.extend_from_slice(RUN);
                allowed.push("style");
                if element.kind == ElementKind::Code {
                    allowed.push("preserve_space");
                }
            }
            ElementKind::List | ElementKind::OrderedList => {
                allowed.extend(["format", "tight", "level", "indent", "spacing"]);
                if element.kind == ElementKind::OrderedList {
                    allowed.extend(["id", "start", "restart"]);
                }
            }
            ElementKind::Table => {
                allowed.extend(["style", "width", "alignment", "cell", "borders"])
            }
            ElementKind::Row => allowed.extend(["repeat_header", "cant_split"]),
            ElementKind::Cell => allowed.extend([
                "style",
                "width",
                "colspan",
                "rowspan",
                "fill",
                "vertical_alignment",
                "alignment",
                "margin",
                "borders",
            ]),
            ElementKind::Image => allowed.extend([
                "src",
                "alt",
                "name",
                "title",
                "description",
                "width",
                "height",
                "alignment",
                "spacing",
                "crop",
                "rotation",
                "flip_horizontal",
                "flip_vertical",
                "effects",
                "border",
                "link",
                "placement",
            ]),
            ElementKind::Link => allowed.push("url"),
            ElementKind::Break => allowed.push("type"),
            ElementKind::Figure | ElementKind::Caption => {
                allowed.extend(["kind", "position", "alignment", "numbering"]);
                if element.kind == ElementKind::Caption {
                    allowed.push("style");
                }
                if let Some(Value::Object(numbering)) = element.attributes.get("numbering") {
                    unknown(
                        numbering,
                        &[
                            "enabled",
                            "restart",
                            "start",
                            "id",
                            "label",
                            "prefix",
                            "format",
                            "separator",
                        ],
                        "numbering",
                        warnings,
                    );
                }
            }
            ElementKind::Bookmark | ElementKind::Label => {
                allowed.extend(["name", "kind", "title", "number"])
            }
            ElementKind::Reference => {
                allowed.extend(["target", "kind", "display", "fallback", "title"])
            }
            ElementKind::Footnote | ElementKind::Endnote => {
                allowed.push("id");
            }
            ElementKind::PageNumber => allowed.push("kind"),
            ElementKind::Tab | ElementKind::LineBreak | ElementKind::Divider => {}
        }
        unknown(attributes, &allowed, &path, warnings);
        if element.kind == ElementKind::Image {
            image_properties(attributes, &path, warnings);
            if flattened {
                warnings.insert(format!(
                    "{path} uses a visible image placeholder inside flattened text"
                ));
            }
        } else if element.kind == ElementKind::Shape {
            for (group, fields) in [
                ("position", &["x", "y", "width", "height"][..]),
                ("border", &["width", "color", "style", "opacity"][..]),
                (
                    "shadow",
                    &["color", "opacity", "blur", "distance", "angle"][..],
                ),
            ] {
                if let Some(Value::Object(properties)) = attributes.get(group) {
                    unknown(properties, fields, &format!("{path}.{group}"), warnings);
                }
            }
        } else if matches!(element.kind, ElementKind::Table | ElementKind::Cell) {
            table_fields(attributes, &path, warnings);
        } else if element.kind != ElementKind::Chart {
            nested(attributes, &path, list, item, warnings);
        }
        if element.kind == ElementKind::Heading
            && let Some(Value::Object(numbering)) = attributes.get("numbering")
        {
            unknown(
                numbering,
                &["enabled", "id", "format", "separator", "restart", "start"],
                &format!("{path}.numbering"),
                warnings,
            );
            if let Some(format) = numbering.get("format")
                && !matches!(
                    format.to_text().as_str(),
                    "" | "decimal" | "lower_alpha" | "upper_alpha" | "lower_roman" | "upper_roman"
                )
            {
                warnings.insert(format!("{path}.numbering.format uses decimal native counters for this unsupported value"));
            }
        }
        if list && let Some(format) = attributes.get("format") {
            let format = format.to_text();
            let supported = if element.kind == ElementKind::List {
                matches!(format.as_str(), "" | "bullet")
            } else {
                matches!(
                    format.trim().to_lowercase().as_str(),
                    "" | "decimal" | "lower_alpha" | "upper_alpha" | "lower_roman" | "upper_roman"
                )
            };
            if !supported {
                warnings.insert(format!(
                    "{path}.format uses the default native marker for this unsupported value"
                ));
            }
        }
        if item {
            if attributes.contains_key("checkbox") && !attributes.contains_key("checked") {
                warnings.insert(format!(
                    "{path}.checkbox does not create a native task marker; use checked"
                ));
            }
            if !drawing
                && let Some(marker) = attributes.get("marker")
                && !source_marker(&marker.to_text())
            {
                warnings.insert(format!(
                    "{path}.marker is retained; custom native marker text is unsupported"
                ));
            }
            if !has_text(&element.children) {
                warnings.insert(format!("{path} is rendered as a marker-only blank line"));
            }
        }
        if element.kind == ElementKind::Blockquote && !in_cell {
            for child in &element.children {
                if let Content::Element(child) = child
                    && (matches!(
                        child.kind,
                        ElementKind::List | ElementKind::OrderedList | ElementKind::Blockquote
                    ) || (matches!(child.kind, ElementKind::Paragraph | ElementKind::Heading)
                        && !child.attributes.is_empty()))
                {
                    warnings.insert(format!("{path} flattens child blocks to inline text; child block formatting and markers are not rendered"));
                    break;
                }
            }
        }
        if in_cell && matches!(element.kind, ElementKind::Paragraph | ElementKind::Heading) {
            for property in ["border", "fill"] {
                if attributes.contains_key(property) {
                    warnings.insert(format!("{path}.{property} does not paint a native paragraph band inside a cell; inline highlighting remains visible"));
                }
            }
        }
        if element.kind == ElementKind::Caption
            && attributes.contains_key("numbering")
            && (in_cell || flattened)
        {
            warnings.insert(format!("{path}.numbering is omitted from native cell or flattened story text; reference numbering can differ from visible caption numbers"));
        }
        let cell_context = in_cell || element.kind == ElementKind::Cell;
        let flattened = flattened
            || (!cell_context
                && matches!(
                    element.kind,
                    ElementKind::Blockquote | ElementKind::ListItem
                ));
        content(
            &element.children,
            &path,
            cell_context,
            flattened,
            drawing || element.kind == ElementKind::Shape,
            warnings,
        );
    }
}
fn nested(
    properties: &Properties,
    path: &str,
    list: bool,
    item: bool,
    warnings: &mut BTreeSet<String>,
) {
    if let Some(Value::Object(spacing)) = properties.get("spacing") {
        unknown(
            spacing,
            if list {
                &["after"]
            } else {
                &[
                    "before",
                    "after",
                    "line",
                    "spacing_before",
                    "spacing_after",
                    "line_spacing",
                ]
            },
            &format!("{path}.spacing"),
            warnings,
        );
        for field in ["line", "line_spacing"] {
            if let Some(Value::Object(line)) = spacing.get(field) {
                unknown(
                    line,
                    &["rule", "value"],
                    &format!("{path}.spacing.{field}"),
                    warnings,
                );
            }
        }
    }
    if let Some(Value::Object(line)) = properties.get("line_spacing") {
        unknown(
            line,
            &["rule", "value"],
            &format!("{path}.line_spacing"),
            warnings,
        );
    }
    if let Some(Value::Object(indent)) = properties.get("indent") {
        let fields: &[&str] = if list {
            &["left", "level", "hanging"]
        } else if item {
            &["left", "right", "first_line", "hanging", "level"]
        } else {
            &["left", "right", "first_line", "hanging"]
        };
        unknown(indent, fields, &format!("{path}.indent"), warnings);
    }
    if let Some(Value::Object(border)) = properties.get("border") {
        unknown(
            border,
            &["all", "bottom"],
            &format!("{path}.border"),
            warnings,
        );
        for side in ["all", "bottom"] {
            if let Some(Value::Object(properties)) = border.get(side) {
                unknown(
                    properties,
                    &["color"],
                    &format!("{path}.border.{side}"),
                    warnings,
                );
            }
        }
    }
}
fn unknown(
    properties: &Properties,
    supported: &[&str],
    path: &str,
    warnings: &mut BTreeSet<String>,
) {
    for name in properties.keys() {
        if !supported.contains(&name.as_str()) {
            warnings.insert(format!(
                "{path}.{name} is retained but has no native rendering effect"
            ));
        }
    }
}
fn has_text(nodes: &[Content]) -> bool {
    nodes.iter().any(|node| match node {
        Content::Text(text) => !text.is_empty(),
        Content::Element(element) => {
            !matches!(element.kind, ElementKind::List | ElementKind::OrderedList)
                && has_text(&element.children)
        }
    })
}
fn source_marker(marker: &str) -> bool {
    let mut marker = marker.trim();
    for suffix in ["[ ]", "[x]", "[X]"] {
        if let Some(prefix) = marker.strip_suffix(suffix)
            && (prefix.is_empty() || prefix.ends_with([' ', '\t']))
        {
            marker = prefix.trim();
            break;
        }
    }
    matches!(marker, "" | "-" | "+" | "*")
        || marker.strip_suffix('.').is_some_and(|digits| {
            !digits.is_empty() && digits.bytes().all(|byte| byte.is_ascii_digit())
        })
}

fn image_properties(attributes: &Properties, path: &str, warnings: &mut BTreeSet<String>) {
    for (group, allowed) in [
        ("spacing", &["before", "after"][..]),
        ("crop", &["left", "top", "right", "bottom"][..]),
        (
            "effects",
            &["brightness", "contrast", "grayscale", "transparency"][..],
        ),
        ("border", &["width", "color", "style"][..]),
        ("placement", &["width", "height", "align", "horizontal"][..]),
    ] {
        if let Some(Value::Object(fields)) = attributes.get(group) {
            unknown(fields, allowed, &format!("{path}.{group}"), warnings);
        }
    }
    if let Some(Value::Object(border)) = attributes.get("border")
        && let Some(style) = border.get("style")
        && matches!(style.to_text().as_str(), "dashed" | "dotted")
    {
        warnings.insert(format!("{path}.border.style uses a solid native stroke"));
    }
    if let Some(Value::Object(placement)) = attributes.get("placement")
        && let Some(Value::Object(horizontal)) = placement.get("horizontal")
    {
        unknown(
            horizontal,
            &["relative_to", "alignment"],
            &format!("{path}.placement.horizontal"),
            warnings,
        );
        if horizontal
            .get("relative_to")
            .is_some_and(|value| !matches!(value.to_text().as_str(), "" | "column"))
        {
            warnings.insert(format!(
                "{path}.placement.horizontal.relative_to uses column-relative native flow"
            ));
        }
        if horizontal
            .get("alignment")
            .is_some_and(|value| !matches!(value.to_text().as_str(), "left" | "center" | "right"))
        {
            warnings.insert(format!(
                "{path}.placement.horizontal.alignment uses left native flow"
            ));
        }
    }
}

fn table_fields(properties: &Properties, path: &str, warnings: &mut BTreeSet<String>) {
    if let Some(Value::Object(cell)) = properties.get("cell") {
        unknown(
            cell,
            &[
                "style",
                "width",
                "fill",
                "vertical_alignment",
                "alignment",
                "margin",
                "borders",
            ],
            &format!("{path}.cell"),
            warnings,
        );
        table_fields(cell, &format!("{path}.cell"), warnings);
    }
    if let Some(Value::Object(margin)) = properties.get("margin") {
        unknown(
            margin,
            &[
                "all",
                "horizontal",
                "vertical",
                "top",
                "right",
                "bottom",
                "left",
            ],
            &format!("{path}.margin"),
            warnings,
        );
    }
    if let Some(Value::Object(borders)) = properties.get("borders") {
        unknown(
            borders,
            &[
                "all", "outside", "inside", "top", "right", "bottom", "left", "insideH", "insideV",
                "inside_h", "inside_v",
            ],
            &format!("{path}.borders"),
            warnings,
        );
        for (side, value) in borders {
            let style = if let Value::Object(border) = value {
                unknown(
                    border,
                    &["style", "val", "width", "size", "color"],
                    &format!("{path}.borders.{side}"),
                    warnings,
                );
                border
                    .get("style")
                    .or_else(|| border.get("val"))
                    .map(Value::to_text)
                    .unwrap_or_default()
            } else {
                value.to_text()
            };
            if !matches!(
                style.as_str(),
                "" | "single" | "none" | "nil" | "dotted" | "dashed"
            ) {
                warnings.insert(format!("{path}.borders.{side} uses a single native stroke for {style:?}; border conflict priority is retained"));
            }
        }
    }
}
fn table_style(properties: &Properties, path: &str, warnings: &mut BTreeSet<String>) {
    unknown(
        properties,
        &[
            "name",
            "type",
            "based_on",
            "next",
            "link",
            "style",
            "width",
            "fill",
            "vertical_alignment",
            "alignment",
            "margin",
            "borders",
            "cell",
            "paragraph",
            "text",
            "banding",
            "first_row",
            "last_row",
        ],
        path,
        warnings,
    );
    table_fields(properties, path, warnings);
    for field in ["first_row", "last_row"] {
        if let Some(Value::Object(region)) = properties.get(field) {
            table_style(region, &format!("{path}.{field}"), warnings);
        }
    }
    if let Some(Value::Object(banding)) = properties.get("banding") {
        unknown(
            banding,
            &["rows", "odd", "even"],
            &format!("{path}.banding"),
            warnings,
        );
        for field in ["odd", "even"] {
            if let Some(Value::Object(region)) = banding.get(field) {
                table_style(region, &format!("{path}.banding.{field}"), warnings);
            }
        }
    }
    for (field, allowed) in [("paragraph", PARAGRAPH), ("text", RUN)] {
        if let Some(Value::Object(fields)) = properties.get(field) {
            unknown(fields, allowed, &format!("{path}.{field}"), warnings);
            nested(fields, &format!("{path}.{field}"), false, false, warnings);
        }
    }
}

fn contains_link(nodes: &[Content]) -> bool {
    nodes.iter().any(|node| {
        matches!(node, Content::Element(element)
        if element.kind == ElementKind::Link || contains_link(&element.children))
    })
}
