use std::f64::consts::PI;

use crate::domain::formula::{num, FormulaTrace};
use crate::types::Point;

const LOOP_RESET_DISTANCE: f64 = 80.0;
const LOOP_END_MARGIN: f64 = 10.0;
const INITIAL_FRONT_DISTANCE: f64 = 120.0;

pub struct Track {
    pub nodes: Vec<Point>,
    distances: Vec<f64>,
    pub total_length: f64,
}

impl Track {
    pub fn wavy_course() -> Self {
        let num_nodes = 400;
        let nodes: Vec<Point> = (0..=num_nodes)
            .map(|i| {
                let t = i as f64 / num_nodes as f64;
                let x = 80.0 + t * 1040.0;
                let y = 300.0 + (t * PI * 3.0).sin() * 160.0 + (t * PI * 1.5).cos() * 80.0;
                Point::new(x, y)
            })
            .collect();

        let mut distances = vec![0.0];
        distances.extend(nodes.windows(2).scan(0.0, |acc, w| {
            let dx = w[1].x - w[0].x;
            let dy = w[1].y - w[0].y;
            *acc += dx.hypot(dy);
            Some(*acc)
        }));
        let total_length = *distances.last().unwrap_or(&0.0);

        Self {
            nodes,
            distances,
            total_length,
        }
    }

    pub fn point_at(&self, distance: f64) -> Point {
        let clamped = distance.clamp(0.0, self.total_length);

        for i in 0..(self.distances.len() - 1) {
            if self.distances[i] <= clamped && clamped <= self.distances[i + 1] {
                let segment_len = self.distances[i + 1] - self.distances[i];
                if segment_len == 0.0 {
                    return self.nodes[i];
                }

                let t = (clamped - self.distances[i]) / segment_len;
                let x = self.nodes[i].x + t * (self.nodes[i + 1].x - self.nodes[i].x);
                let y = self.nodes[i].y + t * (self.nodes[i + 1].y - self.nodes[i].y);
                return Point::new(x, y);
            }
        }

        *self.nodes.last().unwrap()
    }

    /// Chord-length correction: binary search for a rear distance whose
    /// Euclidean distance to the front point equals `length`.
    pub fn find_rear_distance(&self, d_front: f64, length: f64) -> (f64, FormulaTrace) {
        let p_front = self.point_at(d_front);
        let mut low = (d_front - length * 1.8).max(0.0);
        let mut high = d_front;
        let mut last_mid = low;
        let mut last_dist = 0.0;

        for _ in 0..15 {
            last_mid = (low + high) / 2.0;
            let p_mid = self.point_at(last_mid);
            last_dist = (p_front.x - p_mid.x).hypot(p_front.y - p_mid.y);

            if last_dist < length {
                high = last_mid;
            } else {
                low = last_mid;
            }
        }

        let d_rear = (low + high) / 2.0;
        let trace = vec![
            "|P(d_f) − P(d_r)| = L  となる d_r を二分探索".to_string(),
            format!(
                "d_r ∈ [d_f − 1.8L, d_f] = [{}, {}]",
                num((d_front - length * 1.8).max(0.0)),
                num(d_front)
            ),
            format!(
                "15回目: mid = (low + high) / 2 = ({low} + {high}) / 2 = {mid}",
                low = num(low),
                high = num(high),
                mid = num(last_mid)
            ),
            format!(
                "|P(d_f) − P(mid)| = {}  {}  L = {}",
                num(last_dist),
                if last_dist < length { "<" } else { "≥" },
                num(length)
            ),
            format!("d_r = {}", num(d_rear)),
        ];
        (d_rear, trace)
    }
}

pub struct RailState {
    pub track: Track,
    pub d_front: f64,
}

impl RailState {
    pub fn new() -> Self {
        Self {
            track: Track::wavy_course(),
            d_front: INITIAL_FRONT_DISTANCE,
        }
    }

    pub fn tick(&mut self, speed: f64) {
        self.d_front += speed;
        if self.d_front > self.track.total_length - LOOP_END_MARGIN {
            self.d_front = LOOP_RESET_DISTANCE;
        }
    }

    pub fn bogie_points(
        &self,
        use_correction: bool,
        bogie_pitch: f64,
    ) -> (Point, Point, FormulaTrace) {
        let p_front = self.track.point_at(self.d_front);
        let mut trace = vec![
            format!("d_f = {}", num(self.d_front)),
            format!("L = {}", num(bogie_pitch)),
        ];

        let p_rear = if use_correction {
            let (d_rear, search_trace) = self.track.find_rear_distance(self.d_front, bogie_pitch);
            trace.extend(search_trace);
            self.track.point_at(d_rear)
        } else {
            let d_rear = self.d_front - bogie_pitch;
            trace.push("補正OFF: 弧長をそのまま引く".to_string());
            trace.push("d_r = d_f − L".to_string());
            trace.push(format!(
                "    = {} − {} = {}",
                num(self.d_front),
                num(bogie_pitch),
                num(d_rear)
            ));
            self.track.point_at(d_rear)
        };

        let dx = p_front.x - p_rear.x;
        let dy = p_front.y - p_rear.y;
        let chord = dx.hypot(dy);
        trace.push("c = |P(d_f) − P(d_r)|".to_string());
        trace.push("  = √((x_f − x_r)² + (y_f − y_r)²)".to_string());
        trace.push(format!(
            "  = √(({} − {})² + ({} − {})²)",
            num(p_front.x),
            num(p_rear.x),
            num(p_front.y),
            num(p_rear.y)
        ));
        trace.push(format!(
            "  = √({} + {}) = {}",
            num(dx * dx),
            num(dy * dy),
            num(chord)
        ));
        if use_correction {
            trace.push(format!("c ≈ L ({})", num(bogie_pitch)));
        } else {
            trace.push(format!(
                "c ≟ L → {} ≟ {}  （カーブでは弦が弧より短い）",
                num(chord),
                num(bogie_pitch)
            ));
        }

        (p_front, p_rear, trace)
    }
}
