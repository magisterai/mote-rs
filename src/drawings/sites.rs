//! Connection sites: the points of a preset shape at which a connector meets
//! its outline, as the `cxnLst` of its definition gives them.
use crate::{
    layout::drawings::geometry::{preset, rectangle, transform},
    model::Properties,
    properties::text,
};

/// Returns the number of a connection site of the shape with the attributes
/// `shape`, and its place, turned and flipped with the shape. The endpoint
/// `endpoint` names the site by its number `site`, or by the side `side` of
/// the frame. A side takes the site nearest its middle among those that face
/// within an eighth of a turn of its direction, or among all the sites where
/// none faces so.
pub(crate) fn locate(
    shape: &Properties,
    endpoint: &Properties,
) -> Result<(usize, [f64; 2]), String> {
    let bounds = rectangle(shape);
    let sites = preset(shape).sites;
    let index = match crate::properties::number(endpoint, "site") {
        Some(number) => {
            let index = number as usize;
            if index >= sites.len() {
                return Err(format!(
                    "site {index} does not exist; the shape has sites 0 to {}",
                    sites.len().saturating_sub(1)
                ));
            }
            index
        }
        None => {
            let (facing, middle) = match text(endpoint, "side").as_str() {
                "top" => (270.0, [bounds[2] / 2.0, 0.0]),
                "bottom" => (90.0, [bounds[2] / 2.0, bounds[3]]),
                "left" => (180.0, [0.0, bounds[3] / 2.0]),
                _ => (0.0, [bounds[2], bounds[3] / 2.0]),
            };
            let faces = |index: &usize| {
                ((sites[*index].facing - facing + 180.0).rem_euclid(360.0) - 180.0).abs() <= 45.0
            };
            let nearest = |indices: &mut dyn Iterator<Item = usize>| {
                indices.min_by(|first, second| {
                    let distance = |index: &usize| {
                        let point = sites[*index].point;
                        (point[0] - middle[0]).hypot(point[1] - middle[1])
                    };
                    distance(first).total_cmp(&distance(second))
                })
            };
            nearest(&mut (0..sites.len()).filter(faces))
                .or_else(|| nearest(&mut (0..sites.len())))
                .ok_or("the shape has no connection sites")?
        }
    };
    let point = [
        bounds[0] + sites[index].point[0],
        bounds[1] + sites[index].point[1],
    ];
    Ok((index, transform(vec![point], bounds, shape)[0]))
}
