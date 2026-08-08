use std::f64::consts::PI;

use crate::graphics_engine::GraphicsEngine;
use crate::types::Point;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{js_sys::Array, CanvasRenderingContext2d, HtmlCanvasElement};

#[derive(Debug)]
#[wasm_bindgen]
pub struct CanvasGraphicsEngine {
    width: f64,
    height: f64,
    context: CanvasRenderingContext2d,
}

impl CanvasGraphicsEngine {
    pub fn new(canvas: HtmlCanvasElement) -> anyhow::Result<Self> {
        // 1. get_context("2d") で描画コンテキストを取得 (JsValueのOptionが返る)
        let context_object = canvas
            .get_context("2d")
            .map_err(|e| anyhow::format_err!("JavaScript error during get_context: {:?}", e))?
            .ok_or_else(|| anyhow::anyhow!("2D context non-supported or failed to initialize"))?;

        // 2. dyn_into を使って Object / JsValue から CanvasRenderingContext2d にキャスト
        let context = context_object
            .dyn_into::<CanvasRenderingContext2d>()
            .map_err(|_| anyhow::anyhow!("Failed to cast context to CanvasRenderingContext2d"))?;

        let width = f64::try_from(canvas.width())?;
        let height = f64::try_from(canvas.height())?;

        Ok(Self {
            width,
            height,
            context,
        })
    }
}

impl GraphicsEngine for CanvasGraphicsEngine {
    /**
     * 画面全体をクリア
     */
    fn clear(&mut self) {
        self.context.clear_rect(0.0, 0.0, self.width, self.height);
    }

    /**
     * グリッド線の描画
     */
    fn draw_grid(&mut self, step: f64, color: &str) {
        self.context.save();
        self.context.set_stroke_style_str(color);
        self.context.set_line_width(1.0);

        self.context.begin_path();
        // 垂直線 (x を増やす)
        let mut x = 0.0;
        // 丸め誤差で最後の線が消えないよう 1e-9 を加算して比較
        while x <= self.width + 1e-9 {
            self.context.move_to(x, 0.0);
            self.context.line_to(x, self.height);
            x += step;
        }

        // 水平線 (y を増やす)
        let mut y = 0.0;
        while y <= self.height + 1e-9 {
            self.context.move_to(0.0, y);
            self.context.line_to(self.width, y);
            y += step;
        }
        self.context.stroke();
        self.context.restore();
    }

    /**
     * 複数の点を繋ぐパスの描画
     */
    fn draw_path(&mut self, points: &[Point], stroke_color: &str, line_width: f64) {
        if points.len() < 2 {
            return;
        };

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

    /**
     * ドット（複数の円）の描画
     */
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

    /**
     * 円の描画
     */
    fn draw_circle(&mut self, center: Point, radius: f64, fill_color: &str) {
        self.context.save();
        self.context.set_fill_style_str(fill_color);
        self.context.begin_path();
        _ = self.context.arc(center.x, center.y, radius, 0.0, PI * 2.0);
        self.context.fill();
        self.context.restore();
    }

    /**
     * 直線の描画 (破線対応)
     */
    fn draw_line(
        &mut self,
        from: crate::types::Point,
        to: crate::types::Point,
        color: &str,
        is_dashed: Option<bool>,
    ) {
        self.context.save();
        self.context.set_stroke_style_str(color);

        if is_dashed.unwrap_or(false) {
            let pattern = Array::new(); // 破線パターン [描画の長さ, 空白の長さ]
            pattern.push(&JsValue::from_f64(5.0));
            pattern.push(&JsValue::from_f64(5.0));

            _ = self.context.set_line_dash(&pattern);
        }

        self.context.begin_path();
        self.context.move_to(from.x, from.y);
        self.context.line_to(to.x, to.y);
        self.context.stroke();

        self.context.restore(); // restore することで破線設定も元に戻る
    }

    /**
     * 回転付き矩形の描画
     */
    fn draw_rotated_rect(
        &mut self,
        center: crate::types::Point,
        angle: f64, // ラジアン指定 (度数の場合は angle * Math.PI / 180)
        width: f64,
        height: f64,
        fill_color: &str,
        stroke_color: &str,
    ) {
        self.context.save();

        // 1. 中心点へ移動して回転
        _ = self.context.translate(center.x, center.y);
        _ = self.context.rotate(angle);

        // 2. スタイル設定
        self.context.set_fill_style_str(fill_color);
        self.context.set_stroke_style_str(stroke_color);

        // 3. 原点(中心)に合わせてオフセット描画
        let x = -width / 2.0;
        let y = -height / 2.0;

        self.context.fill_rect(x, y, width, height);
        self.context.stroke_rect(x, y, width, height);

        // 4. 行列と座標系を元に戻す (超重要)
        self.context.restore();
    }
}
