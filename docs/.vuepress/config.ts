import { defineUserConfig } from "vuepress";
import type { DefaultThemeOptions } from "vuepress";

import { navbar } from "./configs/navbar";
import { sidebar } from "./configs/sidebar";

export default defineUserConfig<DefaultThemeOptions>({
    base: "/",
    lang: "zh-CN",
    title: "博客",

    debug: true,

    bundler:
        process.env.NODE_ENV === "production"
            ? "@vuepress/webpack"
            : "@vuepress/vite",

    themeConfig: {
        lastUpdatedText: "上次更新",
        repo: "imxood/imxood.github.io",
        editLinks: true,
        editLinkText: "编辑文档！",
        docsDir: "docs",

        navbar: navbar,
        sidebar: sidebar,

        themePlugins: {
            git: true,
        },
    },

    plugins: [],
});
