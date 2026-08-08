import type { SimParams } from '../types/track'
import type { Point, GraphicsEngine } from '../core/graphics/GraphicsEngine'

interface RenderOptions {
    engine: GraphicsEngine // ← Contextではなく抽象エンジンをDI
    trackNodes: Point[]
    pFront: Point
    pRear: Point
    params: SimParams
}

export function renderTrackSim({ engine, trackNodes, pFront, pRear, params }: RenderOptions) {
    // 画面クリア
    engine.clear()

    // 1. グリッド
    engine.drawGrid(50, '#2d2d2d')

    // 2. 枕木
    engine.drawPath(trackNodes, '#3a3a3a', 14)

    // 3. レール
    engine.drawPath(trackNodes, '#666666', 4)

    // 4. ノード点
    engine.drawDots(trackNodes, 3, '#888888')

    // 車体計算
    const centerX = (pFront.x + pRear.x) / 2
    const centerY = (pFront.y + pRear.y) / 2
    const angle = Math.atan2(pFront.y - pRear.y, pFront.x - pRear.x)
    const bodyLength = params.bogiePitch + 50
    const bodyWidth = 42

    // 5. 車体
    engine.drawRotatedRect(
        { x: centerX, y: centerY },
        angle,
        bodyLength,
        bodyWidth,
        'rgba(74, 144, 226, 0.75)',
        '#4A90E2'
    )

    // 6. 前後台車
    engine.drawCircle(pFront, 7, '#FF5252')
    engine.drawCircle(pRear, 7, '#448AFF')

    // 7. 弦（点線）
    engine.drawLine(pFront, pRear, 'rgba(255, 255, 255, 0.6)', true)
}
