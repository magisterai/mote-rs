use super::RenderError;
use tiny_skia::Mask;
/// Reuse one rectangular mask regardless of nesting depth.
pub(super) struct Clips {
    bounds: Vec<[i64; 4]>,
    mask: Mask,
}
impl Clips {
    pub fn new(width: u32, height: u32) -> Result<Self, RenderError> {
        Ok(Self {
            bounds: Vec::new(),
            mask: Mask::new(width, height)
                .ok_or_else(|| RenderError::Invalid("cannot allocate page clip mask".into()))?,
        })
    }
    pub fn mask(&self) -> Option<&Mask> {
        (!self.bounds.is_empty()).then_some(&self.mask)
    }
    pub fn push(&mut self, bounds: [f64; 4], scale: f64) {
        let mut bounds = [
            (bounds[0] * scale).round() as i64,
            (bounds[1] * scale).round() as i64,
            ((bounds[0] + bounds[2]) * scale).round() as i64,
            ((bounds[1] + bounds[3]) * scale).round() as i64,
        ];
        let parent = self.bounds.last().copied().unwrap_or([
            0,
            0,
            self.mask.width() as i64,
            self.mask.height() as i64,
        ]);
        bounds[0] = bounds[0].max(parent[0]);
        bounds[1] = bounds[1].max(parent[1]);
        bounds[2] = bounds[2].min(parent[2]);
        bounds[3] = bounds[3].min(parent[3]);
        self.bounds.push(bounds);
        self.refresh();
    }
    pub fn pop(&mut self) {
        self.bounds.pop();
        self.refresh();
    }
    fn refresh(&mut self) {
        let Some([left, top, right, bottom]) = self.bounds.last().copied() else {
            return;
        };
        self.mask.data_mut().fill(0);
        if left >= right || top >= bottom {
            return;
        }
        let width = self.mask.width() as usize;
        for row in top..bottom {
            self.mask.data_mut()
                [row as usize * width + left as usize..row as usize * width + right as usize]
                .fill(255);
        }
    }
}
