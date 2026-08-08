// 描画エンジンが満たすべき契約
export interface GraphicsEngine {
    clear(): void;
    drawGrid(step: number, color: string): void;
    drawPath(points: Point[], strokeColor: string, lineWidth: number): void;
    drawDots(points: Point[], radius: number, fillColor: string): void;
    drawCircle(center: Point, radius: number, fillColor: string): void;
    drawLine(from: Point, to: Point, color: string, isDashed?: boolean): void;
    drawRotatedRect(
        center: Point,
        angle: number,
        width: number,
        height: number,
        fillColor: string,
        strokeColor: string
    ): void;
}

export interface Point {
    x: number;
    y: number;
}
