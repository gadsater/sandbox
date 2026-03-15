use wasm_bindgen::prelude::*;

use super::viewport::Viewport;
use crate::model::block::Block;

/// Manages canvas interactions: drawing strokes, selecting blocks, panning/zooming.
#[wasm_bindgen]
pub struct CanvasController {
    viewport: Viewport,
    /// Currently selected block id (if any).
    selected_block: Option<String>,
    /// Whether we are in drawing mode (vs text/select mode).
    drawing_mode: bool,
    /// Whether we are currently mid-stroke.
    is_drawing: bool,
    /// Pen colour.
    pen_color: String,
    /// Pen width.
    pen_width: f32,
}

#[wasm_bindgen]
impl CanvasController {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            viewport: Viewport::default(),
            selected_block: None,
            drawing_mode: false,
            is_drawing: false,
            pen_color: "#000000".into(),
            pen_width: 2.0,
        }
    }

    pub fn toggle_drawing_mode(&mut self) {
        self.drawing_mode = !self.drawing_mode;
    }

    pub fn is_drawing_mode(&self) -> bool {
        self.drawing_mode
    }

    pub fn set_pen_color(&mut self, color: &str) {
        self.pen_color = color.to_string();
    }

    pub fn set_pen_width(&mut self, width: f32) {
        self.pen_width = width;
    }

    /// Pan the canvas.
    pub fn pan(&mut self, dx: f64, dy: f64) {
        self.viewport.pan(dx, dy);
    }

    /// Zoom centred on a point.
    pub fn zoom(&mut self, factor: f64, cx: f64, cy: f64) {
        self.viewport.zoom_at(factor, cx, cy);
    }

    pub fn offset_x(&self) -> f64 {
        self.viewport.offset_x
    }

    pub fn offset_y(&self) -> f64 {
        self.viewport.offset_y
    }

    pub fn zoom_level(&self) -> f64 {
        self.viewport.zoom
    }

    /// Convert screen coords to world coords.
    pub fn screen_to_world(&self, sx: f64, sy: f64) -> Vec<f64> {
        let (wx, wy) = self.viewport.screen_to_world(sx, sy);
        vec![wx, wy]
    }

    pub fn select_block(&mut self, id: &str) {
        self.selected_block = Some(id.to_string());
    }

    pub fn deselect(&mut self) {
        self.selected_block = None;
    }

    pub fn selected_block_id(&self) -> Option<String> {
        self.selected_block.clone()
    }
}

impl CanvasController {
    /// Start a stroke on a block (Rust-only API, not wasm-exposed because Block isn't).
    pub fn begin_stroke(&mut self, block: &mut Block) {
        block.begin_stroke(&self.pen_color, self.pen_width);
        self.is_drawing = true;
    }

    /// Add a point to the active stroke.
    pub fn add_stroke_point(&self, block: &mut Block, x: f64, y: f64) {
        if self.is_drawing {
            block.add_point(x, y);
        }
    }

    /// End the current stroke.
    pub fn end_stroke(&mut self) {
        self.is_drawing = false;
    }

    /// Hit-test: which block (if any) is at the given world position?
    pub fn hit_test<'a>(&self, blocks: &'a [Block], wx: f64, wy: f64) -> Option<&'a Block> {
        // Default block size for hit testing when auto-sized.
        const DEFAULT_W: f64 = 200.0;
        const DEFAULT_H: f64 = 100.0;
        for block in blocks.iter().rev() {
            let w = if block.width > 0.0 {
                block.width
            } else {
                DEFAULT_W
            };
            let h = if block.height > 0.0 {
                block.height
            } else {
                DEFAULT_H
            };
            if wx >= block.x && wx <= block.x + w && wy >= block.y && wy <= block.y + h {
                return Some(block);
            }
        }
        None
    }
}
