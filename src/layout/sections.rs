//! Section flow, physical-page ownership, column balancing, and field stabilization.
use super::{
    DocumentTextPlan, FlowCursor, LayoutError, LayoutStyleResolver, PageGeometry, ParagraphStyle,
    TextPage,
};
use super::{
    blocks::{BlockNumbers, prepare_content},
    columns::ColumnSeparators,
    document::{PlacementContext, ensure_space, place_blocks},
};
use crate::{
    measurement::TextMeasurer,
    model::{Content, Document, Element, ElementKind, Section},
};
use std::collections::BTreeMap;

#[derive(Clone, Copy)]
pub(super) struct FlowBoundary {
    pub page: usize,
    pub bottom: f64,
}
pub(super) fn content_bottom(
    geometry: PageGeometry,
    boundary: Option<FlowBoundary>,
    page: usize,
) -> f64 {
    boundary
        .filter(|boundary| boundary.page == page)
        .map(|boundary| boundary.bottom.min(geometry.content_bottom()))
        .unwrap_or(geometry.content_bottom())
}
#[derive(Clone, Copy)]
struct BalanceTarget {
    page_offset: usize,
    height: f64,
    maximum: f64,
}
struct BalanceSearch {
    target: BalanceTarget,
    low: f64,
    high: f64,
    failed_low: bool,
    done: bool,
}
struct SectionFlow {
    start: FlowCursor,
    end: FlowCursor,
    balance_end: FlowCursor,
    trailing: f64,
    geometry: PageGeometry,
}
struct PassResult {
    plan: DocumentTextPlan,
    fields: BTreeMap<usize, usize>,
    reference_pages: BTreeMap<String, usize>,
    flows: Vec<SectionFlow>,
}
#[derive(Default)]
struct PassInputs {
    fields: BTreeMap<usize, usize>,
    reference_pages: BTreeMap<String, usize>,
    total: usize,
    targets: BTreeMap<usize, BalanceTarget>,
}

pub(super) fn plan(
    document: &Document,
    measurer: &mut TextMeasurer<'_>,
    images: &crate::media::ImageCatalog,
) -> Result<DocumentTextPlan, LayoutError> {
    let sections = if document.sections.is_empty() {
        vec![Section {
            page: document.page.clone(),
            margin: document.margin.clone(),
            columns: document.columns.clone(),
            content: document.content.clone(),
            ..Default::default()
        }]
    } else {
        document.sections.clone()
    };
    let mut inputs = PassInputs {
        total: 1,
        ..Default::default()
    };
    for pass in 0..6 {
        let mut result = balanced_pass(document, &sections, measurer, images, &mut inputs)?;
        if inputs.fields == result.fields
            && inputs.reference_pages == result.reference_pages
            && inputs.total == result.plan.pages.len().max(1)
            || pass == 5
        {
            if pass == 5 {
                result
                    .plan
                    .warnings
                    .push("document page fields did not stabilize after 6 layout passes".into());
            }
            super::stories::place(
                &mut result.plan,
                &sections,
                document,
                measurer,
                &result.reference_pages,
            )?;
            super::references::finalize(&mut result.plan);
            return Ok(result.plan);
        }
        inputs.total = result.plan.pages.len().max(1);
        inputs.fields = result.fields;
        inputs.reference_pages = result.reference_pages;
    }
    unreachable!()
}
fn balanced_pass(
    document: &Document,
    sections: &[Section],
    measurer: &mut TextMeasurer<'_>,
    images: &crate::media::ImageCatalog,
    inputs: &mut PassInputs,
) -> Result<PassResult, LayoutError> {
    inputs.targets.clear();
    let mut best = plan_pass(document, sections, measurer, images, inputs)?;
    let styles = LayoutStyleResolver::new(document);
    let boundary_style = boundary_style(&styles);
    let mut searches = BTreeMap::new();
    for (index, flow) in best.flows.iter().enumerate() {
        if index + 1 == sections.len()
            || flow.geometry.column_count < 2
            || has_column_break(&sections[index].content)
            || sections[index]
                .columns
                .properties
                .get("equal_width")
                .is_some_and(|value| value.to_text() == "false")
        {
            continue;
        }
        let page_offset = flow.end.page_number - flow.start.page_number;
        let top = if page_offset == 0 {
            flow.start.y
        } else {
            flow.geometry.margin_top
        };
        let maximum = flow.geometry.content_bottom() - top;
        let end_height = flow.end.y - top
            + flow.trailing.max(boundary_style.spacing_before)
            + boundary_style.line_height
            + boundary_style.spacing_after;
        let height = (flow.end.column_index as f64 * maximum + end_height)
            / flow.geometry.column_count as f64;
        if height <= 0.0 || height >= maximum - 0.01 {
            continue;
        }
        searches.insert(
            index,
            BalanceSearch {
                target: BalanceTarget {
                    page_offset,
                    height,
                    maximum,
                },
                low: height,
                high: maximum,
                failed_low: false,
                done: false,
            },
        );
    }
    if searches.is_empty() {
        return Ok(best);
    }
    for _ in 0..16 {
        inputs.targets = searches
            .iter()
            .map(|(index, search)| (*index, search.target))
            .collect();
        let result = plan_pass(document, sections, measurer, images, inputs)?;
        let mut all_fit = true;
        let mut all_done = true;
        let mut changed = false;
        for (index, search) in &mut searches {
            let flow = &result.flows[*index];
            let fits =
                flow.balance_end.page_number - flow.start.page_number <= search.target.page_offset;
            if !fits {
                all_fit = false;
                search.failed_low = true;
                search.low = search.target.height;
                if search.high - search.low <= 0.01 {
                    changed |= search.target.height != search.high;
                    search.target.height = search.high;
                    search.done = true;
                } else {
                    search.target.height = (search.low + search.high) / 2.0;
                    changed = true;
                }
            } else if !search.failed_low {
                search.done = true;
            } else {
                search.high = search.target.height;
                if search.high - search.low <= 0.01 {
                    search.done = true;
                } else {
                    search.target.height = (search.low + search.high) / 2.0;
                    changed = true;
                }
            }
            all_done &= search.done;
        }
        if all_fit {
            best = result;
        }
        if all_done && !changed {
            break;
        }
    }
    Ok(best)
}
fn plan_pass(
    document: &Document,
    sections: &[Section],
    measurer: &mut TextMeasurer<'_>,
    images: &crate::media::ImageCatalog,
    inputs: &PassInputs,
) -> Result<PassResult, LayoutError> {
    let mut styles = LayoutStyleResolver::new(document);
    styles.reference_pages = inputs.reference_pages.clone();
    let mut reference_pages = BTreeMap::new();
    let mut numbers = BlockNumbers::default();
    let mut pages: Vec<TextPage> = Vec::new();
    let mut cursor = FlowCursor::default();
    let mut pending_spacing = 0.0;
    let mut warnings = Vec::new();
    let mut previous_geometry = PageGeometry::from_document(document)?;
    let mut numbering_offset: i64 = 0;
    let mut next_field = 0;
    let mut field_offsets = BTreeMap::new();
    let mut flows = Vec::new();
    for (section_index, section) in sections.iter().enumerate() {
        let geometry = PageGeometry::from_section(section)?;
        let new_page = pages.is_empty()
            || section.start != "continuous"
            || !same_page(geometry, previous_geometry)
            || cursor.y >= geometry.content_bottom();
        let mut columns = ColumnSeparators::default();
        if new_page {
            pages.push(TextPage::default());
            cursor = FlowCursor {
                page_number: pages.len(),
                current_top: geometry.margin_top,
                y: geometry.margin_top,
                ..Default::default()
            };
            pending_spacing = 0.0;
        } else {
            cursor.column_index = 0;
            cursor.current_top = cursor.y.max(geometry.margin_top);
            cursor.y = cursor.current_top;
        }
        let start = cursor;
        let boundary = inputs.targets.get(&section_index).and_then(|target| {
            let top = if target.page_offset == 0 {
                start.y
            } else {
                geometry.margin_top
            };
            (target.height < target.maximum).then_some(FlowBoundary {
                page: start.page_number + target.page_offset,
                bottom: top + target.height,
            })
        });
        columns.begin(&cursor);
        let restart = crate::properties::number(&section.page_numbering, "start").or_else(|| {
            crate::properties::boolean(&section.page_numbering, "restart").then_some(1.0)
        });
        if let Some(number) = restart {
            numbering_offset = number as i64 - cursor.page_number as i64;
        }
        let first_field = next_field;
        let mut content = section.content.clone();
        super::fields::resolve(
            &mut content,
            (cursor.page_number as i64 + numbering_offset).max(1) as usize,
            inputs.total,
            Some(&mut next_field),
            &inputs.fields,
        );
        for identifier in first_field..next_field {
            field_offsets.insert(identifier, numbering_offset);
        }
        let blocks = prepare_content(
            &content,
            geometry.column_width(),
            &styles,
            measurer,
            images,
            &mut numbers,
            false,
        )?;
        place_blocks(
            &blocks,
            PlacementContext {
                styles: &styles,
                geometry,
                boundary,
            },
            &mut pages,
            &mut cursor,
            &mut columns,
            &mut pending_spacing,
            &mut warnings,
        )?;
        let end = cursor;
        let trailing = pending_spacing;
        let mut balance_end = end;
        if let Some(next) = sections.get(section_index + 1) {
            let style = boundary_style(&styles);
            if next.start == "continuous" && same_page(geometry, PageGeometry::from_section(next)?)
            {
                advance_boundary(
                    &mut cursor,
                    &mut pages,
                    &mut columns,
                    geometry,
                    boundary,
                    pending_spacing,
                    &style,
                );
                if let Some(boundary) = boundary
                    && boundary.page == cursor.page_number
                {
                    cursor.y = cursor.y.max(boundary.bottom);
                }
                balance_end = cursor;
                pending_spacing = 0.0;
            } else {
                let mut virtual_pages = vec![TextPage::default(); pages.len()];
                advance_boundary(
                    &mut balance_end,
                    &mut virtual_pages,
                    &mut ColumnSeparators::default(),
                    geometry,
                    boundary,
                    pending_spacing,
                    &style,
                );
            }
        }
        flows.push(SectionFlow {
            start,
            end,
            balance_end,
            trailing,
            geometry,
        });
        columns.append(&mut pages, geometry);
        for (page_index, page) in pages.iter_mut().enumerate() {
            if page.geometry.is_none() {
                page.geometry = Some(geometry);
                page.section_index = section_index;
                page.page_number = (page_index as i64 + 1 + numbering_offset).max(1) as usize;
            }
        }
        for (page_index, page) in pages.iter().enumerate() {
            for name in page.anchors.keys() {
                reference_pages
                    .entry(name.clone())
                    .or_insert((page_index as i64 + 1 + numbering_offset).max(1) as usize);
            }
        }
        previous_geometry = geometry;
    }
    if document.sections.is_empty() && document.content.is_empty() {
        pages.clear();
    }
    let mut fields = BTreeMap::new();
    for (page_index, page) in pages.iter().enumerate() {
        for fragment in page.text_lines().flat_map(|line| &line.fragments) {
            if let Some(identifier) = fragment
                .current_page_field
                .as_ref()
                .and_then(|field| field.identifier)
            {
                fields.insert(
                    identifier,
                    (page_index as i64 + 1 + field_offsets[&identifier]).max(1) as usize,
                );
            }
        }
    }
    Ok(PassResult {
        plan: DocumentTextPlan {
            geometry: pages
                .first()
                .and_then(|page| page.geometry)
                .unwrap_or(previous_geometry),
            pages,
            warnings,
        },
        fields,
        reference_pages,
        flows,
    })
}
fn same_page(first: PageGeometry, second: PageGeometry) -> bool {
    first.width == second.width && first.height == second.height
}
fn boundary_style(styles: &LayoutStyleResolver<'_>) -> ParagraphStyle {
    styles.paragraph(&Element {
        kind: ElementKind::Paragraph,
        attributes: Default::default(),
        children: Vec::new(),
    })
}
fn advance_boundary(
    cursor: &mut FlowCursor,
    pages: &mut Vec<TextPage>,
    columns: &mut ColumnSeparators,
    geometry: PageGeometry,
    boundary: Option<FlowBoundary>,
    pending: f64,
    style: &ParagraphStyle,
) {
    let leading = pending.max(style.spacing_before);
    let previous = (cursor.page_number, cursor.column_index);
    ensure_space(
        cursor,
        pages,
        columns,
        geometry,
        boundary,
        leading + style.line_height,
    );
    if previous == (cursor.page_number, cursor.column_index) {
        cursor.y += leading;
    }
    ensure_space(
        cursor,
        pages,
        columns,
        geometry,
        boundary,
        style.line_height,
    );
    cursor.y += style.line_height;
    cursor.body_started = true;
    columns.note(cursor, geometry);
    let previous = (cursor.page_number, cursor.column_index);
    ensure_space(
        cursor,
        pages,
        columns,
        geometry,
        boundary,
        style.spacing_after,
    );
    if previous == (cursor.page_number, cursor.column_index) {
        cursor.y += style.spacing_after;
    }
}
fn has_column_break(content: &[Content]) -> bool {
    content.iter().any(|child| matches!(child, Content::Element(element) if (element.kind == ElementKind::Break && crate::properties::text(&element.attributes, "type") == "column") || has_column_break(&element.children)))
}
