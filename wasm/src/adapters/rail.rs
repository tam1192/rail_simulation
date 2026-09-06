use crate::canvas_graphics_engine::CanvasGraphicsEngine;
use crate::domain::formula::join_trace;
use crate::domain::track::RailState;
use crate::render::track::render_track_sim;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

#[wasm_bindgen]
pub struct DebugInfo {
    #[wasm_bindgen]
    pub calculated_chord_length: f64,
    formula_trace: String,
}

#[wasm_bindgen]
impl DebugInfo {
    #[wasm_bindgen(getter)]
    pub fn formula_trace(&self) -> String {
        self.formula_trace.clone()
    }
}

#[wasm_bindgen]
pub struct RailSimulation {
    state: RailState,
    engine: CanvasGraphicsEngine,
}

#[wasm_bindgen]
impl RailSimulation {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: HtmlCanvasElement) -> Self {
        let engine = CanvasGraphicsEngine::new(canvas).expect("canvas 2d context");
        Self {
            engine,
            state: RailState::new(),
        }
    }

    #[wasm_bindgen]
    pub fn execute(&mut self, use_correction: bool, speed: f64, bogie_pitch: f64) -> DebugInfo {
        self.state.tick(speed);
        let (p_front, p_rear, trace) = self.state.bogie_points(use_correction, bogie_pitch);

        render_track_sim(
            &mut self.engine,
            &self.state,
            &p_front,
            &p_rear,
            bogie_pitch,
        );

        DebugInfo {
            calculated_chord_length: f64::hypot(p_front.x - p_rear.x, p_front.y - p_rear.y),
            formula_trace: join_trace(&trace),
        }
    }
}
