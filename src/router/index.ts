import { createRouter, createWebHistory, type RouteRecordRaw } from "vue-router"; 

export const routes = [
    {
        path: "/",
        name: "RailSimulation",
        component: async () => {
            const rs = await import("../pages/RailSimulation.vue");
            return rs;
        },
    },
    {
        path: "/bs",
        name: "BinarySearch",
        component: async () => {
            const bs = await import("../pages/BinarySearch.vue");
            return bs;
        },
    }
]

const router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes,
});

export default router;