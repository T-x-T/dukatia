export default defineNuxtConfig ({
  ssr: false,
  devServer: {
    host: "0.0.0.0",
    port: 3000
  },
  modules: [
    "@nuxtjs/color-mode"
  ],
  colorMode: {
    preference: "dark",
    fallback: "dark",
  },
  css: ["assets/general.sass", "assets/_vars.sass"],
  telemetry: false,
  app: {
    head: {
      charset: 'utf-8',
      viewport: 'width=device-width, initial-scale=1',
      link: [
        {rel: "manifest", href: "/manifest.json"},
        {rel: "icon", type: "image/svg+xml", href: "/dukatia_signet.svg"},
      ]
    }
  }
})