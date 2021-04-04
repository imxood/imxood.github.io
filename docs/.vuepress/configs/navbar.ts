import type { NavbarConfig } from "@vuepress/theme-default";

export const navbar: NavbarConfig = [
    {
        text: "嵌入式",
        children: [
            "/embedded/",
        ],
    },
    {
        text: "后端",
        children: [
            "/backend/rust/",
        ],
    },
    {
        text: "前端",
        children: [
            "/frontend/web.md",
        ],
    },
];
