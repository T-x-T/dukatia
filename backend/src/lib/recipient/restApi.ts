import { IParsedReq } from "../../restApi/index.js";
import router, { IExecutorResponse } from "../../restApi/router.js";
import recipient, { IRecipient } from "./index.js";

export default () => {
	routes.forEach(x => x());
}

const routes = [
	() => {
		router.register({
			authorized: true,
			validatorFn: (req: IParsedReq) => req.path == "/recipients/all" && req.method == "GET",
			executorFn: async (req: IParsedReq): Promise<IExecutorResponse> => {
				return {
					body: await recipient.getAll(),
					status: 200
				}
			}
		})
	},
	() => {
		router.register({
			authorized: true,
			validatorFn: (req: IParsedReq) => req.path == "/recipients" && req.method == "POST",
			executorFn: async (req: IParsedReq): Promise<IExecutorResponse> => {
				const newRecipient: IRecipient = {
					name: req.body.name
				}

				try {
					const res = await recipient.add(newRecipient);
					return {
						status: 201,
						body: res
					}
				} catch(e) {
					console.error(e);
					return {
						status: 500,
						body: {error: e.message}
					}
				}
			}
		})
	}
];