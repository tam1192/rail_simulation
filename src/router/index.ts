import {
    createRouter,
    createWebHistory,
    type RouteRecordRaw,
} from "vue-router";

export const routes: RouteRecordRaw[] = [
    {
        path: "/",
        name: "RailSimulation",
        component: () => import("../pages/RailSimulation.vue"),
    },
    {
        path: "/bs",
        name: "BinarySearch",
        component: () => import("../pages/BinarySearch.vue"),
    },
];

const router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes,
});

export default router;
