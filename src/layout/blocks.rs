use super::{
    LayoutError, LayoutStyleResolver, ListMarker, ListMarkerKind, ParagraphText, TextRun,
    lists::{ListCursor, ListGeometry, ListNumbers},
    numbering::HeadingNumbers,
    paragraphs::plan_resolved_paragraph,
    resolve_inline, style_values as values,
};
use crate::{
    measurement::TextMeasurer,
    model::{Content, Element, ElementKind, Properties, Value},
};

pub(super) enum Block {
    Drawing(Box<super::drawings::Drawing>),
    Paragraph(Box<PreparedParagraph>),
    Break(String),
    Image(Box<super::images::ImageBlock>),
    Table(Box<super::tables::TablePlan>),
}
pub(super) struct PreparedParagraph {
    pub text: ParagraphText,
    pub anchors: Vec<String>,
    pub marker: Option<PreparedMarker>,
    pub clear_fill_before: bool,
    pub clear_fill_after: bool,
}
pub(super) struct PreparedMarker {
    pub level: usize,
    pub hanging: f64,
    pub marker: ListMarker,
}
impl PreparedParagraph {
    pub fn minimum_start(&self) -> f64 {
        if self.marker.as_ref().is_some_and(|marker| marker.level == 0) {
            self.text.lines.iter().map(|line| line.height).sum::<f64>()
                + self
                    .text
                    .style
                    .spacing_after
                    .max(self.text.style.line_height)
        } else {
            0.0
        }
    }
}

#[derive(Default, Clone)]
pub(super) struct BlockNumbers {
    pub headings: HeadingNumbers,
    pub lists: ListNumbers,
    pub captions: std::collections::BTreeMap<String, u64>,
}
pub(super) fn prepare_content(
    content: &[Content],
    width: f64,
    styles: &LayoutStyleResolver<'_>,
    measurer: &mut TextMeasurer<'_>,
    images: &crate::media::ImageCatalog,
    numbers: &mut BlockNumbers,
    in_cell: bool,
) -> Result<Vec<Block>, LayoutError> {
    let content = super::captions::expand(content);
    let mut builder = BlockBuilder {
        width,
        styles,
        measurer,
        blocks: Vec::new(),
        numbers,
        in_cell,
        flow_start: !in_cell,
        pending_anchors: Vec::new(),
    };
    for content in &content {
        if let Some(labels) = crate::references::standalone_labels(content) {
            builder.pending_anchors.extend(
                labels
                    .iter()
                    .map(|label| {
                        crate::properties::text(&label.attributes, "name")
                            .trim()
                            .to_owned()
                    })
                    .filter(|name| !name.is_empty()),
            );
            continue;
        }
        let Content::Element(element) = content else {
            return Err(LayoutError::Unsupported(
                "document planning requires evaluated block content".into(),
            ));
        };
        if values::boolean(&element.attributes, super::captions::BEFORE) {
            builder.flow_start = false;
        }
        match element.kind {
            ElementKind::Paragraph
            | ElementKind::Heading
            | ElementKind::Blockquote
            | ElementKind::Caption => builder.paragraph(element)?,
            ElementKind::List | ElementKind::OrderedList => {
                builder.pending_anchors.clear();
                builder.list(element, 0, true)?;
                builder.flow_start = false;
            }
            ElementKind::Shape => {
                if in_cell {
                    return Err(LayoutError::Unsupported(
                        "native shapes in table cells are not yet supported".into(),
                    ));
                }
                let mut drawing = super::drawings::Drawing::prepare(element)?;
                drawing.anchors = std::mem::take(&mut builder.pending_anchors);
                builder.blocks.push(Block::Drawing(Box::new(drawing)));
                builder.flow_start = false;
            }
            ElementKind::Image => {
                match super::images::ImageBlock::prepare(element, images, width) {
                    Ok(mut image) => {
                        image.anchors = std::mem::take(&mut builder.pending_anchors);
                        builder.blocks.push(Block::Image(Box::new(image)));
                    }
                    Err(error) => builder.paragraph(&super::images::fallback(element, &error))?,
                }
                builder.flow_start = false;
            }
            ElementKind::Table => {
                let mut table = super::tables::prepare(
                    element,
                    width,
                    styles,
                    builder.measurer,
                    images,
                    builder.numbers,
                )?;
                table.anchors = std::mem::take(&mut builder.pending_anchors);
                builder.blocks.push(Block::Table(Box::new(table)));
                builder.flow_start = false;
            }
            ElementKind::Break => {
                builder.pending_anchors.clear();
                let kind = values::text(&element.attributes, "type");
                builder.flow_start = kind != "line";
                builder.blocks.push(Block::Break(kind));
            }
            _ => {
                return Err(LayoutError::Unsupported(format!(
                    "native document planning cannot place a standalone {}",
                    element.kind.as_text()
                )));
            }
        }
    }
    Ok(builder.blocks)
}
struct BlockBuilder<'builder, 'document, 'assets> {
    width: f64,
    styles: &'builder LayoutStyleResolver<'document>,
    measurer: &'builder mut TextMeasurer<'assets>,
    blocks: Vec<Block>,
    numbers: &'builder mut BlockNumbers,
    in_cell: bool,
    flow_start: bool,
    pending_anchors: Vec<String>,
}
impl BlockBuilder<'_, '_, '_> {
    fn paragraph(&mut self, original: &Element) -> Result<(), LayoutError> {
        let mut element = original.clone();
        if element.kind == ElementKind::Blockquote {
            element
                .attributes
                .entry("style".into())
                .or_insert(Value::String("BlockQuote".into()));
        }
        if self.flow_start
            && element.kind == ElementKind::Heading
            && !explicit_before(&element.attributes)
        {
            element
                .attributes
                .insert("spacing_before".into(), Value::Number(0.0));
        }
        let style = self.styles.paragraph(&element);
        let mut runs = resolve_inline(&element.children, &style.text, self.styles);
        if element.kind == ElementKind::Caption && !self.in_cell {
            let parent = match element.attributes.get(super::captions::PARENT) {
                Some(Value::Object(parent)) => parent.clone(),
                _ => Properties::new(),
            };
            let numbering = crate::captions::Numbering::resolve(
                &values::text(&element.attributes, super::captions::KIND),
                &element.attributes,
                &parent,
            );
            if let Some(number) = numbering
                .advance(&mut self.numbers.captions)
                .map_err(LayoutError::InvalidGeometry)?
            {
                let mut prefix_style = style.text.clone();
                prefix_style.measurement.variant = crate::measurement::FontVariant::from_emphasis(
                    true,
                    matches!(
                        prefix_style.measurement.variant,
                        crate::measurement::FontVariant::Italic
                            | crate::measurement::FontVariant::BoldItalic
                    ),
                );
                runs.insert(
                    0,
                    TextRun::new(
                        format!(
                            "{} {}{}",
                            numbering.label, number.formatted, numbering.separator
                        ),
                        prefix_style,
                    ),
                );
            }
        }
        if element.kind == ElementKind::Heading
            && let Some(marker) = self.numbers.headings.next(&element)?
        {
            let marker = TextRun::new(marker, style.text.clone());
            let width = self
                .measurer
                .measure(&marker.style.measurement, &marker.text, true)?
                .measurement
                .layout_width
                .max(0.0);
            let mut separator = TextRun::new(" ", style.text.clone());
            separator.synthetic_tab = true;
            separator.preserve_space = true;
            separator.fixed_advance = Some(((width / 36.0).floor() + 1.0) * 36.0 - width);
            runs.splice(0..0, [marker, separator]);
        }
        let mut paragraph = plan_resolved_paragraph(
            style,
            self.styles.border_color(&element),
            &runs,
            self.width,
            self.measurer,
            true,
        )?;
        if element.kind == ElementKind::Caption && runs.is_empty() {
            paragraph.lines.clear();
            paragraph.style.spacing_before = 0.0;
            paragraph.style.spacing_after = 0.0;
        }
        if paragraph.is_empty {
            if paragraph.style.line_height <= 0.0 {
                return Err(LayoutError::InvalidGeometry(
                    "blank paragraph line height must be positive",
                ));
            }
            for line in &mut paragraph.lines {
                line.width = paragraph.first_frame.width;
                line.height = paragraph.style.line_height;
                line.baseline = paragraph.style.line_height * 0.8;
            }
        }
        self.blocks
            .push(Block::Paragraph(Box::new(PreparedParagraph {
                anchors: {
                    let mut anchors = std::mem::take(&mut self.pending_anchors);
                    anchors.extend(super::references::leading_names(&element.children));
                    anchors
                },
                text: paragraph,
                marker: None,
                clear_fill_before: values::boolean(&element.attributes, super::captions::BEFORE),
                clear_fill_after: values::boolean(&element.attributes, super::captions::AFTER),
            })));
        self.flow_start = false;
        Ok(())
    }
    fn list(
        &mut self,
        list: &Element,
        fallback_level: usize,
        top_level: bool,
    ) -> Result<(), LayoutError> {
        let start_index = self.blocks.len();
        let level = super::lists::level(&list.attributes, fallback_level);
        let geometry = ListGeometry::default().apply(&list.attributes);
        let ordered = list.kind == ElementKind::OrderedList;
        let mut cursor = ListCursor::new(list, level, geometry);
        for child in &list.children {
            let Content::Element(item) = child else {
                continue;
            };
            if item.kind != ElementKind::ListItem {
                continue;
            }
            let item_level = super::lists::level(&item.attributes, level);
            let item_geometry = geometry.item(&item.attributes, item_level);
            if !item_geometry.hanging.is_finite() || item_geometry.hanging <= 0.0 {
                return Err(LayoutError::InvalidGeometry(
                    "list marker slot width must be positive and finite",
                ));
            }
            let mut attributes = item
                .children
                .iter()
                .find_map(|child| match child {
                    Content::Element(paragraph) if paragraph.kind == ElementKind::Paragraph => {
                        Some(paragraph.attributes.clone())
                    }
                    _ => None,
                })
                .unwrap_or_default();
            attributes.extend(item.attributes.clone());
            let mut unit_index = 0;
            for unit in list_item_units(item, &attributes, self.in_cell) {
                let (unit_attributes, children) = match unit {
                    ListItemUnit::Paragraph(attributes, children) => (attributes, children),
                    ListItemUnit::Nested(nested) => {
                        self.list(nested, item_level.saturating_add(1), false)?;
                        continue;
                    }
                };
                let mut attributes = attributes.clone();
                attributes.extend(unit_attributes);
                let element = Element {
                    kind: ElementKind::Paragraph,
                    attributes,
                    children,
                };
                let mut style = self.styles.paragraph(&element);
                style.indent_left = item_geometry.left;
                if !self.in_cell || !explicit_spacing(&element.attributes, "after") {
                    style.spacing_after = item_geometry.after;
                }
                if self.in_cell {
                    style.first_line = 0.0;
                    if !explicit_before(&element.attributes) {
                        style.spacing_before = 0.0;
                    }
                } else {
                    style.widow_control = false;
                }
                let mut runs = resolve_inline(&element.children, &style.text, self.styles);
                if runs.iter().all(|run| run.text.is_empty()) {
                    if runs.is_empty() {
                        runs.push(TextRun::new("", style.text.clone()));
                    }
                    runs[0].text = " ".into();
                    runs[0].preserve_space = true;
                }
                let task = item.attributes.contains_key("checked");
                let kind = if task {
                    ListMarkerKind::Task
                } else if ordered {
                    ListMarkerKind::Ordered
                } else {
                    ListMarkerKind::Bullet
                };
                let marker_text = if task || unit_index > 0 {
                    String::new()
                } else if ordered {
                    cursor.next(&mut self.numbers.lists, item_level)?
                } else {
                    "•".into()
                };
                let text = if task {
                    None
                } else {
                    let mut marker_style = style.clone();
                    marker_style.alignment = "left".into();
                    marker_style.indent_left = 0.0;
                    marker_style.indent_right = 0.0;
                    marker_style.first_line = 0.0;
                    let measured = self
                        .measurer
                        .measure(&style.text.measurement, &marker_text, true)?
                        .measurement
                        .layout_width;
                    let paragraph = plan_resolved_paragraph(
                        marker_style,
                        None,
                        &[TextRun::new(marker_text, style.text.clone())],
                        item_geometry.hanging.max(measured).max(1.0),
                        self.measurer,
                        true,
                    )?;
                    paragraph.lines.into_iter().next().map(|mut line| {
                        line.highlights.clear();
                        line.decorations.clear();
                        line
                    })
                };
                let font_size = if style.text.measurement.size_points > 0.0 {
                    style.text.measurement.size_points
                } else {
                    11.0
                };
                let paragraph = plan_resolved_paragraph(
                    style,
                    self.styles.border_color(&element),
                    &runs,
                    self.width,
                    self.measurer,
                    true,
                )?;
                self.blocks
                    .push(Block::Paragraph(Box::new(PreparedParagraph {
                        anchors: {
                            let mut anchors = std::mem::take(&mut self.pending_anchors);
                            anchors.extend(super::references::leading_names(&element.children));
                            anchors
                        },
                        text: paragraph,
                        marker: (unit_index == 0).then_some(PreparedMarker {
                            level: item_level,
                            hanging: item_geometry.hanging,
                            marker: ListMarker {
                                kind,
                                checked: values::boolean(&item.attributes, "checked"),
                                slot: [0.0; 4],
                                baseline: 0.0,
                                font_size,
                                text,
                            },
                        }),
                        clear_fill_before: false,
                        clear_fill_after: false,
                    })));
                unit_index += 1;
            }
        }
        if top_level {
            if let Some(Block::Paragraph(paragraph)) = self.blocks.get_mut(start_index) {
                paragraph.clear_fill_before = true;
            }
            if self.blocks.len() > start_index
                && let Some(Block::Paragraph(paragraph)) = self.blocks.last_mut()
            {
                paragraph.clear_fill_after = true;
            }
        }
        Ok(())
    }
}
fn explicit_before(properties: &Properties) -> bool {
    properties.contains_key("spacing_before")
        || matches!(properties.get("spacing"),Some(Value::Object(spacing))if spacing.contains_key("before")||spacing.contains_key("spacing_before"))
}

fn explicit_spacing(properties: &Properties, name: &str) -> bool {
    properties.contains_key(&format!("spacing_{name}"))
        || matches!(properties.get("spacing"),Some(Value::Object(spacing)) if spacing.contains_key(name)||spacing.contains_key(&format!("spacing_{name}")))
}

/// Preserve block order within cell list items while retaining ordinary document list flow.
enum ListItemUnit<'source> {
    Paragraph(Properties, Vec<Content>),
    Nested(&'source Element),
}
fn list_item_units<'source>(
    item: &'source Element,
    attributes: &Properties,
    in_cell: bool,
) -> Vec<ListItemUnit<'source>> {
    let mut output = Vec::new();
    let mut pending = Vec::new();
    let mut nested_lists = Vec::new();
    for child in &item.children {
        if let Content::Element(nested) = child
            && matches!(nested.kind, ElementKind::List | ElementKind::OrderedList)
        {
            if in_cell {
                if output.is_empty()
                    || pending
                        .iter()
                        .any(|node| !matches!(node, Content::Text(text) if text.trim().is_empty()))
                {
                    output.extend(
                        crate::tables::paragraph_units(&pending).into_iter().map(
                            |(attributes, content)| ListItemUnit::Paragraph(attributes, content),
                        ),
                    );
                }
                pending.clear();
                output.push(ListItemUnit::Nested(nested));
            } else {
                nested_lists.push(nested);
            }
        } else {
            pending.push(child.clone());
        }
    }
    if in_cell {
        if output.is_empty()
            || pending
                .iter()
                .any(|node| !matches!(node, Content::Text(text) if text.trim().is_empty()))
        {
            output.extend(
                crate::tables::paragraph_units(&pending)
                    .into_iter()
                    .map(|(attributes, content)| ListItemUnit::Paragraph(attributes, content)),
            );
        }
    } else {
        output.push(ListItemUnit::Paragraph(attributes.clone(), pending));
        output.extend(nested_lists.into_iter().map(ListItemUnit::Nested));
    }
    output
}
