// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
    devtools: { enabled: false },
    ssr: false,
    modules: ['@nuxtjs/tailwindcss', '@nuxtjs/i18n'],
    i18n: {
        vueI18n: './i18n/i18n.config.ts', // if you are using custom path, default
        locales: [
            { code: 'en', file: 'en.json' },
            { code: 'it', file: 'it.json' },
            { code: 'zh-CN', file: 'zh-CN.json' }
        ]
    }
})