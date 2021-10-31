export default {
  server: {
    host: "0.0.0.0",
    port: process.env.HTTP_PORT ? parseInt(process.env.HTTP_PORT) : 3000
  },
  modules: [
    "@nuxtjs/axios"
  ],
  css: ["assets/general", "assets/_vars"],
  components: true,
  head: {
    //titleTemplate: "%s | TxTs Treasury",
    //link: [
    //  {rel: "icon", type: "image/png", href: "/logo-icon.svg"}
    //],
    meta: [
      {name: "viewport", content:"width=device-width, initial-scale=1.0"}
    ]
  },
  axios: {
    proxy: true,
    baseURL: process.env.API_HOST ? process.env.API_HOST : "http://localhost:4000",
    credentials: true
  },
  proxy: {
    "/api": process.env.API_HOST ? process.env.API_HOST : "http://localhost:4000"
	}
}