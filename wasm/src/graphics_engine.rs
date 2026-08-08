use crate::types::Point;

/// 描画エンジンが満たすべき契約（Trait）
pub trait GraphicsEngine {
    /// 画面全体をクリアする
    fn clear(&mut self);

    #[warn(unused)]
    /// グリッドを描画する
    fn draw_grid(&mut self, step: f64, color: &str);

    /// 連続した線を描画する
    fn draw_path(&mut self, points: &[Point], stroke_color: &str, line_width: f64);

    /// 複数の点を描画する
    fn draw_dots(&mut self, points: &[Point], radius: f64, fill_color: &str);

    /// 円を描画する
    fn draw_circle(&mut self, center: Point, radius: f64, fill_color: &str);

    /// 線を描画する (is_dashed は Option<bool> で表現)
    fn draw_line(&mut self, from: Point, to: Point, color: &str, is_dashed: Option<bool>);

    /// 回転した矩形を描画する
    fn draw_rotated_rect(
        &mut self,
        center: Point,
        angle: f64,
        width: f64,
        height: f64,
        fill_color: &str,
        stroke_color: &str,
    );
}
