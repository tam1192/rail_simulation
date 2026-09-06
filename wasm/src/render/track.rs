use crate::domain::track::RailState;
use crate::graphics_engine::GraphicsEngine;
use crate::types::Point;

pub fn render_track_sim(
    engine: &mut dyn GraphicsEngine,
    state: &RailState,
    p_front: &Point,
    p_rear: &Point,
    bogie_pitch: f64,
) {
    engine.clear();
    engine.draw_path(&state.track.nodes, "#3a3a3a", 14.0);
    engine.draw_path(&state.track.nodes, "#666666", 4.0);
    engine.draw_dots(&state.track.nodes, 3.0, "#888888");

    let center_x = (p_front.x + p_rear.x) / 2.0;
    let center_y = (p_front.y + p_rear.y) / 2.0;
    let angle = f64::atan2(p_front.y - p_rear.y, p_front.x - p_rear.x);
    let body_length = bogie_pitch + 50.0;
    let body_width = 42.0;

    engine.draw_rotated_rect(
        Point::new(center_x, center_y),
        angle,
        body_length,
        body_width,
        "rgba(74, 144, 226, 0.75)",
        "#4A90E2",
    );

    engine.draw_circle(*p_front, 7.0, "#FF5252");
    engine.draw_circle(*p_rear, 7.0, "#448AFF");
    engine.draw_line(*p_front, *p_rear, "rgba(255, 255, 255, 0.6)", Some(true));
}
