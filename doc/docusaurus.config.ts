import { themes as prismThemes } from "prism-react-renderer";
import type { Config } from "@docusaurus/types";
import type * as Preset from "@docusaurus/preset-classic";

// This runs in Node.js - Don't use client-side code here (browser APIs, JSX...)

const config: Config = {
    title: "FocusFlow – Open Source Pomodoro Tracker",
    tagline:
        "A comprehensive Pomodoro technique tracking solution featuring a Rust backend and a Flutter mobile application.",
    favicon: "img/app_icon.png",

    // Future flags, see https://docusaurus.io/docs/api/docusaurus-config#future
    future: {
        v4: true, // Improve compatibility with the upcoming Docusaurus v4
    },

    // Set the production url of your site here
    // Set the production url of your site here
    url: "https://francesco-gaglione.github.io",
    // Set the /<baseUrl>/ pathname under which your site is served
    // For GitHub pages deployment, it is often '/<projectName>/'
    baseUrl: "/focus_flow_cloud/",

    // GitHub pages deployment config.
    // If you aren't using GitHub pages, you don't need these.
    organizationName: "francesco-gaglione", // Usually your GitHub org/user name.
    projectName: "focus_flow_cloud", // Usually your repo name.

    onBrokenLinks: "throw",

    // Even if you don't use internationalization, you can use this field to set
    // useful metadata like html lang. For example, if your site is Chinese, you
    // may want to replace "en" with "zh-Hans".
    i18n: {
        defaultLocale: "en",
        locales: ["en", "it"],
    },

    headTags: [
        {
            tagName: "script",
            attributes: { type: "application/ld+json" },
            innerHTML: JSON.stringify({
                "@context": "https://schema.org",
                "@type": "SoftwareApplication",
                name: "FocusFlow",
                applicationCategory: "ProductivityApplication",
                operatingSystem: "iOS, Android, macOS, Windows, Linux",
                description:
                    "Open-source Pomodoro technique tracker with real-time sync, task management, and productivity analytics.",
                url: "https://francesco-gaglione.github.io/focus_flow_cloud/",
                author: {
                    "@type": "Person",
                    name: "Francesco Gaglione",
                },
                license: "https://opensource.org/licenses/MIT",
                offers: { "@type": "Offer", price: "0", priceCurrency: "USD" },
            }),
        },
    ],

    presets: [
        [
            "classic",
            {
                docs: {
                    sidebarPath: "./sidebars.ts",
                },
                blog: {
                    showReadingTime: false,
                    feedOptions: {
                        type: ["rss", "atom"],
                        xslt: true,
                    },
                    onInlineTags: "warn",
                    onInlineAuthors: "warn",
                    onUntruncatedBlogPosts: "warn",
                },
                sitemap: {
                    lastmod: "date",
                    changefreq: "weekly",
                    priority: 0.5,
                },
                theme: {
                    customCss: "./src/css/custom.css",
                },
            } satisfies Preset.Options,
        ],
    ],

    markdown: {
        mermaid: true,
    },
    themes: ['@docusaurus/theme-mermaid'],

    themeConfig: {
        image: "img/app_icon.png",
        metadata: [
            { name: "keywords", content: "pomodoro, focus timer, productivity, task management, rust, flutter, open source, time tracking" },
            { name: "author", content: "Francesco Gaglione" },
            { name: "robots", content: "index, follow" },
            { property: "og:type", content: "website" },
            { name: "twitter:card", content: "summary_large_image" },
        ],
        colorMode: {
            respectPrefersColorScheme: true,
        },
        navbar: {
            title: "Focus Flow",
            logo: {
                alt: "Focus Flow Logo",
                src: "img/app_icon.png",
            },
            items: [
                {
                    type: "docSidebar",
                    sidebarId: "tutorialSidebar",
                    position: "left",
                    label: "Tutorial",
                },
                {
                    href: "https://github.com/francesco-gaglione/focus_flow_cloud",
                    label: "GitHub",
                    position: "right",
                },
            ],
        },
        footer: {
            style: "dark",
            links: [
                {
                    title: "Docs",
                    items: [
                        {
                            label: "Tutorial",
                            to: "/docs/intro",
                        },
                    ],
                },
                {
                    title: "More",
                    items: [
                        {
                            label: "GitHub",
                            href: "https://github.com/francesco-gaglione/focus_flow_cloud",
                        },
                    ],
                },
            ],
            copyright: `Copyright © ${new Date().getFullYear()} Focus Flow`,
        },
        prism: {
            theme: prismThemes.github,
            darkTheme: prismThemes.dracula,
        },
    } satisfies Preset.ThemeConfig,
};

export default config;
