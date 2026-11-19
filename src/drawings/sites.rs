//! Connection sites: the points of a preset shape at which a connector meets
//! its outline, as the `cxnLst` of each definition in ECMA-376
//! presetShapeDefinitions.xml gives them for the default adjustments.
use crate::{
    layout::drawings::geometry::{rectangle, transform},
    model::Properties,
    properties::text,
};

/// A connection site: the direction in which it faces, in degrees clockwise
/// from the right, and its place within the frame of its shape.
struct Site {
    facing: u16,
    point: [f64; 2],
}

/// Returns the connection sites of a shape of the kind `kind` and of the size
/// `width` by `height`, in the order of its definition, which is the order
/// that numbers them in DrawingML.
fn sites(kind: &str, width: f64, height: f64) -> Vec<Site> {
    let (center, middle) = (width / 2.0, height / 2.0);
    let shorter = width.min(height);
    let site = |facing, x, y| Site {
        facing,
        point: [x, y],
    };
    match kind {
        "ellipse" => {
            let horizontal = center * std::f64::consts::FRAC_1_SQRT_2;
            let vertical = middle * std::f64::consts::FRAC_1_SQRT_2;
            vec![
                site(270, center, 0.0),
                site(270, center - horizontal, middle - vertical),
                site(180, 0.0, middle),
                site(90, center - horizontal, middle + vertical),
                site(90, center, height),
                site(90, center + horizontal, middle + vertical),
                site(0, width, middle),
                site(270, center + horizontal, middle - vertical),
            ]
        }
        "triangle" => vec![
            site(270, center, 0.0),
            site(180, width / 4.0, middle),
            site(90, 0.0, height),
            site(90, center, height),
            site(90, width, height),
            site(0, width * 3.0 / 4.0, middle),
        ],
        "hexagon" => {
            let inset = shorter / 4.0;
            vec![
                site(0, width, middle),
                site(90, width - inset, height),
                site(90, inset, height),
                site(180, 0.0, middle),
                site(270, inset, 0.0),
                site(270, width - inset, 0.0),
            ]
        }
        "chevron" => {
            let notch = shorter / 2.0;
            let across = (width - notch) / 2.0;
            vec![
                site(270, across, 0.0),
                site(180, notch, middle),
                site(90, across, height),
                site(0, width, middle),
            ]
        }
        "cloud" => vec![
            site(0, width * 21582.0 / 21600.0, middle),
            site(90, center, height * 21577.0 / 21600.0),
            site(180, width * 67.0 / 21600.0, middle),
            site(270, center, height * 1235.0 / 21600.0),
        ],
        "parallelogram" => {
            let slant = shorter / 4.0;
            let lower = (width - slant) / 2.0;
            vec![
                site(270, center, 0.0),
                site(270, width - lower, 0.0),
                site(0, width - slant / 2.0, middle),
                site(90, lower, height),
                site(90, center, height),
                site(180, slant / 2.0, middle),
            ]
        }
        "trapezoid" => vec![
            site(270, center, 0.0),
            site(180, shorter / 8.0, middle),
            site(90, center, height),
            site(0, width - shorter / 8.0, middle),
        ],
        _ => vec![
            site(270, center, 0.0),
            site(180, 0.0, middle),
            site(90, center, height),
            site(0, width, middle),
        ],
    }
}

/// Returns the number of a connection site of the shape with the attributes
/// `shape`, and its place, turned and flipped with the shape. The endpoint
/// `endpoint` names the site by its number `site`, or by the side `side` of
/// the frame, which takes the site that faces that way nearest the middle of
/// that side.
pub(crate) fn locate(
    shape: &Properties,
    endpoint: &Properties,
) -> Result<(usize, [f64; 2]), String> {
    let bounds = rectangle(shape);
    let sites = sites(&text(shape, "kind"), bounds[2], bounds[3]);
    let index = match crate::properties::number(endpoint, "site") {
        Some(number) => {
            let index = number as usize;
            if index >= sites.len() {
                return Err(format!(
                    "site {index} does not exist; the shape has sites 0 to {}",
                    sites.len() - 1
                ));
            }
            index
        }
        None => {
            let (facing, middle) = match text(endpoint, "side").as_str() {
                "top" => (270, [bounds[2] / 2.0, 0.0]),
                "bottom" => (90, [bounds[2] / 2.0, bounds[3]]),
                "left" => (180, [0.0, bounds[3] / 2.0]),
                _ => (0, [bounds[2], bounds[3] / 2.0]),
            };
            let distance =
                |site: &Site| (site.point[0] - middle[0]).hypot(site.point[1] - middle[1]);
            (0..sites.len())
                .filter(|index| sites[*index].facing == facing)
                .min_by(|first, second| {
                    distance(&sites[*first]).total_cmp(&distance(&sites[*second]))
                })
                .unwrap_or_default()
        }
    };
    let point = [
        bounds[0] + sites[index].point[0],
        bounds[1] + sites[index].point[1],
    ];
    Ok((index, transform(vec![point], bounds, shape)[0]))
}
