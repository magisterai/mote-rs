use crate::charts::Chart;

#[derive(Clone, Copy)]
pub(super) struct Scale {
    pub minimum: f64,
    pub maximum: f64,
    pub step: f64,
    /// A scale with a stated bound may exclude data, and its marks are then
    /// confined to the plot. An automatic scale contains all of its data.
    pub clips: bool,
}
impl Scale {
    fn automatic(minimum: f64, maximum: f64, step: f64) -> Self {
        Self {
            minimum,
            maximum,
            step,
            clips: false,
        }
    }
    pub fn ratio(self, value: f64) -> f64 {
        let span = self.maximum - self.minimum;
        if span <= 0.0 {
            0.0
        } else {
            (value - self.minimum) / span
        }
    }
    pub fn visible_ratio(self, value: f64) -> f64 {
        let ratio = self.ratio(value);
        if self.clips {
            ratio.clamp(0.0, 1.0)
        } else {
            ratio
        }
    }
    pub fn shows(self, lower: f64, upper: f64) -> bool {
        !self.clips || (upper >= self.minimum && lower <= self.maximum)
    }
    /// Office places the category axis at zero, or at the bound nearest zero
    /// when the range excludes zero.
    pub fn crossing(self) -> f64 {
        0.0_f64.clamp(self.minimum, self.maximum)
    }
    /// The ticks of a stated range start at its minimum, which need not be a
    /// multiple of the step, and a stated step need not be a single digit. A
    /// label then needs the digits of both.
    pub fn label_unit(self) -> f64 {
        let whole = |quotient: f64| (quotient - quotient.round()).abs() < 1e-6;
        let mut unit = 10.0_f64.powf(self.step.log10().floor());
        if !unit.is_finite() || unit <= 0.0 || whole(self.step / unit) {
            unit = self.step;
        }
        for _ in 0..6 {
            if whole(self.minimum / unit) && whole(self.step / unit) {
                break;
            }
            unit /= 10.0;
        }
        unit
    }
    pub fn for_chart(chart: &Chart, length: f64, horizontal: bool) -> Self {
        let automatic = Self::automatic_for_chart(chart, length, horizontal);
        if (chart.value_minimum.is_none()
            && chart.value_maximum.is_none()
            && chart.value_step.is_none())
            || automatic.validate().is_err()
        {
            return automatic;
        }
        // The step of a stated range allows for enlarged labels as the automatic
        // step does.
        automatic.with_stated_range(
            chart.value_minimum,
            chart.value_maximum,
            chart.value_step,
            length / chart.label_scale(),
            horizontal,
        )
    }
    /// A stated bound replaces the automatic one, and the step follows from the
    /// resulting range by the automatic rule. An unstated bound keeps its
    /// automatic value, moved outward to a whole number of steps; if the stated
    /// bound lies beyond that value, the automatic span is kept instead.
    fn with_stated_range(
        self,
        stated_minimum: Option<f64>,
        stated_maximum: Option<f64>,
        stated_step: Option<f64>,
        length: f64,
        horizontal: bool,
    ) -> Self {
        let span = self.maximum - self.minimum;
        let minimum = stated_minimum.unwrap_or_else(|| {
            let maximum = stated_maximum.unwrap_or(self.maximum);
            if self.minimum < maximum {
                self.minimum
            } else {
                maximum - span
            }
        });
        let maximum = stated_maximum.unwrap_or(if self.maximum > minimum {
            self.maximum
        } else {
            minimum + span
        });
        let step = stated_step.unwrap_or_else(|| {
            if horizontal {
                horizontal_step(maximum - minimum, length)
            } else {
                step_at_least((maximum - minimum) / vertical_parts(length))
            }
        });
        Self {
            minimum: if stated_minimum.is_some() {
                minimum
            } else {
                (minimum / step + 1e-9).floor() * step
            },
            maximum: if stated_maximum.is_some() {
                maximum
            } else {
                minimum + ((maximum - minimum) / step - 1e-9).ceil() * step
            },
            step,
            clips: stated_minimum.is_some() || stated_maximum.is_some(),
        }
    }
    fn automatic_for_chart(chart: &Chart, length: f64, horizontal: bool) -> Self {
        let stacked = chart.kind.contains("stacked");
        let percent = chart.kind.contains("percent_stacked");
        let (minimum, maximum) = extents(chart, stacked);
        if !minimum.is_finite() || !maximum.is_finite() {
            return Self::automatic(minimum, maximum, 0.0);
        }
        // The step rules were measured for default labels, so a larger label
        // sees a proportionally shorter axis.
        let length = length / chart.label_scale();
        if percent {
            let (minimum, maximum, step) = match (minimum < 0.0, maximum > 0.0) {
                (true, true) => (-100.0, 100.0, 20.0),
                (true, false) => (-100.0, 0.0, if horizontal { 20.0 } else { 10.0 }),
                _ => (0.0, 100.0, if horizontal { 20.0 } else { 10.0 }),
            };
            // A default chart keeps its percentage step at every height; larger
            // labels on a vertical axis take the finest step that separates them.
            let enlarged =
                !horizontal && chart.label_size > crate::charts::DEFAULT_LABEL_SIZE && length > 0.0;
            let step = [step, 20.0, 50.0]
                .into_iter()
                .find(|candidate| {
                    *candidate >= step
                        && (!enlarged || length * candidate / (maximum - minimum) >= 12.5)
                })
                .unwrap_or(100.0);
            return Self {
                minimum,
                maximum,
                step,
                clips: false,
            };
        }
        if !horizontal {
            return vertical_scale(minimum, maximum, length);
        }
        if minimum >= 0.0 {
            let (maximum, step) = positive_scale(maximum, length);
            return Self::automatic(0.0, maximum, step);
        }
        if maximum <= 0.0 {
            let (magnitude, step) = positive_scale(-minimum, 0.0);
            return Self::automatic(-magnitude, 0.0, step);
        }
        let step = nice_step((maximum - minimum) / 6.0);
        let mut lower = (minimum / step).floor() * step;
        let mut upper = (maximum / step).ceil() * step;
        if lower != 0.0 && (minimum / lower).abs() > 0.95 {
            lower -= step;
        }
        if upper != 0.0 && maximum / upper > 0.95 {
            upper += step;
        }
        Self::automatic(lower, upper, step)
    }
    pub fn validate(self) -> Result<Self, super::LayoutError> {
        if !self.minimum.is_finite()
            || !self.maximum.is_finite()
            || !self.step.is_finite()
            || self.step <= 0.0
            || self.maximum <= self.minimum
            || !(self.maximum - self.minimum).is_finite()
        {
            return Err(super::LayoutError::InvalidGeometry(
                "chart scale must be finite and increasing",
            ));
        }
        if (self.maximum - self.minimum) / self.step > crate::charts::MAXIMUM_VALUE_INTERVALS + 1e-9
        {
            return Err(super::LayoutError::InvalidGeometry(
                "chart axis.value.step divides the value range into more than 100 intervals",
            ));
        }
        Ok(self)
    }
}
fn extents(chart: &Chart, stacked: bool) -> (f64, f64) {
    let mut minimum: f64 = 0.0;
    let mut maximum: f64 = 0.0;
    if stacked {
        for index in 0..chart.categories.len() {
            let mut negative = 0.0;
            let mut positive = 0.0;
            for series in &chart.series {
                let value = super::series_value(series, index);
                if value < 0.0 {
                    negative += value;
                } else {
                    positive += value;
                }
            }
            minimum = minimum.min(negative);
            maximum = maximum.max(positive);
        }
    } else {
        for value in chart.series.iter().flat_map(|series| &series.values) {
            minimum = minimum.min(*value);
            maximum = maximum.max(*value);
        }
    }
    (minimum, maximum)
}
/// Office pads each nonzero end of the data by a twentieth of its range, then
/// takes the finest step of one, two or five times a power of ten that divides
/// the padded span into no more parts than the plot height allows. The limit on
/// the parts lies midway in the range that Word's chart-value-axis probe admits.
fn vertical_scale(minimum: f64, maximum: f64, length: f64) -> Scale {
    let margin = (maximum - minimum) / 20.0;
    let top = if maximum > 0.0 { maximum + margin } else { 0.0 };
    let bottom = if minimum < 0.0 { minimum - margin } else { 0.0 };
    if top - bottom <= 0.0 {
        return Scale::automatic(0.0, 1.0, 0.2);
    }
    let step = step_at_least((top - bottom) / vertical_parts(length));
    Scale::automatic(
        (bottom / step).floor() * step,
        (top / step).ceil() * step,
        step,
    )
}
fn vertical_parts(length: f64) -> f64 {
    if length > 0.0 {
        ((length - 16.0) * 0.085).clamp(1.0, 10.0)
    } else {
        10.0
    }
}
/// The step of a stated horizontal range follows the rule of `positive_scale`
/// with the range fixed: about six intervals, coarser while an interval is
/// narrower than 12.5 points, and finer by one grade if that gives from five to
/// ten intervals of sufficient width.
fn horizontal_step(span: f64, length: f64) -> f64 {
    let mut step = nice_step(span / 6.0);
    if length <= 0.0 {
        return step;
    }
    let wide_enough = |step: f64| length / (span / step).round().max(1.0) >= 12.5;
    while !wide_enough(step) {
        let next = nice_step(step * 2.0);
        if next <= step {
            break;
        }
        step = next;
    }
    let candidate = nice_step(step / 2.0);
    if candidate < step
        && (5.0..=10.0).contains(&(span / candidate).round())
        && wide_enough(candidate)
    {
        step = candidate;
    }
    step
}
fn step_at_least(rough: f64) -> f64 {
    let power = 10.0_f64.powf(rough.log10().floor());
    let normalized = rough / power;
    let selected = [1.0, 2.0, 5.0]
        .into_iter()
        .find(|candidate| normalized <= candidate * (1.0 + 1e-9))
        .unwrap_or(10.0);
    selected * power
}
fn positive_scale(maximum: f64, length: f64) -> (f64, f64) {
    if maximum <= 0.0 || !maximum.is_finite() {
        return (1.0, 0.2);
    }
    let upper = |step: f64| {
        let mut upper = (maximum / step).ceil() * step;
        if maximum / upper > 0.95 {
            upper += step;
        }
        upper
    };
    let mut step = nice_step(maximum / 6.0);
    let mut limit = upper(step);
    if length <= 0.0 || step <= 0.0 {
        return (limit, step);
    }
    loop {
        let intervals = (limit / step).round();
        if intervals <= 0.0 || length / intervals >= 12.5 {
            break;
        }
        let next = nice_step(step * 2.0);
        if next <= step {
            break;
        }
        step = next;
        limit = upper(step);
    }
    let candidate = nice_step(step / 2.0);
    if candidate >= step {
        return (limit, step);
    }
    let candidate_limit = upper(candidate);
    let intervals = (candidate_limit / candidate).round();
    if !(5.0..=10.0).contains(&intervals) || length / intervals < 12.5 {
        return (limit, step);
    }
    (candidate_limit, candidate)
}
fn nice_step(rough: f64) -> f64 {
    if rough <= 0.0 || !rough.is_finite() {
        return 0.2;
    }
    let power = 10.0_f64.powf(rough.log10().floor());
    let normalized = rough / power;
    let selected = if normalized >= 50.0_f64.sqrt() {
        10.0
    } else if normalized >= 10.0_f64.sqrt() {
        5.0
    } else if normalized >= 2.0_f64.sqrt() {
        2.0
    } else {
        1.0
    };
    selected * power
}
pub(super) fn axis_label(value: f64, step: f64, percent: bool) -> String {
    let magnitude = value.abs();
    let step_exponent = step.log10().floor();
    let scientific = magnitude >= 1_000_000.0 || (magnitude > 0.0 && magnitude < 0.0001);
    let precision = if scientific {
        (magnitude.log10().floor() - step_exponent).clamp(0.0, 15.0) as usize
    } else {
        (-step_exponent).clamp(0.0, 15.0) as usize
    };
    let formatted = if scientific {
        format!("{value:.precision$e}")
    } else {
        format!("{value:.precision$}")
    };
    let (mantissa, exponent) = formatted.split_once('e').unwrap_or((&formatted, ""));
    let mut label = if mantissa.contains('.') {
        mantissa
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_owned()
    } else {
        mantissa.to_owned()
    };
    if label == "-0" {
        label = "0".into();
    }
    if !exponent.is_empty() {
        label.push('e');
        label.push_str(&exponent.parse::<i32>().unwrap().to_string());
    }
    if percent {
        label.push('%');
    }
    label
}

#[cfg(test)]
mod tests {
    use super::axis_label;

    #[test]
    fn axis_precision_removes_arithmetic_noise_without_rounding_away_ticks() {
        for (value, step, expected) in [
            (0.6000000000000001, 0.2, "0.6"),
            (-0.6000000000000001, 0.2, "-0.6"),
            (-0.0, 0.2, "0"),
            (0.00012, 0.00002, "0.00012"),
            (0.00000012, 0.00000002, "1.2e-7"),
            (1_200_000.0, 200_000.0, "1.2e6"),
            (1.2e100, 2.0e99, "1.2e100"),
            (1.2e-100, 2.0e-101, "1.2e-100"),
        ] {
            assert_eq!(axis_label(value, step, false), expected);
        }
        assert_eq!(axis_label(-40.00000000000001, 20.0, true), "-40%");
    }
}
