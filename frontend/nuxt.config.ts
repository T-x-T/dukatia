export default defineNuxtConfig ({
  devServer: {
    host: "0.0.0.0",
    port: process.env.HTTP_PORT ? parseInt(process.env.HTTP_PORT) : 3000
  },
  modules: [
    "@nuxtjs/color-mode"
  ],
  css: ["assets/general.sass", "assets/_vars.sass"],
  telemetry: false,

  //meta: {
  //  //titleTemplate: "%s | TxTs Treasury",
  //  //link: [
  //  //  {rel: "icon", type: "image/png", href: "/logo-icon.svg"}
  //  //],
  //  meta: [
  //    {name: "viewport", content:"width=device-width, initial-scale=1.0"}
  //  ]
  //},
  //axios: {
  //  proxy: true,
  //  baseURL: process.env.API_HOST ? process.env.API_HOST : "http://127.0.0.1:4000",
  //  credentials: true,
  //  headers: {
  //    "Content-Type": "application/json"
  //  }
  //},
/*   vite: {
    server: {
      proxy: {
        "/api": process.env.API_HOST ? process.env.API_HOST : "http://127.0.0.1:4000"
      }
    }
  }, */
  nitro: {
    routeRules: {
      "/api/**": { proxy: process.env.API_HOST ? process.env.API_HOST : "http://127.0.0.1:4000/api/**" },
    }
  }
})