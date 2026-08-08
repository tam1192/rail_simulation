import type { GraphicsEngine, Point } from './GraphicsEngine';

export class CanvasGraphicsEngine implements GraphicsEngine {
  // 明示的にフィールド宣言 (erasableSyntaxOnly 対策)
  private ctx: CanvasRenderingContext2D;

  constructor(ctx: CanvasRenderingContext2D) {
    this.ctx = ctx;
  }

  /**
   * 画面全体をクリア
   */
  clear(): void {
    const { width, height } = this.ctx.canvas;
    this.ctx.clearRect(0, 0, width, height);
  }

  /**
   * グリッド線の描画
   */
  drawGrid(step: number, color: string): void {
    const { width, height } = this.ctx.canvas;
    this.ctx.save();
    this.ctx.strokeStyle = color;
    this.ctx.lineWidth = 1;

    this.ctx.beginPath();
    // 垂直線
    for (let x = 0; x <= width; x += step) {
      this.ctx.moveTo(x, 0);
      this.ctx.lineTo(x, height);
    }
    // 水平線
    for (let y = 0; y <= height; y += step) {
      this.ctx.moveTo(0, y);
      this.ctx.lineTo(width, y);
    }
    this.ctx.stroke();
    this.ctx.restore();
  }

  /**
   * 複数の点を繋ぐパスの描画
   */
  drawPath(points: Point[], strokeColor: string, lineWidth: number): void {
    if (points.length < 2) return;

    this.ctx.save();
    this.ctx.strokeStyle = strokeColor;
    this.ctx.lineWidth = lineWidth;

    this.ctx.beginPath();
    this.ctx.moveTo(points[0].x, points[0].y);
    for (let i = 1; i < points.length; i++) {
      this.ctx.lineTo(points[i].x, points[i].y);
    }
    this.ctx.stroke();
    this.ctx.restore();
  }

  /**
   * ドット（複数の円）の描画
   */
  drawDots(points: Point[], radius: number, fillColor: string): void {
    this.ctx.save();
    this.ctx.fillStyle = fillColor;

    for (const pt of points) {
      this.ctx.beginPath();
      this.ctx.arc(pt.x, pt.y, radius, 0, Math.PI * 2);
      this.ctx.fill();
    }
    this.ctx.restore();
  }

  /**
   * 円の描画
   */
  drawCircle(center: Point, radius: number, fillColor: string): void {
    this.ctx.save();
    this.ctx.fillStyle = fillColor;
    this.ctx.beginPath();
    this.ctx.arc(center.x, center.y, radius, 0, Math.PI * 2);
    this.ctx.fill();
    this.ctx.restore();
  }

  /**
   * 直線の描画 (破線対応)
   */
  drawLine(from: Point, to: Point, color: string, isDashed = false): void {
    this.ctx.save();
    this.ctx.strokeStyle = color;

    if (isDashed) {
      this.ctx.setLineDash([5, 5]); // 破線パターン [描画の長さ, 空白の長さ]
    }

    this.ctx.beginPath();
    this.ctx.moveTo(from.x, from.y);
    this.ctx.lineTo(to.x, to.y);
    this.ctx.stroke();

    this.ctx.restore(); // restore することで破線設定も元に戻る
  }

  /**
   * 回転付き矩形の描画
   */
  drawRotatedRect(
    center: Point,
    angle: number, // ラジアン指定 (度数の場合は angle * Math.PI / 180)
    width: number,
    height: number,
    fillColor: string,
    strokeColor: string
  ): void {
    this.ctx.save();

    // 1. 中心点へ移動して回転
    this.ctx.translate(center.x, center.y);
    this.ctx.rotate(angle);

    // 2. スタイル設定
    this.ctx.fillStyle = fillColor;
    this.ctx.strokeStyle = strokeColor;

    // 3. 原点(中心)に合わせてオフセット描画
    const x = -width / 2;
    const y = -height / 2;

    this.ctx.fillRect(x, y, width, height);
    this.ctx.strokeRect(x, y, width, height);

    // 4. 行列と座標系を元に戻す (超重要)
    this.ctx.restore();
  }
}
