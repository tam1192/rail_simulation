import { ref, onMounted, onUnmounted } from "vue";

export function useTheme() {
    const isDarkMode = ref(false);
    const isSystemSetting = ref(true); // 手動設定かOS自動検知か

    // DOMへのdarkクラス適用 & localStorage保持
    const updateTheme = (isDark: boolean, savePreference = true) => {
        isDarkMode.value = isDark;

        if (isDark) {
            document.documentElement.classList.add("dark");
        } else {
            document.documentElement.classList.remove("dark");
        }

        if (savePreference) {
            isSystemSetting.value = false;
            localStorage.setItem("theme", isDark ? "dark" : "light");
        }
    };

    // 手動切り替え（トグルボタン用）
    const toggleDarkMode = () => {
        updateTheme(!isDarkMode.value, true);
    };

    // OS設定リセット（OSの設定に従う状態に戻す）
    const resetToSystem = () => {
        localStorage.removeItem("theme");
        isSystemSetting.value = true;
        const prefersDark = window.matchMedia(
            "(prefers-color-scheme: dark)"
        ).matches;
        updateTheme(prefersDark, false);
    };

    // OS設定変更イベントのハンドラ
    let mediaQuery: MediaQueryList | null = null;
    const handleSystemThemeChange = (e: MediaQueryListEvent) => {
        // ユーザーが手動選択していない（System追従）場合のみ反映
        if (isSystemSetting.value) {
            updateTheme(e.matches, false);
        }
    };

    onMounted(() => {
        mediaQuery = window.matchMedia("(prefers-color-scheme: dark)");
        const savedTheme = localStorage.getItem("theme");

        if (savedTheme) {
            // 1. ユーザーの過去の保存設定があれば優先
            isSystemSetting.value = false;
            updateTheme(savedTheme === "dark", false);
        } else {
            // 2. なければOSの設定を初期値にする
            isSystemSetting.value = true;
            updateTheme(mediaQuery.matches, false);
        }

        // 3. OSのテーマ変更をリアルタイム監視
        mediaQuery.addEventListener("change", handleSystemThemeChange);
    });

    onUnmounted(() => {
        if (mediaQuery) {
            mediaQuery.removeEventListener("change", handleSystemThemeChange);
        }
    });

    return {
        isDarkMode,
        isSystemSetting,
        toggleDarkMode,
        resetToSystem,
    };
}
