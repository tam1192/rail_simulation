<script setup lang="ts">
import { ref } from "vue";
import { useTheme } from "../composables/useTheme";
import { routes } from "../router/index";

const { isDarkMode, toggleDarkMode } = useTheme();
const isMenuOpen = ref(false);

function toggleMenu() {
    isMenuOpen.value = !isMenuOpen.value;
}
</script>

<template>
    <header
        class="sticky top-0 z-50 bg-surface-primary border-b border-surface-secondary transition-colors duration-200"
    >
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
            <div class="flex items-center justify-between h-16">
                <div class="flex-shrink-0">
                    <RouterLink
                        to="/"
                        class="text-xl font-bold text-brand hover:opacity-80 transition-opacity"
                    >
                        Simulations
                    </RouterLink>
                </div>

                <nav class="hidden md:flex space-x-8">
                    <RouterLink
                        v-for="item in routes"
                        :key="item.path"
                        :to="item.path"
                        class="text-content-muted hover:text-brand font-medium transition-colors"
                        active-class="text-brand font-bold"
                    >
                        {{ String(item.name ?? item.path) }}
                    </RouterLink>
                </nav>

                <div class="flex items-center space-x-3">
                    <button
                        @click="toggleDarkMode"
                        type="button"
                        class="p-2 rounded-lg bg-surface-secondary text-content-main hover:opacity-80 transition-all flex items-center justify-center"
                        aria-label="Toggle Dark Mode"
                    >
                        <span v-if="isDarkMode" class="text-sm">☀️</span>
                        <span v-else class="text-sm">🌙</span>
                    </button>

                    <div class="md:hidden">
                        <button
                            @click="toggleMenu"
                            type="button"
                            class="p-2 rounded-md text-content-main hover:bg-surface-secondary focus:outline-none transition-colors"
                        >
                            <svg
                                class="h-6 w-6"
                                fill="none"
                                viewBox="0 0 24 24"
                                stroke="currentColor"
                            >
                                <path
                                    v-if="!isMenuOpen"
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="2"
                                    d="M4 6h16M4 12h16M4 18h16"
                                />
                                <path
                                    v-else
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="2"
                                    d="M6 18L18 6M6 6l12 12"
                                />
                            </svg>
                        </button>
                    </div>
                </div>
            </div>
        </div>

        <div
            v-if="isMenuOpen"
            class="md:hidden bg-surface-primary border-t border-surface-secondary"
        >
            <div class="px-2 pt-2 pb-3 space-y-1">
                <RouterLink
                    v-for="item in routes"
                    :key="item.path"
                    :to="item.path"
                    @click="isMenuOpen = false"
                    class="block px-3 py-2 rounded-md text-base font-medium text-content-main hover:text-brand hover:bg-surface-secondary transition-colors"
                    active-class="bg-surface-secondary text-brand font-bold"
                >
                    {{ String(item.name ?? item.path) }}
                </RouterLink>
            </div>
        </div>
    </header>
</template>
