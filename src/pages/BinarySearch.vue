<script setup lang="ts">
import { ref, reactive, onMounted, onUnmounted } from "vue";
// WASMのビルド出力パスに合わせてインポートパスを適宜変更してください
import initWasm, { BinarySearch } from "../../wasm/pkg";

// --- DOM参照 & WASMインスタンス ---
const canvasRef = ref<HTMLCanvasElement | null>(null);
let searchInstance: BinarySearch | null = null;
let timerId: number | null = null;

// --- State（コントロールパネル用） ---
const params = reactive({
    targetValue: 45,
    arraySize: 15,
    stepSpeed: 500, // ms
});

// --- State（デバッグ・ステータス表示用） ---
const visualizerState = reactive({
    status: "idle" as "idle" | "searching" | "found" | "not_found",
    currentStep: 0,
    lowIndex: -1,
    midIndex: -1,
    highIndex: -1,
    message:
        "探索目標と要素数を設定して「自動再生」または「コマ送り」を押してください",
});

// WASMのステータス(SearchStatus)を Vue 側の State に反映するヘルパー
function syncStatus() {
    if (!searchInstance) return;
    const status = searchInstance.get_status();

    visualizerState.currentStep = status.current_step;
    visualizerState.lowIndex = status.low;
    visualizerState.midIndex = status.mid;
    visualizerState.highIndex = status.high;

    if (status.is_found) {
        visualizerState.status = "found";
        visualizerState.message = `目標値 (${params.targetValue}) が Index ${status.mid} で見つかりました！`;
        stopTimer();
    } else if (status.is_finished) {
        visualizerState.status = "not_found";
        visualizerState.message = `目標値 (${params.targetValue}) は配列内に存在しませんでした。`;
        stopTimer();
    } else if (status.current_step > 0) {
        visualizerState.status = "searching";
        visualizerState.message = `Index ${status.mid} を確認中...`;
    }
}

// 自動再生用タイマーの停止
function stopTimer() {
    if (timerId !== null) {
        clearInterval(timerId);
        timerId = null;
    }
}

// --- イベントハンドラ ---

// 初期化・リセット
function handleReset() {
    stopTimer();
    if (!searchInstance && canvasRef.value) return;

    // WASM側でリセットと再描画を実行
    searchInstance?.reset(params.arraySize, params.targetValue);

    visualizerState.status = "idle";
    visualizerState.currentStep = 0;
    visualizerState.lowIndex = -1;
    visualizerState.midIndex = -1;
    visualizerState.highIndex = -1;
    visualizerState.message = "リセットしました。";
}

// コマ送り (1ステップ実行)
function handleStepNext() {
    if (!searchInstance) return;

    if (
        visualizerState.status === "found" ||
        visualizerState.status === "not_found"
    ) {
        return;
    }

    visualizerState.status = "searching";
    searchInstance.step();
    syncStatus();
}

// 自動再生
function handleStart() {
    if (!searchInstance) return;

    // すでに完了している場合はリセットしてから再生
    if (
        visualizerState.status === "found" ||
        visualizerState.status === "not_found"
    ) {
        handleReset();
    }

    visualizerState.status = "searching";
    stopTimer();

    // 1回目を即時実行
    searchInstance.step();
    syncStatus();

    // 以降、設定スピードごとに step() を実行
    if (visualizerState.status === "searching") {
        timerId = window.setInterval(() => {
            if (searchInstance) {
                searchInstance.step();
                syncStatus();
            }
        }, params.stepSpeed);
    }
}

// 初期化処理
onMounted(async () => {
    if (!canvasRef.value) return;

    // WASMの読み込みと初期化
    await initWasm();
    searchInstance = new BinarySearch(
        canvasRef.value,
        params.arraySize,
        params.targetValue
    );
});

onUnmounted(() => {
    stopTimer();
    searchInstance = null;
});
</script>

<template>
    <div
        class="flex flex-col items-center p-5 min-h-screen bg-surface-primary text-content-main transition-colors duration-200">
        <!-- コントロールパネル -->
        <div
            class="mb-4 flex flex-wrap items-center justify-center gap-6 bg-surface-primary px-6 py-3 rounded-lg shadow-lg border border-surface-secondary">
            <!-- ターゲット値入力 -->
            <label class="flex items-center gap-2 text-sm text-content-main">
                <span>探索目標 (Target):</span>
                <input type="number" v-model.number="params.targetValue" @change="handleReset" min="1" max="200"
                    class="w-20 bg-surface-secondary border border-surface-secondary rounded px-2 py-1 font-mono text-brand focus:outline-none focus:border-brand" />
            </label>

            <!-- 配列サイズ変更 -->
            <label class="flex items-center gap-2 text-sm text-content-main">
                <span>要素数 (N):</span>
                <input type="range" v-model.number="params.arraySize" @input="handleReset" min="5" max="30"
                    class="accent-brand cursor-pointer" />
                <span class="w-6 font-mono text-right text-content-muted">{{
                    params.arraySize
                    }}</span>
            </label>

            <!-- 再生スピード -->
            <label class="flex items-center gap-2 text-sm text-content-main">
                <span>速度:</span>
                <input type="range" v-model.number="params.stepSpeed" min="100" max="1500" step="100"
                    class="accent-brand cursor-pointer" />
                <span class="w-12 font-mono text-right text-content-muted">{{ params.stepSpeed }}ms</span>
            </label>

            <div class="h-5 w-[1px] bg-surface-secondary mx-1"></div>

            <!-- 操作ボタン群 -->
            <div class="flex items-center gap-2">
                <button @click="handleStart" :disabled="timerId !== null"
                    class="px-3 py-1.5 bg-brand hover:opacity-90 disabled:opacity-50 text-white font-medium rounded text-sm transition-colors cursor-pointer disabled:cursor-not-allowed">
                    {{ timerId !== null ? "再生中..." : "自動再生" }}
                </button>
                <button @click="handleStepNext" :disabled="timerId !== null"
                    class="px-3 py-1.5 bg-surface-secondary hover:opacity-80 disabled:opacity-50 text-content-main font-medium rounded text-sm transition-colors cursor-pointer">
                    コマ送り
                </button>
                <button @click="handleReset"
                    class="px-3 py-1.5 bg-surface-secondary hover:opacity-80 text-content-muted border border-surface-secondary font-medium rounded text-sm transition-colors cursor-pointer">
                    リセット
                </button>
            </div>
        </div>

        <!-- Visualizer Canvas -->
        <canvas ref="canvasRef" width="1200" height="650"
            class="bg-surface-secondary border border-surface-secondary rounded-md shadow-2xl max-w-full h-auto"></canvas>

        <!-- デバッグ / ステータス表示パネル -->
        <div
            class="mt-4 font-mono text-sm w-full max-w-[1200px] text-left bg-surface-primary p-4 rounded border border-surface-secondary flex flex-col gap-2">
            <div class="flex items-center justify-between border-b border-surface-secondary pb-2">
                <div class="flex items-center gap-4">
                    <span class="text-content-muted">ステータス:
                        <strong :class="{
                            'text-brand': visualizerState.status === 'searching',
                            'text-emerald-500 dark:text-emerald-400': visualizerState.status === 'found',
                            'text-rose-500 dark:text-rose-400': visualizerState.status === 'not_found',
                            'text-content-muted': visualizerState.status === 'idle',
                        }">
                            {{ visualizerState.status.toUpperCase() }}
                        </strong>
                    </span>
                    <span class="text-surface-secondary">|</span>
                    <span class="text-content-muted">Step:
                        <strong class="text-amber-500 dark:text-amber-400">{{
                            visualizerState.currentStep
                            }}</strong></span>
                </div>

                <!-- Pointer 情報 (WASMのLow/Mid/Highと同期) -->
                <div class="flex items-center gap-4 text-xs">
                    <span class="flex items-center gap-1">
                        <span class="w-2.5 h-2.5 rounded-full bg-purple-500 inline-block"></span>
                        Low:
                        <strong class="text-purple-600 dark:text-purple-400">{{
                            visualizerState.lowIndex >= 0
                                ? visualizerState.lowIndex
                                : "-"
                        }}</strong>
                    </span>
                    <span class="flex items-center gap-1">
                        <span class="w-2.5 h-2.5 rounded-full bg-amber-500 inline-block"></span>
                        Mid:
                        <strong class="text-amber-600 dark:text-amber-400">{{
                            visualizerState.midIndex >= 0
                                ? visualizerState.midIndex
                                : "-"
                        }}</strong>
                    </span>
                    <span class="flex items-center gap-1">
                        <span class="w-2.5 h-2.5 rounded-full bg-cyan-500 inline-block"></span>
                        High:
                        <strong class="text-cyan-600 dark:text-cyan-400">{{
                            visualizerState.highIndex >= 0
                                ? visualizerState.highIndex
                                : "-"
                        }}</strong>
                    </span>
                </div>
            </div>

            <!-- メッセージエリア -->
            <div class="text-xs text-content-muted pt-1">
                💡 {{ visualizerState.message }}
            </div>
        </div>
    </div>
</template>