// https://v3.nuxtjs.org/api/configuration/nuxt.config
export default defineNuxtConfig({
  typescript: {
    shim: false
  },

  modules: [
    "@nuxtjs/color-mode",
    "@nuxt/content"
  ],
  css: ["assets/general.sass", "assets/_vars.sass"],

  content: {
    base: "/docs",
    highlight: {
      theme: "github-dark"
    }
  }
})
