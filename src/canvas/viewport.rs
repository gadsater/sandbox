use serde::{Deserialize, Serialize};

/// Camera / viewport state for the infinite canvas.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Viewport {
    /// Horizontal offset (pan).
    pub offset_x: f64,
    /// Vertical offset (pan).
    pub offset_y: f64,
    /// Zoom level (1.0 = 100%).
    pub zoom: f64,
}

impl Default for Viewport {
    fn default() -> Self {
        Self {
            offset_x: 0.0,
            offset_y: 0.0,
            zoom: 1.0,
        }
    }
}

impl Viewport {
    /// Convert a screen coordinate to a canvas (world) coordinate.
    pub fn screen_to_world(&self, sx: f64, sy: f64) -> (f64, f64) {
        let wx = (sx - self.offset_x) / self.zoom;
        let wy = (sy - self.offset_y) / self.zoom;
        (wx, wy)
    }

    /// Convert a canvas coordinate to a screen coordinate.
    pub fn world_to_screen(&self, wx: f64, wy: f64) -> (f64, f64) {
        let sx = wx * self.zoom + self.offset_x;
        let sy = wy * self.zoom + self.offset_y;
        (sx, sy)
    }

    /// Pan by a delta in screen pixels.
    pub fn pan(&mut self, dx: f64, dy: f64) {
        self.offset_x += dx;
        self.offset_y += dy;
    }

    /// Zoom centred on a screen point.
    pub fn zoom_at(&mut self, factor: f64, cx: f64, cy: f64) {
        let (wx, wy) = self.screen_to_world(cx, cy);
        self.zoom *= factor;
        self.zoom = self.zoom.clamp(0.1, 10.0);
        // Recalculate offset so the world point stays under the cursor.
        self.offset_x = cx - wx * self.zoom;
        self.offset_y = cy - wy * self.zoom;
    }
}
