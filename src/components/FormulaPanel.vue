<script setup lang="ts">
defineProps<{
    trace: string;
}>();
</script>

<template>
    <section
        class="mt-4 font-mono text-sm w-full max-w-[1200px] text-left bg-surface-primary p-4 rounded border border-surface-secondary"
    >
        <h2 class="text-xs uppercase tracking-wide text-content-muted mb-2">
            途中式
        </h2>
        <pre class="whitespace-pre-wrap text-content-main leading-6">{{
            trace || "—"
        }}</pre>
    </section>
</template>
