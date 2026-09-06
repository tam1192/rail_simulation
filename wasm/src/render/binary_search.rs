use crate::domain::binary_search::BinarySearchState;
use crate::graphics_engine::GraphicsEngine;

const COLOR_OUTSIDE: &str = "#333333";
const COLOR_RANGE: &str = "#2563eb";
const COLOR_MID: &str = "#f59e0b";
const COLOR_FOUND: &str = "#10b981";
const COLOR_VALUE: &str = "#ffffff";
const COLOR_INDEX: &str = "#888888";
const COLOR_LOW: &str = "#a855f7";
const COLOR_HIGH: &str = "#06b6d4";

pub fn render_binary_search(engine: &mut dyn GraphicsEngine, state: &BinarySearchState) {
    engine.clear();
    if state.data.is_empty() {
        return;
    }

    let (width, height) = engine.size();
    let padding = 50.0;
    let available_width = width - (padding * 2.0);
    let bar_width = available_width / state.data.len() as f64;

    for (i, &val) in state.data.iter().enumerate() {
        let x = padding + (i as f64 * bar_width);
        let y = height / 2.0;

        let mut fill_color = COLOR_OUTSIDE;
        if i >= state.low && i <= state.high {
            fill_color = COLOR_RANGE;
        }
        if state.step > 0 && i == state.mid {
            fill_color = if state.is_found {
                COLOR_FOUND
            } else {
                COLOR_MID
            };
        }

        engine.fill_rect(x + 2.0, y - 20.0, bar_width - 4.0, 40.0, fill_color);
        engine.draw_text(
            &val.to_string(),
            x + (bar_width / 4.0),
            y + 5.0,
            COLOR_VALUE,
            "14px monospace",
        );
        engine.draw_text(
            &i.to_string(),
            x + (bar_width / 4.0),
            y + 35.0,
            COLOR_INDEX,
            "10px monospace",
        );
    }

    if state.step > 0 {
        let low_x = padding + (state.low as f64 * bar_width) + (bar_width / 3.0);
        engine.draw_text(
            "▲ Low",
            low_x,
            height / 2.0 - 30.0,
            COLOR_LOW,
            "14px monospace",
        );

        let high_x = padding + (state.high as f64 * bar_width) + (bar_width / 3.0);
        engine.draw_text(
            "▲ High",
            high_x,
            height / 2.0 + 55.0,
            COLOR_HIGH,
            "14px monospace",
        );
    }
}
