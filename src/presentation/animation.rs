//! The effects of an animated object and its appearance at a time.
//!
//! A time is a number of seconds from the first showing of a slide. An effect
//! occupies the interval from its start to its start and duration together.
use crate::{
    model::{Element, Properties, Value},
    properties::{number, object, text},
};

/// An edge of the extent of an object.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Edge {
    Left,
    Right,
    Top,
    Bottom,
}

impl Edge {
    fn opposite(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Top => Self::Bottom,
            Self::Bottom => Self::Top,
        }
    }
}

/// The part of an object that a wipe leaves in view: the stated fraction of
/// the extent of the object, measured from the stated edge.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Reveal {
    pub edge: Edge,
    pub fraction: f64,
}

impl Reveal {
    /// Returns the left, top, width and height of the region in which the
    /// object is drawn, for an extent given by its left, top, right and
    /// bottom. The region is bounded at the advancing side of the wipe alone
    /// and passes the extent by the margin on the other sides, since a stroke
    /// lies partly outside the extent and the extent of a straight line has no
    /// breadth.
    pub fn clip(&self, [left, top, right, bottom]: [f64; 4], margin: f64) -> [f64; 4] {
        let horizontal = (right - left) * self.fraction;
        let vertical = (bottom - top) * self.fraction;
        let [left, top, right, bottom] = match self.edge {
            Edge::Left => [
                left - margin,
                top - margin,
                left + horizontal,
                bottom + margin,
            ],
            Edge::Right => [
                right - horizontal,
                top - margin,
                right + margin,
                bottom + margin,
            ],
            Edge::Top => [left - margin, top - margin, right + margin, top + vertical],
            Edge::Bottom => [
                left - margin,
                bottom - vertical,
                right + margin,
                bottom + margin,
            ],
        };
        [left, top, right - left, bottom - top]
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum EffectKind {
    FadeIn,
    FadeOut,
    /// A progressive revealing that begins at the edge.
    WipeIn(Edge),
    /// A progressive hiding that begins at the edge.
    WipeOut(Edge),
    Appear,
    Disappear,
    /// A displacement in drawing units.
    Move([i64; 2]),
    /// A factor of enlargement about the centre of the object.
    Scale(f64),
    /// A clockwise turn in degrees about the centre of the object.
    Rotate(f64),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Effect {
    pub kind: EffectKind,
    pub start: f64,
    pub duration: f64,
    /// The fraction of the duration spent in gathering speed.
    pub acceleration: f64,
    /// The fraction of the duration spent in losing speed.
    pub deceleration: f64,
}

impl Effect {
    pub fn end(&self) -> f64 {
        self.start + self.duration
    }

    /// Returns the fraction of the effect that is complete at a time. The speed
    /// rises uniformly through the acceleration, is constant thereafter, and
    /// falls uniformly through the deceleration, as PowerPoint defines them.
    pub fn progress(&self, time: f64) -> f64 {
        if time < self.start {
            return 0.0;
        }
        if self.duration <= 0.0 || time >= self.end() {
            return 1.0;
        }
        let elapsed = (time - self.start) / self.duration;
        let (acceleration, deceleration) = (self.acceleration, self.deceleration);
        let speed = 1.0 / (1.0 - acceleration / 2.0 - deceleration / 2.0);
        if elapsed < acceleration {
            speed * elapsed * elapsed / (2.0 * acceleration)
        } else if elapsed <= 1.0 - deceleration {
            speed * (elapsed - acceleration / 2.0)
        } else {
            1.0 - speed * (1.0 - elapsed) * (1.0 - elapsed) / (2.0 * deceleration)
        }
    }

    fn concerns_visibility(&self) -> bool {
        matches!(
            self.kind,
            EffectKind::FadeIn
                | EffectKind::FadeOut
                | EffectKind::WipeIn(_)
                | EffectKind::WipeOut(_)
                | EffectKind::Appear
                | EffectKind::Disappear
        )
    }
}

/// The names of the effects, as the evaluator admits them.
pub(crate) const EFFECT_NAMES: [&str; 9] = [
    "fade_in",
    "fade_out",
    "wipe_in",
    "wipe_out",
    "appear",
    "disappear",
    "move",
    "scale",
    "rotate",
];
pub(crate) const EASING_NAMES: [&str; 4] = ["linear", "ease_in", "ease_out", "ease_in_out"];
pub(crate) const EDGE_NAMES: [&str; 4] = ["left", "right", "top", "bottom"];
pub(crate) const DEFAULT_DURATION: f64 = 0.5;

/// Reads the normalized `animate` attribute of an object, in order of start.
pub(crate) fn effects(attributes: &Properties) -> Vec<Effect> {
    let Some(Value::Array(entries)) = attributes.get("animate") else {
        return Vec::new();
    };
    let mut output: Vec<Effect> = entries
        .iter()
        .filter_map(|entry| {
            let Value::Object(fields) = entry else {
                return None;
            };
            let displacement = |name| {
                object(fields, "by")
                    .and_then(|by| super::units(by.get(name)))
                    .unwrap_or(0)
            };
            let edge = match text(fields, "from").as_str() {
                "right" => Edge::Right,
                "top" => Edge::Top,
                "bottom" => Edge::Bottom,
                _ => Edge::Left,
            };
            let kind = match text(fields, "effect").as_str() {
                "fade_in" => EffectKind::FadeIn,
                "fade_out" => EffectKind::FadeOut,
                "wipe_in" => EffectKind::WipeIn(edge),
                "wipe_out" => EffectKind::WipeOut(edge),
                "appear" => EffectKind::Appear,
                "disappear" => EffectKind::Disappear,
                "move" => EffectKind::Move([displacement("x"), displacement("y")]),
                "scale" => EffectKind::Scale(number(fields, "to").unwrap_or(1.0)),
                "rotate" => EffectKind::Rotate(number(fields, "by").unwrap_or(0.0)),
                _ => return None,
            };
            let (acceleration, deceleration) = match text(fields, "easing").as_str() {
                "ease_in" => (0.5, 0.0),
                "ease_out" => (0.0, 0.5),
                "ease_in_out" => (0.5, 0.5),
                _ => (0.0, 0.0),
            };
            Some(Effect {
                kind,
                start: number(fields, "start").unwrap_or(0.0),
                duration: if matches!(kind, EffectKind::Appear | EffectKind::Disappear) {
                    0.0
                } else {
                    number(fields, "duration").unwrap_or(DEFAULT_DURATION)
                },
                acceleration,
                deceleration,
            })
        })
        .collect();
    output.sort_by(|first, second| first.start.total_cmp(&second.start));
    output
}

/// The appearance of an object, relative to the object as authored.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Appearance {
    pub opacity: f64,
    /// A displacement in points.
    pub offset: [f64; 2],
    pub scale: f64,
    /// A clockwise turn in degrees.
    pub rotation: f64,
    /// The part in view of an object that a wipe has revealed or hidden in
    /// part.
    pub reveal: Option<Reveal>,
}

impl Default for Appearance {
    fn default() -> Self {
        Self {
            opacity: 1.0,
            offset: [0.0; 2],
            scale: 1.0,
            rotation: 0.0,
            reveal: None,
        }
    }
}

impl Appearance {
    /// Returns the appearance of an object at a time, or the object as
    /// authored where no time is given. An object whose first effect upon its
    /// visibility brings it into view is out of view until that effect begins.
    pub fn at(element: &Element, time: Option<f64>) -> Self {
        let mut appearance = Self::default();
        if let Some(offset) = object(&element.attributes, "offset") {
            for (axis, name) in ["x", "y"].into_iter().enumerate() {
                appearance.offset[axis] =
                    super::units(offset.get(name)).unwrap_or(0) as f64 / 12700.0;
            }
        }
        if element.kind == crate::model::ElementKind::Group {
            appearance.rotation = number(&element.attributes, "rotation").unwrap_or(0.0);
        }
        let Some(time) = time else {
            return appearance;
        };
        let effects = effects(&element.attributes);
        if effects
            .iter()
            .find(|effect| effect.concerns_visibility())
            .is_some_and(|effect| {
                matches!(
                    effect.kind,
                    EffectKind::FadeIn | EffectKind::WipeIn(_) | EffectKind::Appear
                )
            })
        {
            appearance.opacity = 0.0;
        }
        for effect in effects.iter().filter(|effect| time >= effect.start) {
            let progress = effect.progress(time);
            if effect.concerns_visibility() {
                appearance.reveal = None;
            }
            match effect.kind {
                EffectKind::FadeIn => appearance.opacity = progress,
                EffectKind::FadeOut => appearance.opacity = 1.0 - progress,
                EffectKind::WipeIn(edge) => {
                    appearance.opacity = 1.0;
                    appearance.reveal = (progress < 1.0).then_some(Reveal {
                        edge,
                        fraction: progress,
                    });
                }
                EffectKind::WipeOut(edge) => {
                    appearance.opacity = if progress < 1.0 { 1.0 } else { 0.0 };
                    appearance.reveal = (progress > 0.0).then_some(Reveal {
                        edge: edge.opposite(),
                        fraction: 1.0 - progress,
                    });
                }
                EffectKind::Appear => appearance.opacity = 1.0,
                EffectKind::Disappear => appearance.opacity = 0.0,
                EffectKind::Move(by) => {
                    for (offset, by) in appearance.offset.iter_mut().zip(by) {
                        *offset += by as f64 / 12700.0 * progress;
                    }
                }
                EffectKind::Scale(factor) => appearance.scale *= 1.0 + (factor - 1.0) * progress,
                EffectKind::Rotate(degrees) => appearance.rotation += degrees * progress,
            }
        }
        appearance
    }

    /// Returns the affine map of the page that enlarges and turns an object
    /// about a centre given in points. The map sends a point `(x, y)` to
    /// `(a x + c y + e, b x + d y + f)` for the coefficients `[a, b, c, d, e, f]`.
    pub fn matrix(&self, center: [f64; 2]) -> [f64; 6] {
        let (sine, cosine) = self.rotation.to_radians().sin_cos();
        let [first, second] = [cosine * self.scale, sine * self.scale];
        [
            first,
            second,
            -second,
            first,
            center[0] - first * center[0] + second * center[1],
            center[1] - second * center[0] - first * center[1],
        ]
    }
}

/// Returns the time at which the animation of a document ends: the latest
/// end of an effect or of the stated duration of a slide.
pub(crate) fn end(content: &[crate::model::Content]) -> f64 {
    content
        .iter()
        .filter_map(|node| match node {
            crate::model::Content::Element(element) => Some(element),
            crate::model::Content::Text(_) => None,
        })
        .map(|element| {
            effects(&element.attributes)
                .iter()
                .map(Effect::end)
                .chain(
                    (element.kind == crate::model::ElementKind::Slide)
                        .then(|| number(&element.attributes, "duration"))
                        .flatten(),
                )
                .fold(end(&element.children), f64::max)
        })
        .fold(0.0, f64::max)
}
