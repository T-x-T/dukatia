import { IParsedReq } from ".";
import accessToken from "../lib/accessToken/index.js";

let routes: {authorized?: boolean, validatorFn: Function, executorFn: Function}[] = [];

type IExecutorResponse = {
	status: number,
	body?: any
}

type IRouteData = {
	authorized?: boolean,
	validatorFn: Function,
	executorFn: Function
}

export type { IExecutorResponse }

export default {
	async route(req: IParsedReq): Promise<IExecutorResponse> {
		const matchedRoutes = routes.filter(x => x.validatorFn(req));
		if(matchedRoutes.length === 0) return {status: 404};
		if(matchedRoutes.length === 1) {
			if(matchedRoutes[0].authorized) {
				req.userId = await turnAccessTokenIntoUserId(req.cookies.get("accessToken"));
				if(typeof req.userId != "number") return {
					status: 401,
					body: {error: "No valid accessToken cookie found"}
				}
			}
			return await matchedRoutes[0].executorFn(req);
		} else {
			console.error("Multiple routes for request found", req, matchedRoutes.map(route => route.validatorFn.toString()));
      throw new Error("Multiple routes for request found");
		}
	},
	register (routeData: IRouteData) {
		routes.push(routeData);
	}
}

async function turnAccessTokenIntoUserId(token: string) {
	return await accessToken.getUserOfToken(token);
}