<script setup lang="ts">
import { ref, reactive, onMounted, onUnmounted } from "vue";
import init, { RailSimulation, DebugInfo } from "../../wasm/pkg"

type SimParams = {
    useCorrection: boolean
    speed: number
    bogiePitch: number
}

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
        )
    }

    animationFrameId = requestAnimationFrame(loop);
}

onMounted(async () => {
    try {
        await init();
        wasmReady.value = true;

        const canvas = canvasRef.value;
        if (canvas) {
            engine.value = new RailSimulation(canvas)
        }
        loop();
    } catch (error) {
        console.error(error);
        wasmReady.value = false;
    }
})

onUnmounted(() => {
    if (animationFrameId !== null) {
        cancelAnimationFrame(animationFrameId);
    }
});
</script>

<template>
    <div class="flex flex-col items-center bg-[#181818] text-gray-200 p-5 min-h-screen">
        <!-- コントロールパネル -->
        <div
            class="mb-4 flex flex-wrap items-center justify-center gap-6 bg-[#282828] px-6 py-3 rounded-lg shadow-lg border border-neutral-700">
            <label class="flex items-center gap-2 cursor-pointer select-none text-sm">
                <input type="checkbox" v-model="params.useCorrection" class="rounded accent-blue-500 cursor-pointer" />
                <span>弦長補正（余弦引き込み）</span>
            </label>

            <label class="flex items-center gap-2 text-sm">
                <span>速度:</span>
                <input type="range" v-model.number="params.speed" min="1" max="12"
                    class="accent-blue-500 cursor-pointer" />
                <span class="w-6 text-right font-mono">{{ params.speed }}</span>
            </label>

            <label class="flex items-center gap-2 text-sm">
                <span>台車間距離(L):</span>
                <input type="range" v-model.number="params.bogiePitch" min="50" max="200"
                    class="accent-blue-500 cursor-pointer" />
                <span class="w-12 text-right font-mono">{{ params.bogiePitch }}px</span>
            </label>
        </div>

        <!-- Canvas -->
        <canvas ref="canvasRef" width="1200" height="650"
            class="bg-[#1f1f1f] border border-[#3a3a3a] rounded-md shadow-2xl max-w-full h-auto"></canvas>

        <!-- デバッグパネル -->
        <div
            class="mt-4 font-mono text-sm w-full max-w-[1200px] text-left bg-[#222] p-3 rounded border border-neutral-800">
            <div class="flex items-center gap-4">
                <span>設定台車間距離 (L):
                    <strong class="text-blue-400 font-bold">{{ params.bogiePitch.toFixed(1) }} px</strong></span>
                <span class="text-neutral-600">|</span>
                <span>実際の弦長:
                    <strong class="text-emerald-400 font-bold">{{
                        debugInfo?.calculated_chord_length.toFixed(2)
                        }}
                        px</strong></span>
            </div>
            <p v-if="!params.useCorrection" class="text-red-400 text-xs mt-2">
                ※補正OFF：曲率の大きいカーブ通過時に台車間隔（弦長）が縮む様子が確認できます
            </p>
        </div>
    </div>
</template>
