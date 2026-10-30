use super::{BTreeSet, Properties, unknown};

pub(super) fn warnings(
    attributes: &Properties,
    path: &str,
    flattened: bool,
    warnings: &mut BTreeSet<String>,
) -> &'static [&'static str] {
    const ALLOWED: &[&str] = &[
        "kind",
        "data",
        "title",
        "width",
        "height",
        "legend",
        "axis",
        "colors",
        "background",
        "border",
        "plot_area",
        "gridlines",
        "data_labels",
        "gap_width",
        "overlap",
        "name",
    ];
    if flattened {
        warnings.insert(format!(
            "{path} chart geometry is omitted from flattened native stories"
        ));
    }
    for (name, fields) in [
        ("legend", &["enabled", "position"][..]),
        ("border", &["enabled", "color", "width"][..]),
        ("gridlines", &["major", "minor", "color", "width"][..]),
        (
            "data_labels",
            &[
                "enabled",
                "value",
                "category",
                "series",
                "percent",
                "percentage",
                "position",
            ][..],
        ),
        ("plot_area", &["background", "fill", "color", "border"][..]),
        ("axis", &["category", "value"][..]),
    ] {
        if let Some(properties) = crate::properties::object(attributes, name) {
            unknown(properties, fields, &format!("{path}.{name}"), warnings);
        }
    }
    if let Some(plot) = crate::properties::object(attributes, "plot_area")
        && let Some(border) = crate::properties::object(plot, "border")
    {
        unknown(
            border,
            &["enabled", "color", "width"],
            &format!("{path}.plot_area.border"),
            warnings,
        );
    }
    if let Some(axis) = crate::properties::object(attributes, "axis") {
        for name in ["category", "value"] {
            if let Some(properties) = crate::properties::object(axis, name) {
                unknown(
                    properties,
                    &["title", "name"],
                    &format!("{path}.axis.{name}"),
                    warnings,
                );
            }
        }
    }
    kind_limits(attributes, path, warnings);
    ALLOWED
}

fn kind_limits(attributes: &Properties, path: &str, warnings: &mut BTreeSet<String>) {
    let kind = crate::properties::text(attributes, "kind");
    let kind = kind.as_str();
    let labels = crate::charts::appearance::DataLabels::from_value(attributes.get("data_labels"));
    if matches!(kind, "area" | "stacked_area" | "percent_stacked_area") && labels.enabled {
        warnings.insert(format!(
            "{path}.data_labels are omitted from native area charts"
        ));
    }
    if kind == "line" && labels.enabled && !labels.position.is_empty() && labels.position != "t" {
        warnings.insert(format!(
            "{path}.data_labels.position uses the fixed position above native line markers"
        ));
    }
    if matches!(
        kind,
        "line" | "area" | "stacked_area" | "percent_stacked_area"
    ) {
        for name in ["gap_width", "overlap"] {
            if attributes.contains_key(name) {
                warnings.insert(format!(
                    "{path}.{name} has no effect on native {kind} charts"
                ));
            }
        }
    }
}
