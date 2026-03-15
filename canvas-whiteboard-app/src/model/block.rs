use serde::{Deserialize, Serialize};

use super::hash;

/// The atomic unit of the whiteboard.
/// A block lives on a canvas (shelf) at a position and holds either
/// text (optionally markdown) or drawing strokes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    /// Hash-based reference id.
    pub id: String,
    /// Canvas x-position.
    pub x: f64,
    /// Canvas y-position.
    pub y: f64,
    /// Width (0 = auto-size).
    pub width: f64,
    /// Height (0 = auto-size).
    pub height: f64,
    /// Content of this block.
    pub content: BlockContent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BlockContent {
    /// Plain or markdown text.
    Text(String),
    /// Freehand drawing encoded as a series of strokes.
    Drawing(Vec<Stroke>),
}

/// A single continuous pen stroke.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stroke {
    pub points: Vec<Point>,
    pub color: String,
    pub width: f32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Block {
    pub fn new(x: f64, y: f64) -> Self {
        let id = hash::make_id("block");
        Self {
            id,
            x,
            y,
            width: 0.0,
            height: 0.0,
            content: BlockContent::Text(String::new()),
        }
    }

    /// Set the text content, replacing whatever was there.
    pub fn set_text(&mut self, text: &str) {
        self.content = BlockContent::Text(text.to_string());
    }

    /// Return the text content (empty string for drawings).
    pub fn content_text(&self) -> String {
        match &self.content {
            BlockContent::Text(t) => t.clone(),
            BlockContent::Drawing(_) => String::new(),
        }
    }

    /// Start a new stroke (for drawing mode).
    pub fn begin_stroke(&mut self, color: &str, width: f32) {
        if let BlockContent::Drawing(strokes) = &mut self.content {
            strokes.push(Stroke {
                points: Vec::new(),
                color: color.to_string(),
                width,
            });
        } else {
            // Switch to drawing mode.
            self.content = BlockContent::Drawing(vec![Stroke {
                points: Vec::new(),
                color: color.to_string(),
                width,
            }]);
        }
    }

    /// Append a point to the current (last) stroke.
    pub fn add_point(&mut self, x: f64, y: f64) {
        if let BlockContent::Drawing(strokes) = &mut self.content {
            if let Some(stroke) = strokes.last_mut() {
                stroke.points.push(Point { x, y });
            }
        }
    }
}
