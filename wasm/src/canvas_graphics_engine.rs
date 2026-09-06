use std::f64::consts::PI;

use crate::graphics_engine::GraphicsEngine;
use crate::types::Point;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{js_sys::Array, CanvasRenderingContext2d, HtmlCanvasElement};

#[derive(Debug)]
pub struct CanvasGraphicsEngine {
    width: f64,
    height: f64,
    context: CanvasRenderingContext2d,
}

impl CanvasGraphicsEngine {
    pub fn new(canvas: HtmlCanvasElement) -> anyhow::Result<Self> {
        let context_object = canvas
            .get_context("2d")
            .map_err(|e| anyhow::format_err!("JavaScript error during get_context: {:?}", e))?
            .ok_or_else(|| anyhow::anyhow!("2D context non-supported or failed to initialize"))?;

        let context = context_object
            .dyn_into::<CanvasRenderingContext2d>()
            .map_err(|_| anyhow::anyhow!("Failed to cast context to CanvasRenderingContext2d"))?;

        Ok(Self {
            width: f64::from(canvas.width()),
            height: f64::from(canvas.height()),
            context,
        })
    }
}

impl GraphicsEngine for CanvasGraphicsEngine {
    fn clear(&mut self) {
        self.context.clear_rect(0.0, 0.0, self.width, self.height);
    }

    fn size(&self) -> (f64, f64) {
        (self.width, self.height)
    }

    fn draw_path(&mut self, points: &[Point], stroke_color: &str, line_width: f64) {
        if points.len() < 2 {
            return;
        }

        self.context.save();
        self.context.set_stroke_style_str(stroke_color);
        self.context.set_line_width(line_width);

        self.context.begin_path();
        self.context.move_to(points[0].x, points[0].y);
        for point in points {
            self.context.line_to(point.x, point.y);
        }
        self.context.stroke();
        self.context.restore();
    }

    fn draw_dots(&mut self, points: &[Point], radius: f64, fill_color: &str) {
        self.context.save();
        self.context.set_fill_style_str(fill_color);

        for point in points {
            self.context.begin_path();
            _ = self.context.arc(point.x, point.y, radius, 0.0, PI * 2.0);
            self.context.fill();
        }

        self.context.restore();
    }

    fn draw_circle(&mut self, center: Point, radius: f64, fill_color: &str) {
        self.context.save();
        self.context.set_fill_style_str(fill_color);
        self.context.begin_path();
        _ = self.context.arc(center.x, center.y, radius, 0.0, PI * 2.0);
        self.context.fill();
        self.context.restore();
    }

    fn draw_line(&mut self, from: Point, to: Point, color: &str, is_dashed: Option<bool>) {
        self.context.save();
        self.context.set_stroke_style_str(color);

        if is_dashed.unwrap_or(false) {
            let pattern = Array::new();
            pattern.push(&JsValue::from_f64(5.0));
            pattern.push(&JsValue::from_f64(5.0));
            _ = self.context.set_line_dash(&pattern);
        }

        self.context.begin_path();
        self.context.move_to(from.x, from.y);
        self.context.line_to(to.x, to.y);
        self.context.stroke();
        self.context.restore();
    }

    fn draw_rotated_rect(
        &mut self,
        center: Point,
        angle: f64,
        width: f64,
        height: f64,
        fill_color: &str,
        stroke_color: &str,
    ) {
        self.context.save();
        _ = self.context.translate(center.x, center.y);
        _ = self.context.rotate(angle);
        self.context.set_fill_style_str(fill_color);
        self.context.set_stroke_style_str(stroke_color);
        let x = -width / 2.0;
        let y = -height / 2.0;
        self.context.fill_rect(x, y, width, height);
        self.context.stroke_rect(x, y, width, height);
        self.context.restore();
    }

    fn fill_rect(&mut self, x: f64, y: f64, width: f64, height: f64, fill_color: &str) {
        self.context.save();
        self.context.set_fill_style_str(fill_color);
        self.context.fill_rect(x, y, width, height);
        self.context.restore();
    }

    fn draw_text(&mut self, text: &str, x: f64, y: f64, color: &str, font: &str) {
        self.context.save();
        self.context.set_fill_style_str(color);
        self.context.set_font(font);
        _ = self.context.fill_text(text, x, y);
        self.context.restore();
    }
}
