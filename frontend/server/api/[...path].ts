export default defineEventHandler((event) => {
	return proxyRequest(
			event,
			`${process.env.API_HOST ? process.env.API_HOST : "http://127.0.0.1:4000"}/api/${event.context.params!.path}`
	)
});