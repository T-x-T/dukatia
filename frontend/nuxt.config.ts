export default defineNuxtConfig ({
  ssr: false,
  devServer: {
    host: "0.0.0.0",
    port: process.env.HTTP_PORT ? parseInt(process.env.HTTP_PORT) : 3000
  },
  modules: [
    "@nuxtjs/color-mode"
  ],
  css: ["assets/general.sass", "assets/_vars.sass"],
  telemetry: false,
  nitro: {
    routeRules: {
      "/api/**": { proxy: `${process.env.API_HOST ? process.env.API_HOST : "http://127.0.0.1:4000"}/api/**` },
    }
  }
})