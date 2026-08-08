import type { Point } from "../core/graphics/GraphicsEngine";

export function useTrack() {
    const trackNodes: Point[] = [];
    const numNodes = 400;

    // コースノードの生成
    for (let i = 0; i <= numNodes; i++) {
        const t = i / numNodes;
        const x = 80 + t * 1040;
        const y =
            300 +
            Math.sin(t * Math.PI * 3) * 160 +
            Math.cos(t * Math.PI * 1.5) * 80;

        trackNodes.push({ x, y });
    }

    // 累積距離 D_i の事前計算
    const D: number[] = [0];
    for (let i = 0; i < trackNodes.length - 1; i++) {
        const dx = trackNodes[i + 1].x - trackNodes[i].x;
        const dy = trackNodes[i + 1].y - trackNodes[i].y;
        D.push(D[i] + Math.hypot(dx, dy));
    }
    const totalTrackLength = D[D.length - 1];

    // 指定距離の座標を取得
    function getPointAtDistance(d: number): Point {
        const clampedD = Math.max(0, Math.min(d, totalTrackLength));
        for (let i = 0; i < D.length - 1; i++) {
            if (D[i] <= clampedD && clampedD <= D[i + 1]) {
                const t = (clampedD - D[i]) / (D[i + 1] - D[i]);
                return {
                    x:
                        trackNodes[i].x +
                        t * (trackNodes[i + 1].x - trackNodes[i].x),
                    y:
                        trackNodes[i].y +
                        t * (trackNodes[i + 1].y - trackNodes[i].y),
                };
            }
        }
        return trackNodes[trackNodes.length - 1];
    }

    // 後方台車の距離を二分探索（弦長補正）
    function findRearDistance(dFront: number, length: number): number {
        const pFront = getPointAtDistance(dFront);
        let low = Math.max(0, dFront - length * 1.8);
        let high = dFront;

        for (let iter = 0; iter < 15; iter++) {
            const mid = (low + high) / 2;
            const pMid = getPointAtDistance(mid);
            const dist = Math.hypot(pFront.x - pMid.x, pFront.y - pMid.y);

            if (dist < length) {
                high = mid;
            } else {
                low = mid;
            }
        }
        return (low + high) / 2;
    }

    return {
        trackNodes,
        totalTrackLength,
        getPointAtDistance,
        findRearDistance,
    };
}
