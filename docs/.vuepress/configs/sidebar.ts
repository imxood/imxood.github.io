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
    "/hardware/": [
        {
            isGroup: true,
            text: "硬件",
            children: [
                "/hardware/三极管.md",
                "/hardware/MOS管.md",
                "/hardware/usb,电源,电池--供电电路.md",
            ],
        },
    ],
    "/system/rust/": [
        {
            isGroup: true,
            text: "系统",
            children: [
                "/system/rust/",
                "/system/rust/basic.md",
            ],
        },
    ],
    "/math/": [
        {
            isGroup: true,
            text: "数学",
            children: [
                "/math/积分.md",
                "/math/微分.md",
            ],
        },
    ],
    "/algorithm/": [
        {
            isGroup: true,
            text: "算法",
            children: [
                "/algorithm/README.md",
            ],
        },
    ],
    "/frontend/": [
        {
            isGroup: true,
            text: "前端",
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
