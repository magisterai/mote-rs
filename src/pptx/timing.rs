//! The timing tree of a slide: the effects of its objects on one clock.
//!
//! PowerPoint keeps no absolute clock. The effects are therefore written into
//! the first step of the main sequence, which begins with the slide, each to
//! start with that step after a delay equal to its own start.
use crate::{
    model::{Content, Element, ElementKind},
    presentation::animation::{self, Edge, Effect, EffectKind},
};
use std::collections::BTreeMap;

struct Target {
    identifier: usize,
    /// A shape with a text body is listed among the builds of the slide.
    has_text_body: bool,
    effects: Vec<Effect>,
}

#[derive(Default)]
pub(super) struct Timeline {
    targets: Vec<Target>,
}

impl Timeline {
    /// Records the effects of an object whose shape identifier is known.
    pub fn record(&mut self, identifier: usize, element: &Element) {
        let effects = animation::effects(&element.attributes);
        if effects.is_empty() {
            return;
        }
        let has_text_body = match element.kind {
            ElementKind::Shape => true,
            ElementKind::Box => !matches!(
                element.children.as_slice(),
                [Content::Element(child)] if matches!(
                    child.kind,
                    ElementKind::Image | ElementKind::Table | ElementKind::Chart
                )
            ),
            _ => false,
        };
        self.targets.push(Target {
            identifier,
            has_text_body,
            effects,
        });
    }

    /// Writes the timing element of a slide of the given size in drawing
    /// units, or nothing where no object of the slide is animated.
    pub fn write(&self, [width, height]: [i64; 2]) -> String {
        let mut ordered: Vec<(&Target, &Effect)> = self
            .targets
            .iter()
            .flat_map(|target| target.effects.iter().map(move |effect| (target, effect)))
            .collect();
        if ordered.is_empty() {
            return String::new();
        }
        ordered.sort_by(|first, second| first.1.start.total_cmp(&second.1.start));
        let mut output = String::from(
            r#"<p:timing><p:tnLst><p:par><p:cTn id="1" dur="indefinite" restart="never" nodeType="tmRoot"><p:childTnLst><p:seq concurrent="1" nextAc="seek"><p:cTn id="2" dur="indefinite" nodeType="mainSeq"><p:childTnLst><p:par><p:cTn id="3" fill="hold"><p:stCondLst><p:cond delay="indefinite"/><p:cond evt="onBegin" delay="0"><p:tn val="2"/></p:cond></p:stCondLst><p:childTnLst><p:par><p:cTn id="4" fill="hold"><p:stCondLst><p:cond delay="0"/></p:stCondLst><p:childTnLst>"#,
        );
        let mut next_node = 5;
        let mut node = || {
            next_node += 1;
            next_node - 1
        };
        // A motion path is measured from the place of the object as authored,
        // so that each path begins where the paths before it ended.
        let mut displacements: BTreeMap<usize, [f64; 2]> = BTreeMap::new();
        let mut build_groups: BTreeMap<usize, usize> = BTreeMap::new();
        let mut builds = String::new();
        for (index, (target, effect)) in ordered.iter().enumerate() {
            let identifier = target.identifier;
            let shape = format!(r#"<p:tgtEl><p:spTgt spid="{identifier}"/></p:tgtEl>"#);
            let duration = milliseconds(effect.duration).max(1);
            let visibility = |node: usize, delay: i64, state: &str| {
                format!(
                    r#"<p:set><p:cBhvr><p:cTn id="{node}" dur="1" fill="hold"><p:stCondLst><p:cond delay="{delay}"/></p:stCondLst></p:cTn>{shape}<p:attrNameLst><p:attrName>style.visibility</p:attrName></p:attrNameLst></p:cBhvr><p:to><p:strVal val="{state}"/></p:to></p:set>"#
                )
            };
            let outer = node();
            // A wipe is the fade of PowerPoint with another preset and filter;
            // the subtype of the preset names the edge as the filter does.
            let (transition, subtype, filter) = match effect.kind {
                EffectKind::WipeIn(edge) | EffectKind::WipeOut(edge) => {
                    let (subtype, filter) = match edge {
                        Edge::Left => (8, "wipe(left)"),
                        Edge::Right => (2, "wipe(right)"),
                        Edge::Top => (1, "wipe(up)"),
                        Edge::Bottom => (4, "wipe(down)"),
                    };
                    (22, subtype, filter)
                }
                _ => (10, 0, "fade"),
            };
            let (preset, class, behaviors) = match effect.kind {
                EffectKind::FadeIn | EffectKind::WipeIn(_) => (
                    transition,
                    "entr",
                    format!(
                        r#"{}<p:animEffect transition="in" filter="{filter}"><p:cBhvr><p:cTn id="{}" dur="{duration}"/>{shape}</p:cBhvr></p:animEffect>"#,
                        visibility(node(), 0, "visible"),
                        node()
                    ),
                ),
                EffectKind::FadeOut | EffectKind::WipeOut(_) => (
                    transition,
                    "exit",
                    format!(
                        r#"<p:animEffect transition="out" filter="{filter}"><p:cBhvr><p:cTn id="{}" dur="{duration}"/>{shape}</p:cBhvr></p:animEffect>{}"#,
                        node(),
                        visibility(node(), duration - 1, "hidden")
                    ),
                ),
                EffectKind::Appear => (1, "entr", visibility(node(), 0, "visible")),
                EffectKind::Disappear => (1, "exit", visibility(node(), 0, "hidden")),
                EffectKind::Move(by) => {
                    let from = displacements.entry(identifier).or_default();
                    let to = [
                        from[0] + by[0] as f64 / width as f64,
                        from[1] + by[1] as f64 / height as f64,
                    ];
                    let path = format!(
                        "M {} {} L {} {}",
                        fraction(from[0]),
                        fraction(from[1]),
                        fraction(to[0]),
                        fraction(to[1])
                    );
                    *from = to;
                    (
                        0,
                        "path",
                        format!(
                            r#"<p:animMotion origin="layout" path="{path}" pathEditMode="relative" ptsTypes="AA"><p:cBhvr><p:cTn id="{}" dur="{duration}" fill="hold"/>{shape}<p:attrNameLst><p:attrName>ppt_x</p:attrName><p:attrName>ppt_y</p:attrName></p:attrNameLst></p:cBhvr></p:animMotion>"#,
                            node()
                        ),
                    )
                }
                EffectKind::Scale(factor) => {
                    let by = (factor * 100000.0).round() as i64;
                    (
                        6,
                        "emph",
                        format!(
                            r#"<p:animScale><p:cBhvr><p:cTn id="{}" dur="{duration}" fill="hold"/>{shape}</p:cBhvr><p:by x="{by}" y="{by}"/></p:animScale>"#,
                            node()
                        ),
                    )
                }
                EffectKind::Rotate(degrees) => (
                    8,
                    "emph",
                    format!(
                        r#"<p:animRot by="{}"><p:cBhvr><p:cTn id="{}" dur="{duration}" fill="hold"/>{shape}<p:attrNameLst><p:attrName>r</p:attrName></p:attrNameLst></p:cBhvr></p:animRot>"#,
                        (degrees * 60000.0).round() as i64,
                        node()
                    ),
                ),
            };
            let mut pace = String::new();
            for (name, value) in [
                ("accel", effect.acceleration),
                ("decel", effect.deceleration),
            ] {
                if value > 0.0 {
                    pace.push_str(&format!(
                        r#" {name}="{}""#,
                        (value * 100000.0).round() as i64
                    ));
                }
            }
            // PowerPoint keeps one effect of a text shape for each build group,
            // so that every effect of such a shape is a group of its own.
            let build = if target.has_text_body {
                let group = build_groups.entry(identifier).or_default();
                *group += 1;
                builds.push_str(&format!(
                    r#"<p:bldP spid="{identifier}" grpId="{}" animBg="1"/>"#,
                    *group - 1
                ));
                format!(r#" grpId="{}""#, *group - 1)
            } else {
                String::new()
            };
            let node_type = if index == 0 {
                "afterEffect"
            } else {
                "withEffect"
            };
            output.push_str(&format!(
                r#"<p:par><p:cTn id="{outer}" presetID="{preset}" presetClass="{class}" presetSubtype="{subtype}"{pace} fill="hold"{build} nodeType="{node_type}"><p:stCondLst><p:cond delay="{}"/></p:stCondLst><p:childTnLst>{behaviors}</p:childTnLst></p:cTn></p:par>"#,
                milliseconds(effect.start)
            ));
        }
        output.push_str(
            r#"</p:childTnLst></p:cTn></p:par></p:childTnLst></p:cTn></p:par></p:childTnLst></p:cTn><p:prevCondLst><p:cond evt="onPrev" delay="0"><p:tgtEl><p:sldTgt/></p:tgtEl></p:cond></p:prevCondLst><p:nextCondLst><p:cond evt="onNext" delay="0"><p:tgtEl><p:sldTgt/></p:tgtEl></p:cond></p:nextCondLst></p:seq></p:childTnLst></p:cTn></p:par></p:tnLst>"#,
        );
        if !builds.is_empty() {
            output.push_str(&format!("<p:bldLst>{builds}</p:bldLst>"));
        }
        output.push_str("</p:timing>");
        output
    }
}

/// Writes the transition that advances a slide after its duration in seconds.
pub(super) fn advance(duration: f64) -> String {
    format!(r#"<p:transition advTm="{}"/>"#, milliseconds(duration))
}

fn milliseconds(seconds: f64) -> i64 {
    (seconds * 1000.0).round() as i64
}

fn fraction(value: f64) -> String {
    let rounded = (value * 1_000_000.0).round() / 1_000_000.0;
    // A negative zero would be written with its sign.
    (if rounded == 0.0 { 0.0 } else { rounded }).to_string()
}
