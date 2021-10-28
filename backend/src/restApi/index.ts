import http, { IncomingMessage, ServerResponse } from "http";
import { StringDecoder } from "string_decoder";
import { URL } from "url";
import router, { IExecutorResponse } from "./router.js";

type IParsedReq = {
	path: string,
	method: string,
	query: URLSearchParams,
	body: any,
	cookies: Map<string, string>,
	userId?: number
}

export type { IParsedReq }

export default function(port: number) {
	http.createServer(listener).listen(port, () => console.log("REST API Server started on port " + port));
}

async function listener(req: IncomingMessage, res: ServerResponse) {
	const reqData = await getRequestData(req);

	res.setHeader("Content-Type", "application/json");

	if(reqData.path.startsWith("/api/v1")) {
		reqData.path = reqData.path.replace("/api/v1", "");

		try {
			console.log(reqData)
			const resData: IExecutorResponse = await router.route(reqData);
			if(resData) {
				res.writeHead(resData.status);
				res.end(JSON.stringify(resData.body));
      } else {
        res.writeHead(404);
        res.end("{\"error\": \"API resource not found\"}");
			}
		} catch(e) {
      res.writeHead(500);
      res.end(JSON.stringify({error: e.message}));
      console.error(e);
		}

	} else {
		res.writeHead(404);
		res.end("{\"error\": \"API resource not found\"}");
	}
}

async function getRequestData(req: IncomingMessage) {
	const parsedUrl = new URL("http://dummy" + req.url);
	
	let cookies: Map<string, string> = new Map<string, string>();
	if(req.headers.cookie) req.headers.cookie.replace(/\s/g, "").split(";").forEach(cookie => {
		const parts = cookie.split("=");
		cookies.set(parts[0], parts[1]);
	});

	return {
		path: parsedUrl.pathname,
		method: req.method,
		query: parsedUrl.searchParams,
		body: await getPayload(req),
		cookies: cookies
	}
}

function getPayload(req: IncomingMessage) {
	return new Promise((resolve, reject) => {
		//Try to get payload, if there is some
    const decoder = new StringDecoder("utf-8");
    let buffer = "";
    req.on("data", function (data) {
      buffer += decoder.write(data);
    });
    req.on("end", function () {
      buffer += decoder.end();

      let jsonObject;
      try {
        jsonObject = JSON.parse(buffer);
      } catch(e) {
        jsonObject = {};
      }
			resolve(jsonObject);
		});
	});
}