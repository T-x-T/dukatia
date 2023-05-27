export default defineNuxtConfig ({
  ssr: false,
  devServer: {
    host: "0.0.0.0",
    port: 3000
  },
  modules: [
    "@nuxtjs/color-mode"
  ],
  css: ["assets/general.sass", "assets/_vars.sass"],
  telemetry: false
})