use web_sys::CanvasRenderingContext2d;

use crate::canvas::viewport::Viewport;
use crate::model::block::{Block, BlockContent};

/// Draw all blocks onto a canvas 2D context, applying the viewport transform.
pub fn paint_blocks(
    ctx: &CanvasRenderingContext2d,
    viewport: &Viewport,
    blocks: &[Block],
    canvas_width: f64,
    canvas_height: f64,
) {
    // Clear
    ctx.clear_rect(0.0, 0.0, canvas_width, canvas_height);

    ctx.save();
    // Apply viewport transform.
    ctx.translate(viewport.offset_x, viewport.offset_y).ok();
    ctx.scale(viewport.zoom, viewport.zoom).ok();

    for block in blocks {
        paint_block(ctx, block);
    }

    ctx.restore();
}

fn paint_block(ctx: &CanvasRenderingContext2d, block: &Block) {
    let w = if block.width > 0.0 { block.width } else { 200.0 };
    let h = if block.height > 0.0 {
        block.height
    } else {
        100.0
    };

    // Block background
    ctx.set_fill_style_str("#ffffff");
    ctx.fill_rect(block.x, block.y, w, h);

    // Block border
    ctx.set_stroke_style_str("#cccccc");
    ctx.set_line_width(1.0);
    ctx.stroke_rect(block.x, block.y, w, h);

    match &block.content {
        BlockContent::Text(text) => {
            if !text.is_empty() {
                ctx.set_fill_style_str("#333333");
                ctx.set_font("14px monospace");
                // Simple single-line rendering; the real app renders markdown via DOM overlay.
                let _ = ctx.fill_text(text, block.x + 8.0, block.y + 20.0);
            }
        }
        BlockContent::Drawing(strokes) => {
            for stroke in strokes {
                if stroke.points.len() < 2 {
                    continue;
                }
                ctx.begin_path();
                ctx.set_stroke_style_str(&stroke.color);
                ctx.set_line_width(stroke.width as f64);
                ctx.move_to(stroke.points[0].x, stroke.points[0].y);
                for pt in &stroke.points[1..] {
                    ctx.line_to(pt.x, pt.y);
                }
                ctx.stroke();
            }
        }
    }
}
