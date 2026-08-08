use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

use crate::{
    canvas_graphics_engine::CanvasGraphicsEngine,
    graphics_engine::GraphicsEngine,
    utils::{render_track_sim, use_track, UseTrackOutput},
};
mod canvas_graphics_engine;
mod graphics_engine;
mod types;
mod utils;

#[wasm_bindgen]
pub struct DebugInfo {
    #[wasm_bindgen]
    pub calculated_chord_length: f64,
}

#[wasm_bindgen]
pub struct RailSimulation {
    track: UseTrackOutput,
    engine: CanvasGraphicsEngine,
    d_front: f64,
}

#[wasm_bindgen]
impl RailSimulation {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: HtmlCanvasElement) -> Self {
        let engine = CanvasGraphicsEngine::new(canvas).unwrap();
        Self {
            engine,
            d_front: 120.0,
            track: use_track(),
        }
    }

    #[wasm_bindgen]
    pub fn execute(&mut self, use_correction: bool, speed: f64, bogie_pitch: f64) -> DebugInfo {
        // 位置の更新
        self.d_front += speed;
        if self.d_front > self.track.total_track_length - 10.0 {
            self.d_front = 80.0; // loop
        }

        // 台車位置計算
        let p_front = self.track.get_point_at_distance(self.d_front);
        let p_rear = if use_correction {
            let d_rear = self.track.find_rear_distance(self.d_front, bogie_pitch);
            self.track.get_point_at_distance(d_rear)
        } else {
            self.track.get_point_at_distance(self.d_front - bogie_pitch)
        };

        self.engine.clear();
        render_track_sim(
            &mut self.engine,
            &self.track.track_nodes,
            &p_front,
            &p_rear,
            bogie_pitch,
        );

        DebugInfo {
            calculated_chord_length: f64::hypot(p_front.x - p_rear.x, p_front.y - p_rear.y),
        }
    }
}
