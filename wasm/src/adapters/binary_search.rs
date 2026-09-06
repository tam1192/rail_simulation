use crate::canvas_graphics_engine::CanvasGraphicsEngine;
use crate::domain::binary_search::BinarySearchState;
use crate::domain::formula::join_trace;
use crate::render::binary_search::render_binary_search;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct SearchStatus {
    pub current_step: usize,
    pub low: i32,
    pub mid: i32,
    pub high: i32,
    pub is_found: bool,
    pub is_finished: bool,
}

#[wasm_bindgen]
pub struct BinarySearch {
    engine: CanvasGraphicsEngine,
    state: BinarySearchState,
    last_trace: String,
}

#[wasm_bindgen]
impl BinarySearch {
    #[wasm_bindgen(constructor)]
    pub fn new(
        canvas: HtmlCanvasElement,
        array_size: usize,
        target: i32,
    ) -> Result<BinarySearch, JsValue> {
        let engine =
            CanvasGraphicsEngine::new(canvas).map_err(|e| JsValue::from_str(&format!("{e:#}")))?;
        let state = BinarySearchState::new(array_size, target);
        let formula_trace = format!(
            "初期区間\nl = 0\nh = {}\nt = {target}",
            array_size.saturating_sub(1)
        );
        let mut instance = Self {
            engine,
            state,
            last_trace: formula_trace,
        };
        instance.paint();
        Ok(instance)
    }

    #[wasm_bindgen]
    pub fn reset(&mut self, array_size: usize, target: i32) {
        self.state.reset(array_size, target);
        self.last_trace = format!(
            "初期区間\nl = 0\nh = {}\nt = {target}",
            array_size.saturating_sub(1)
        );
        self.paint();
    }

    #[wasm_bindgen]
    pub fn step(&mut self) -> SearchStatus {
        let trace = self.state.step();
        self.last_trace = join_trace(&trace);
        self.paint();
        self.get_status()
    }

    #[wasm_bindgen(getter)]
    pub fn formula_trace(&self) -> String {
        self.last_trace.clone()
    }

    #[wasm_bindgen]
    pub fn get_status(&self) -> SearchStatus {
        SearchStatus {
            current_step: self.state.step,
            low: self.state.low as i32,
            mid: self.state.mid as i32,
            high: self.state.high as i32,
            is_found: self.state.is_found,
            is_finished: self.state.is_finished,
        }
    }

    fn paint(&mut self) {
        render_binary_search(&mut self.engine, &self.state);
    }
}
