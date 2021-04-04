import type { SidebarConfig } from "@vuepress/theme-default";

export const sidebar: SidebarConfig = {
    "/embedded/": [
        {
            isGroup: true,
            text: "嵌入式",
            children: [
                "/embedded/C语言.md",
            ],
        },
    ],
    "/backend/rust/": [
        {
            isGroup: true,
            text: "后端",
            children: [
                "/backend/rust/",
                "/backend/rust/basic.md",
            ],
        },
    ],
    "/frontend/": [
        {
            isGroup: true,
            text: "前端",
            link: "a",
            children: [
                "/frontend/web.md",
                "/frontend/vue/vue.md",
            ],
        },
    ],
    "/tools/": [
        {
            isGroup: true,
            text: "工具",
            children: [
                "/tools/git.md",
            ],
        },
    ],
};
