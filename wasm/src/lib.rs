use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

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

// JS側に返すデバッグ/ステータス情報
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
    canvas: HtmlCanvasElement,
    ctx: CanvasRenderingContext2d,

    // 二分探索の内部状態
    data: Vec<i32>,
    target: i32,
    low: usize,
    high: usize,
    mid: usize,
    step: usize,

    is_found: bool,
    is_finished: bool,
}

#[wasm_bindgen]
impl BinarySearch {
    /// コンストラクタ：Canvas要素を受け取って初期化
    #[wasm_bindgen(constructor)]
    pub fn new(
        canvas: HtmlCanvasElement,
        array_size: usize,
        target: i32,
    ) -> Result<BinarySearch, JsValue> {
        let ctx = canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()?;

        // 1. ソート済みデータの生成（例: 5, 10, 15...）
        let data: Vec<i32> = (1..=array_size).map(|i| (i * 5) as i32).collect();
        let high = if data.is_empty() { 0 } else { data.len() - 1 };

        let mut instance = BinarySearch {
            canvas,
            ctx,
            data,
            target,
            low: 0,
            high,
            mid: 0,
            step: 0,
            is_found: false,
            is_finished: false,
        };

        // 初期フレームを描画
        instance.render();

        Ok(instance)
    }

    /// 設定値のリセット・データ再生成
    #[wasm_bindgen]
    pub fn reset(&mut self, array_size: usize, target: i32) {
        self.data = (1..=array_size).map(|i| (i * 5) as i32).collect();
        self.target = target;
        self.low = 0;
        self.high = if self.data.is_empty() {
            0
        } else {
            self.data.len() - 1
        };
        self.mid = 0;
        self.step = 0;
        self.is_found = false;
        self.is_finished = false;

        self.render();
    }

    /// 1ステップ進める（コマ送り / タイマー実行用）
    #[wasm_bindgen]
    pub fn step(&mut self) -> SearchStatus {
        if !self.is_finished && self.low <= self.high {
            self.step += 1;
            self.mid = self.low + (self.high - self.low) / 2;
            let mid_val = self.data[self.mid];

            if mid_val == self.target {
                self.is_found = true;
                self.is_finished = true;
            } else if mid_val < self.target {
                if self.mid == usize::MAX || self.mid + 1 > self.high {
                    self.is_finished = true;
                } else {
                    self.low = self.mid + 1;
                }
            } else {
                if self.mid == 0 || self.mid - 1 < self.low {
                    self.is_finished = true;
                } else {
                    self.high = self.mid - 1;
                }
            }
        } else {
            self.is_finished = true;
        }

        // 状態が更新されたので再描画
        self.render();

        // JS側（Vue）に最新ステータスを返す
        self.get_status()
    }

    /// 現在の状態を取得
    #[wasm_bindgen]
    pub fn get_status(&self) -> SearchStatus {
        SearchStatus {
            current_step: self.step,
            low: self.low as i32,
            mid: self.mid as i32,
            high: self.high as i32,
            is_found: self.is_found,
            is_finished: self.is_finished,
        }
    }

    /// Canvasへの描画処理（可視化の核心）
    fn render(&self) {
        let width = self.canvas.width() as f64;
        let height = self.canvas.height() as f64;

        // 画面クリア
        self.ctx.clear_rect(0.0, 0.0, width, height);

        if self.data.is_empty() {
            return;
        }

        // バーやポインタ描画のレイアウト計算
        let padding = 50.0;
        let available_width = width - (padding * 2.0);
        let bar_width = available_width / self.data.len() as f64;

        for (i, &val) in self.data.iter().enumerate() {
            let x = padding + (i as f64 * bar_width);
            let y = height / 2.0;

            // --- 色分けロジック ---
            // 範囲外: 暗いグレー
            // Low/High 範囲内: 青
            // Mid: 黄色 / 発見時: 緑
            let mut fill_color = "#333333";
            if i >= self.low && i <= self.high {
                fill_color = "#2563eb"; // 探索対象領域（青）
            }
            if self.step > 0 && i == self.mid {
                fill_color = if self.is_found { "#10b981" } else { "#f59e0b" }; // Mid（黄/緑）
            }

            // ボックス描画
            self.ctx.set_fill_style_str(fill_color);
            self.ctx.fill_rect(x + 2.0, y - 20.0, bar_width - 4.0, 40.0);

            // 値（数値）のテキスト描画
            self.ctx.set_fill_style_str("#ffffff");
            self.ctx.set_font("14px monospace");
            self.ctx
                .fill_text(&val.to_string(), x + (bar_width / 4.0), y + 5.0)
                .ok();

            // インデックス (0, 1, 2...) の描画
            self.ctx.set_fill_style_str("#888888");
            self.ctx.set_font("10px monospace");
            self.ctx
                .fill_text(&i.to_string(), x + (bar_width / 4.0), y + 35.0)
                .ok();
        }

        // ポインタ（Low / Mid / High）の矢印やラベルを描画
        if self.step > 0 {
            // Low ポインタ (紫)
            let low_x = padding + (self.low as f64 * bar_width) + (bar_width / 3.0);
            self.ctx.set_fill_style_str("#a855f7");
            self.ctx.fill_text("▲ Low", low_x, height / 2.0 - 30.0).ok();

            // High ポインタ (水色)
            let high_x = padding + (self.high as f64 * bar_width) + (bar_width / 3.0);
            self.ctx.set_fill_style_str("#06b6d4");
            self.ctx
                .fill_text("▲ High", high_x, height / 2.0 + 55.0)
                .ok();
        }
    }
}
