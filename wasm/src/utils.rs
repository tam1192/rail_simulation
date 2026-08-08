use std::f64::consts::PI;

use crate::graphics_engine::GraphicsEngine;
use crate::types::Point;

pub fn render_track_sim(
    engine: &mut dyn GraphicsEngine,
    track_nodes: &[Point],
    p_front: &Point,
    p_rear: &Point,
    bogie_pitch: f64,
) {
    engine.clear();
    engine.draw_path(track_nodes, "#3a3a3a", 14.0);
    engine.draw_path(track_nodes, "#666666", 4.0);
    engine.draw_dots(track_nodes, 3.0, "#888888");

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

    // 6. 前後台車
    engine.draw_circle(*p_front, 7.0, "#FF5252");
    engine.draw_circle(*p_rear, 7.0, "#448AFF");

    // 7. 弦（点線）
    engine.draw_line(*p_front, *p_rear, "rgba(255, 255, 255, 0.6)", Some(true));
}

// 戻り値となる構造体
pub struct UseTrackOutput {
    pub track_nodes: Vec<Point>,
    pub d: Vec<f64>,
    pub total_track_length: f64,
}

impl UseTrackOutput {
    // 指定距離の座標を取得
    pub fn get_point_at_distance(&self, d: f64) -> Point {
        let clamped_d = d.clamp(0.0, self.total_track_length);

        // 該当する線分（インデックス）を探索
        for i in 0..(self.d.len() - 1) {
            if self.d[i] <= clamped_d && clamped_d <= self.d[i + 1] {
                let segment_len = self.d[i + 1] - self.d[i];
                if segment_len == 0.0 {
                    return self.track_nodes[i];
                }

                let t = (clamped_d - self.d[i]) / segment_len;
                let x =
                    self.track_nodes[i].x + t * (self.track_nodes[i + 1].x - self.track_nodes[i].x);
                let y =
                    self.track_nodes[i].y + t * (self.track_nodes[i + 1].y - self.track_nodes[i].y);

                return Point::new(x, y);
            }
        }

        *self.track_nodes.last().unwrap()
    }

    // 後方台車の距離を二分探索（弦長補正）
    pub fn find_rear_distance(&self, d_front: f64, length: f64) -> f64 {
        let p_front = self.get_point_at_distance(d_front);
        let mut low = (d_front - length * 1.8).max(0.0);
        let mut high = d_front;

        for _ in 0..15 {
            let mid = (low + high) / 2.0;
            let p_mid = self.get_point_at_distance(mid);
            let dx = p_front.x - p_mid.x;
            let dy = p_front.y - p_mid.y;
            let dist = dx.hypot(dy);

            if dist < length {
                high = mid;
            } else {
                low = mid;
            }
        }

        (low + high) / 2.0
    }
}

pub fn use_track() -> UseTrackOutput {
    let num_nodes = 400;

    // コースノードの生成 (0..=num_nodes で 401 個生成)
    let track_nodes: Vec<Point> = (0..=num_nodes)
        .map(|i| {
            let t = i as f64 / num_nodes as f64;
            let x = 80.0 + t * 1040.0;
            let y = 300.0 + (t * PI * 3.0).sin() * 160.0 + (t * PI * 1.5).cos() * 80.0;

            Point::new(x, y)
        })
        .collect();

    // 累積距離 D_i の事前計算 (前回の scan を使用)
    let mut d = vec![0.0];
    d.extend(track_nodes.windows(2).scan(0.0, |acc, w| {
        let dx = w[1].x - w[0].x;
        let dy = w[1].y - w[0].y;
        *acc += dx.hypot(dy);
        Some(*acc)
    }));

    let total_track_length = *d.last().unwrap_or(&0.0);

    UseTrackOutput {
        track_nodes,
        d,
        total_track_length,
    }
}
