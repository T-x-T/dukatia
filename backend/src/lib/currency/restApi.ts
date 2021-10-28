import { IParsedReq } from "../../restApi/index.js";
import router, { IExecutorResponse } from "../../restApi/router.js";
import currency from "./index.js";

export default () => {
	routes.forEach(x => x());
}

const routes = [
	() => {
		router.register({
			authorized: true,
			validatorFn: (req: IParsedReq) => req.path == "/currencies/all" && req.method == "GET",
			executorFn: async (req: IParsedReq): Promise<IExecutorResponse> => {
				return {
					body: await currency.getAll(),
					status: 200
				}
			}
		})
	}
];