use mote::layout::{DocumentTextPlan, PageOperation};
use serde_json::{Value, json};

pub fn projection(plan: &DocumentTextPlan) -> Value {
    Value::Array(plan.pages.iter().map(|page| {
        let mut clips = Vec::new();
        let mut rotations = Vec::new();
        let mut operations = Vec::new();
        for operation in &page.operations {
            let clip = (clips.len() > 1).then(|| clips.last().unwrap());
            let previous_count = operations.len();
            match operation {
                PageOperation::PushRotation { degrees, center } => rotations.push(json!({"degrees":degrees,"center":center})),
                PageOperation::PopRotation => { rotations.pop().unwrap(); },
                PageOperation::Image(image) => operations.push(json!({"kind":"image","bounds":image.bounds,"image_clip":image.clip,"clip":clip,"link":image.link})),
                PageOperation::PushClip(bounds) => clips.push(*bounds),
                PageOperation::PopClip => { clips.pop().unwrap(); },
                PageOperation::Text(index) => {
                    let placed = &page.lines[*index];
                    let line = &placed.line;
                    for rectangle in &line.highlights {
                        operations.push(json!({"kind":"rectangle","bounds":[rectangle.x,rectangle.top,rectangle.width,rectangle.height],"color":rectangle.color,"clip":clip}));
                    }
                    let fragments: Vec<_> = line.fragments.iter().map(|fragment| json!({"text":fragment.text,"x":fragment.x,"baseline":fragment.baseline,"advance":fragment.advance,"size":fragment.style.measurement.size_points})).collect();
                    operations.push(json!({"kind":"text","line":{"top":placed.top,"height":line.height,"x":line.x,"width":line.width,"fragments":fragments},"clip":clip}));
                    for decoration in &line.decorations {
                        operations.push(json!({"kind":"decoration","start":decoration.start,"end":decoration.end,"y":decoration.y,"width":decoration.width,"color":decoration.color,"decoration_kind":match decoration.kind {mote::layout::DecorationKind::Underline=>"underline",mote::layout::DecorationKind::Strike=>"strike"},"clip":clip}));
                    }
                },
                PageOperation::Rectangle(rectangle) => operations.push(json!({"kind":"rectangle","bounds":[rectangle.x,rectangle.top,rectangle.width,rectangle.height],"color":rectangle.color})),
                PageOperation::PaintedRectangle {bounds,fill,stroke} => operations.push(json!({"kind":"painted_rectangle","bounds":bounds,"fill":fill.as_ref().map(|paint|json!({"color":paint.color,"alpha":paint.alpha})),"stroke":stroke.as_ref().map(|stroke|json!({"color":stroke.paint.color,"alpha":stroke.paint.alpha,"width":stroke.width,"dash":stroke.dash,"cap":format!("{:?}",stroke.cap)}))})),
                PageOperation::Stroke {rule,dash,round_cap} => operations.push(json!({"kind":"path","points":[rule.start,rule.end],"closed":false,"fill":null,"stroke":{"color":rule.color,"alpha":255,"width":rule.width,"dash":dash,"cap":if *round_cap {"Round"} else {"Flat"}}})),
                PageOperation::Path(path) => operations.push(json!({"kind":"path","points":path.points,"closed":path.closed,"fill":path.fill.as_ref().map(|paint|json!({"color":paint.color,"alpha":paint.alpha})),"stroke":path.stroke.as_ref().map(|stroke|json!({"color":stroke.paint.color,"alpha":stroke.paint.alpha,"width":stroke.width,"dash":stroke.dash,"cap":format!("{:?}",stroke.cap)}))})),
                PageOperation::ListMarker(marker) => operations.push(json!({"kind":"marker","slot":marker.slot,"baseline":marker.baseline,"font_size":marker.font_size,"checked":marker.checked,"clip":clip})),
                _ => panic!("uncaptured presentation operation: {operation:?}"),
            }
            if let Some(rotation) = rotations.last() {
                for operation in &mut operations[previous_count..] {
                    operation["rotation"] = rotation.clone();
                }
            }
        }
        assert!(clips.is_empty());
        assert!(rotations.is_empty());
        Value::Array(operations)
    }).collect())
}
