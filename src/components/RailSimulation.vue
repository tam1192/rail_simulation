<script setup lang="ts">
import { ref, reactive, onMounted, onUnmounted } from "vue";
import type { SimParams, DebugInfo } from "../types/track";
import type { Point, GraphicsEngine } from "../core/graphics/GraphicsEngine";
import { CanvasGraphicsEngine } from "../core/graphics/CanvasGraphicsEngine";
import { useTrack } from "../composables/useTrack";
import { renderTrackSim } from "../utils/trackRenderer";

const { trackNodes, totalTrackLength, getPointAtDistance, findRearDistance } =
    useTrack();

// パラメータ
const params = reactive<SimParams>({
    useCorrection: true,
    speed: 3,
    bogiePitch: 90,
});

const debugInfo = reactive<DebugInfo>({
    calculatedChordLength: 0,
});

// DOM参照
const canvasRef = ref<HTMLCanvasElement | null>(null);

// 描画関連（Vueのリアクティビティに含めず、純粋な変数として保持して軽量化）
let ctx: CanvasRenderingContext2D | null = null;
let engine: GraphicsEngine | null = null;
let animationFrameId: number | null = null;
let dFront = 120;

function update() {
    dFront += params.speed;
    if (dFront > totalTrackLength - 10) {
        dFront = 80; // ループ
    }
}

function loop() {
    if (ctx && engine) {
        // 位置更新
        update();

        // 台車位置計算
        const pFront = getPointAtDistance(dFront);
        let pRear: Point;

        if (params.useCorrection) {
            const dRear = findRearDistance(dFront, params.bogiePitch);
            pRear = getPointAtDistance(dRear);
        } else {
            pRear = getPointAtDistance(dFront - params.bogiePitch);
        }

        // デバッグ情報更新
        debugInfo.calculatedChordLength = Math.hypot(
            pFront.x - pRear.x,
            pFront.y - pRear.y
        );

        // 画面クリア＆描画実行
        ctx.clearRect(0, 0, ctx.canvas.width, ctx.canvas.height);
        renderTrackSim({ engine: engine, trackNodes, pFront, pRear, params });
    }

    animationFrameId = requestAnimationFrame(loop);
}

onMounted(() => {
    const canvas = canvasRef.value;
    if (canvas) {
        ctx = canvas.getContext("2d");
        if (ctx) {
            engine = new CanvasGraphicsEngine(ctx);
            loop();
        }
    }
});

onUnmounted(() => {
    if (animationFrameId !== null) {
        cancelAnimationFrame(animationFrameId);
    }
});
</script>

<template>
    <div
        class="flex flex-col items-center bg-[#181818] text-gray-200 p-5 min-h-screen"
    >
        <!-- コントロールパネル -->
        <div
            class="mb-4 flex flex-wrap items-center justify-center gap-6 bg-[#282828] px-6 py-3 rounded-lg shadow-lg border border-neutral-700"
        >
            <label
                class="flex items-center gap-2 cursor-pointer select-none text-sm"
            >
                <input
                    type="checkbox"
                    v-model="params.useCorrection"
                    class="rounded accent-blue-500 cursor-pointer"
                />
                <span>弦長補正（余弦引き込み）</span>
            </label>

            <label class="flex items-center gap-2 text-sm">
                <span>速度:</span>
                <input
                    type="range"
                    v-model.number="params.speed"
                    min="1"
                    max="12"
                    class="accent-blue-500 cursor-pointer"
                />
                <span class="w-6 text-right font-mono">{{ params.speed }}</span>
            </label>

            <label class="flex items-center gap-2 text-sm">
                <span>台車間距離(L):</span>
                <input
                    type="range"
                    v-model.number="params.bogiePitch"
                    min="50"
                    max="200"
                    class="accent-blue-500 cursor-pointer"
                />
                <span class="w-12 text-right font-mono"
                    >{{ params.bogiePitch }}px</span
                >
            </label>
        </div>

        <!-- Canvas -->
        <canvas
            ref="canvasRef"
            width="1200"
            height="650"
            class="bg-[#1f1f1f] border border-[#3a3a3a] rounded-md shadow-2xl max-w-full h-auto"
        ></canvas>

        <!-- デバッグパネル -->
        <div
            class="mt-4 font-mono text-sm w-full max-w-[1200px] text-left bg-[#222] p-3 rounded border border-neutral-800"
        >
            <div class="flex items-center gap-4">
                <span
                    >設定台車間距離 (L):
                    <strong class="text-blue-400 font-bold"
                        >{{ params.bogiePitch.toFixed(1) }} px</strong
                    ></span
                >
                <span class="text-neutral-600">|</span>
                <span
                    >実際の弦長:
                    <strong class="text-emerald-400 font-bold"
                        >{{
                            debugInfo.calculatedChordLength.toFixed(2)
                        }}
                        px</strong
                    ></span
                >
            </div>
            <p v-if="!params.useCorrection" class="text-red-400 text-xs mt-2">
                ※補正OFF：曲率の大きいカーブ通過時に台車間隔（弦長）が縮む様子が確認できます
            </p>
        </div>
    </div>
</template>
