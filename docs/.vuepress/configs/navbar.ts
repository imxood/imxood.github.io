import type { NavbarConfig } from "@vuepress/theme-default";

export const navbar: NavbarConfig = [
    {
        text: "嵌入式",
        children: [
            "/embedded/",
        ],
    },
    {
        text: "硬件",
        children: [
            "/hardware/",
        ],
    },
    {
        text: "系统",
        children: [
            "/system/rust/",
        ],
    },
    {
        text: "数学",
        children: [
            "/math/",
        ],
    },
    {
        text: "算法",
        children: [
            "/algorithm/",
        ],
    },
    {
        text: "前端",
        children: [
            "/frontend/web.md",
        ],
    },
    {
        text: "工具",
        children: [
            "/tools/",
        ],
    },
];
