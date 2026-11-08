//! Keep radial labels inside the circle, or arrange them in two clear columns.
use super::geometry::Rectangle;

pub(super) struct RadialLabel {
    pub text: String,
    pub middle: f64,
    pub sweep: f64,
    pub bounds: Rectangle,
    pub outside: bool,
}

pub(super) fn arrange(
    labels: &mut [RadialLabel],
    center: [f64; 2],
    plot: Rectangle,
    radius: f64,
    doughnut: bool,
) -> f64 {
    // Office sets doughnut labels within the ring even where they cross its edges.
    let clear = labels.iter().enumerate().all(|(index, label)| {
        (doughnut || fits_circle(label, center, radius))
            && labels[..index]
                .iter()
                .all(|earlier| !overlap(label.bounds, earlier.bounds))
    });
    if clear {
        return radius;
    }
    let width = labels
        .iter()
        .map(|label| label.bounds.width)
        .fold(0.0, f64::max)
        .min(plot.width * 0.3);
    let radius = radius.min(((plot.width - 2.0 * (width + 8.0)) / 2.0).max(1.0));
    for right in [false, true] {
        let mut order: Vec<_> = labels
            .iter()
            .enumerate()
            .filter(|(_, label)| (label.middle.cos() >= -1e-9) == right)
            .map(|(index, _)| index)
            .collect();
        order.sort_by(|&first, &second| {
            labels[first]
                .middle
                .sin()
                .total_cmp(&labels[second].middle.sin())
                .then(first.cmp(&second))
        });
        if order.is_empty() {
            continue;
        }
        let gap = 4.0_f64.min(plot.height / (2.0 * order.len() as f64));
        let total_height: f64 = order.iter().map(|&index| labels[index].bounds.height).sum();
        let scale = ((plot.height - gap * (order.len() - 1) as f64) / total_height).min(1.0);
        let mut top = plot.top;
        for &index in &order {
            let label = &mut labels[index];
            let height = label.bounds.height * scale;
            let desired = center[1] + label.middle.sin() * radius - height / 2.0;
            label.bounds = Rectangle::new(
                if right {
                    center[0] + radius + 8.0
                } else {
                    center[0] - radius - width - 8.0
                },
                desired.max(top),
                width,
                height,
            );
            label.outside = true;
            top = label.bounds.top + height + gap;
        }
        let mut bottom = plot.top + plot.height;
        for &index in order.iter().rev() {
            let bounds = &mut labels[index].bounds;
            bounds.top = bounds.top.min(bottom - bounds.height);
            bottom = bounds.top - gap;
        }
    }
    radius
}

// A pie label may cross into a neighbouring sector, as Office's best fit allows.
fn fits_circle(label: &RadialLabel, center: [f64; 2], radius: f64) -> bool {
    if label.sweep <= 0.0 {
        return false;
    }
    let bounds = label.bounds;
    [bounds.left, bounds.left + bounds.width]
        .into_iter()
        .flat_map(|horizontal| {
            [bounds.top, bounds.top + bounds.height]
                .map(|vertical| (horizontal - center[0]).hypot(vertical - center[1]))
        })
        .all(|distance| distance <= radius - 1.5 + 1e-9)
}

fn overlap(first: Rectangle, second: Rectangle) -> bool {
    first.left < second.left + second.width + 2.0
        && second.left < first.left + first.width + 2.0
        && first.top < second.top + second.height + 2.0
        && second.top < first.top + first.height + 2.0
}
