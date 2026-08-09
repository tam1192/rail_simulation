<script setup lang="ts">
import { ref, reactive, onMounted, onUnmounted } from "vue";
import init, {
    RailSimulation,
    DebugInfo,
} from "../../wasm/pkg/rail_simulation_wasm";

type SimParams = {
    useCorrection: boolean;
    speed: number;
    bogiePitch: number;
};

const wasmReady = ref(false);
const engine = ref<RailSimulation | null>(null);
const debugInfo = ref<DebugInfo | null>(null);

// パラメータ
const params = reactive<SimParams>({
    useCorrection: true,
    speed: 3,
    bogiePitch: 90,
});

// DOM参照
const canvasRef = ref<HTMLCanvasElement | null>(null);
let animationFrameId: number | null = null;

function loop() {
    if (engine.value) {
        debugInfo.value = engine.value.execute(
            params.useCorrection,
            params.speed,
            params.bogiePitch
        );
    }

    animationFrameId = requestAnimationFrame(loop);
}

onMounted(async () => {
    try {
        await init();
        wasmReady.value = true;

        const canvas = canvasRef.value;
        if (canvas) {
            engine.value = new RailSimulation(canvas);
        }
        loop();
    } catch (error) {
        console.error(error);
        wasmReady.value = false;
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
        class="flex flex-col items-center p-5 min-h-screen bg-surface-primary text-content-main transition-colors duration-200">
        <!-- コントロールパネル -->
        <div
            class="mb-4 flex flex-wrap items-center justify-center gap-6 bg-surface-primary px-6 py-3 rounded-lg shadow-lg border border-surface-secondary">
            <!-- 弦長補正チェックボックス -->
            <label class="flex items-center gap-2 cursor-pointer select-none text-sm text-content-main">
                <input type="checkbox" v-model="params.useCorrection" class="rounded accent-brand cursor-pointer" />
                <span>弦長補正（余弦引き込み）</span>
            </label>

            <!-- 速度スライダー -->
            <label class="flex items-center gap-2 text-sm text-content-main">
                <span>速度:</span>
                <input type="range" v-model.number="params.speed" min="1" max="12"
                    class="accent-brand cursor-pointer" />
                <span class="w-6 text-right font-mono text-content-muted">{{ params.speed }}</span>
            </label>

            <!-- 台車間距離スライダー -->
            <label class="flex items-center gap-2 text-sm text-content-main">
                <span>台車間距離(L):</span>
                <input type="range" v-model.number="params.bogiePitch" min="50" max="200"
                    class="accent-brand cursor-pointer" />
                <span class="w-12 text-right font-mono text-content-muted">{{ params.bogiePitch }}px</span>
            </label>
        </div>

        <!-- Canvas -->
        <canvas ref="canvasRef" width="1200" height="650"
            class="bg-surface-secondary border border-surface-secondary rounded-md shadow-2xl max-w-full h-auto"></canvas>

        <!-- デバッグパネル -->
        <div
            class="mt-4 font-mono text-sm w-full max-w-[1200px] text-left bg-surface-primary p-3 rounded border border-surface-secondary flex flex-col gap-2">
            <div class="flex items-center gap-4">
                <span class="text-content-muted">設定台車間距離 (L):
                    <strong class="text-brand font-bold">{{ params.bogiePitch.toFixed(1) }} px</strong></span>
                <span class="text-surface-secondary">|</span>
                <span class="text-content-muted">実際の弦長:
                    <strong class="text-emerald-600 dark:text-emerald-400 font-bold">{{
                        debugInfo?.calculated_chord_length.toFixed(2)
                        }}
                        px</strong></span>
            </div>
            <p v-if="!params.useCorrection" class="text-rose-600 dark:text-rose-400 text-xs mt-1">
                ※補正OFF：曲率の大きいカーブ通過時に台車間隔（弦長）が縮む様子が確認できます
            </p>
        </div>
    </div>
</template>
