use crate::types::Point;

/// Drawing port. Domain and use cases depend on this, not on Canvas.
pub trait GraphicsEngine {
    fn clear(&mut self);
    fn size(&self) -> (f64, f64);

    fn draw_path(&mut self, points: &[Point], stroke_color: &str, line_width: f64);
    fn draw_dots(&mut self, points: &[Point], radius: f64, fill_color: &str);
    fn draw_circle(&mut self, center: Point, radius: f64, fill_color: &str);
    fn draw_line(&mut self, from: Point, to: Point, color: &str, is_dashed: Option<bool>);
    fn draw_rotated_rect(
        &mut self,
        center: Point,
        angle: f64,
        width: f64,
        height: f64,
        fill_color: &str,
        stroke_color: &str,
    );
    fn fill_rect(&mut self, x: f64, y: f64, width: f64, height: f64, fill_color: &str);
    fn draw_text(&mut self, text: &str, x: f64, y: f64, color: &str, font: &str);
}
