// https://v3.nuxtjs.org/api/configuration/nuxt.config
export default defineNuxtConfig({
  ssr: true,

  routeRules: {
    '/': { prerender: true },
  },

  typescript: {
    shim: false
  },

  nitro: {
    prerender: {
      autoSubfolderIndex: false,
      crawlLinks: true
    }
  },

  modules: [
    "@nuxtjs/color-mode",
    "@nuxt/content"
  ],
  css: ["assets/general.sass", "assets/_vars.sass"],

  content: {
    api: {
      baseURL: "/docs",
    },
    highlight: {
      theme: "github-dark"
    }
  }
})
