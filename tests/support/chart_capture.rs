use mote::layout::{DocumentTextPlan, PageOperation};
use serde_json::{Value, json};

/// Compare painted operations in order; filled/stroked rectangles are two marks.
pub fn projection(plan: &DocumentTextPlan) -> Value {
    Value::Array(plan.pages.iter().map(|page| {
        let mut clips = Vec::new();
        let mut rotations = Vec::new();
        let mut operations = Vec::new();
        for operation in &page.operations {
            match operation {
                PageOperation::PushClip(bounds) => clips.push(*bounds),
                PageOperation::PopClip => { clips.pop().unwrap(); },
                PageOperation::PushRotation { degrees, center } => rotations.push(json!({"degrees":degrees,"center":center})),
                PageOperation::PopRotation => { rotations.pop().unwrap(); },
                PageOperation::Text(index) => {
                    let placed = &page.lines[*index];
                    let line = &placed.line;
                    let fragments: Vec<_> = line.fragments.iter().map(|fragment| json!({"text": fragment.text, "x":fragment.x,"baseline":fragment.baseline,"advance":fragment.advance,"size":fragment.style.measurement.size_points})).collect();
                    operations.push(json!({"kind":"text","line":{"top":placed.top,"height":line.height,"x":line.x,"width":line.width,"fragments":fragments},"clip":clips.last(),"rotation":rotations.last()}));
                },
                PageOperation::Rectangle(rectangle) => operations.push(json!({"kind":"rectangle","bounds":[rectangle.x,rectangle.top,rectangle.width,rectangle.height],"color":rectangle.color})),
                PageOperation::OutlineRectangle { bounds, width, color } => operations.push(json!({"kind":"outline","bounds":bounds,"width":width,"color":color})),
                PageOperation::Rule(rule) => operations.push(json!({"kind":"rule","start":rule.start,"end":rule.end,"width":rule.width,"color":rule.color})),
                _ => panic!("uncaptured chart operation: {operation:?}"),
            }
        }
        assert!(clips.is_empty() && rotations.is_empty());
        Value::Array(operations)
    }).collect())
}
