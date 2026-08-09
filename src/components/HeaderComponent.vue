<script setup>
import { ref, onMounted } from "vue";
import { useTheme } from '../composables/useTheme'
import { routes } from '../router/index'

// 作成した Composable から状態と関数を取得
const { isDarkMode, toggleDarkMode } = useTheme()

// モバイルメニューの開閉状態
const isMenuOpen = ref(false);
const toggleMenu = () => {
    isMenuOpen.value = !isMenuOpen.value;
};

// 初期化（OS設定およびローカルストレージの保持状態をチェック）
onMounted(() => {
    const savedTheme = localStorage.getItem("theme");
    const prefersDark = window.matchMedia(
        "(prefers-color-scheme: dark)"
    ).matches;

    if (savedTheme === "dark" || (!savedTheme && prefersDark)) {
        isDarkMode.value = true;
        document.documentElement.classList.add("dark");
    }
});

</script>

<template>
    <header
        class="sticky top-0 z-50 bg-surface-primary border-b border-surface-secondary transition-colors duration-200">
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
            <div class="flex items-center justify-between h-16">
                <!-- ロゴ (ブランドカラー適用) -->
                <div class="flex-shrink-0">
                    <RouterLink to="/" class="text-xl font-bold text-brand hover:opacity-80 transition-opacity">
                        Simulations
                    </RouterLink>
                </div>

                <!-- デスクトップナビゲーション -->
                <nav class="hidden md:flex space-x-8">
                    <RouterLink v-for="item in routes" :key="item.path" :to="item.path"
                        class="text-content-muted hover:text-brand font-medium transition-colors"
                        active-class="text-brand font-bold">
                        {{ item.name }}
                    </RouterLink>
                </nav>

                <!-- 右側アクション領域 (ダークモードボタン + ハンバーガー) -->
                <div class="flex items-center space-x-3">
                    <!-- ダークモード切り替えボタン -->
                    <button @click="toggleDarkMode" type="button"
                        class="p-2 rounded-lg bg-surface-secondary text-content-main hover:opacity-80 transition-all flex items-center justify-center"
                        aria-label="Toggle Dark Mode">
                        <span v-if="isDarkMode" class="text-sm">☀️</span>
                        <span v-else class="text-sm">🌙</span>
                    </button>

                    <!-- モバイル用ハンバーガーボタン -->
                    <div class="md:hidden">
                        <button @click="toggleMenu" type="button"
                            class="p-2 rounded-md text-content-main hover:bg-surface-secondary focus:outline-none transition-colors">
                            <svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path v-if="!isMenuOpen" stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                    d="M4 6h16M4 12h16M4 18h16" />
                                <path v-else stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                    d="M6 18L18 6M6 6l12 12" />
                            </svg>
                        </button>
                    </div>
                </div>
            </div>
        </div>

        <!-- モバイル用メニュー -->
        <div v-if="isMenuOpen" class="md:hidden bg-surface-primary border-t border-surface-secondary">
            <div class="px-2 pt-2 pb-3 space-y-1">
                <RouterLink v-for="item in routes" :key="item.path" :to="item.path" @click="isMenuOpen = false"
                    class="block px-3 py-2 rounded-md text-base font-medium text-content-main hover:text-brand hover:bg-surface-secondary transition-colors"
                    active-class="bg-surface-secondary text-brand font-bold">
                    {{ item.name }}
                </RouterLink>
            </div>
        </div>
    </header>
</template>
