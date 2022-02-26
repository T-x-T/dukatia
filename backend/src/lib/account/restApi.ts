import { IParsedReq } from "../../restApi/index.js";
import router, { IExecutorResponse } from "../../restApi/router.js";
import account, { IShallowAccount } from "./index.js";

export default () => {
	routes.forEach(x => x());
}

const routes = [
	() => {
		router.register({
			authorized: true,
			validatorFn: (req: IParsedReq) => req.path == "/accounts/all" && req.method == "GET",
			executorFn: async (req: IParsedReq): Promise<IExecutorResponse> => {
				return {
					body: await account.getAll(),
					status: 200
				}
			}
		})
	},
	() => {
		router.register({
			authorized: true,
			validatorFn: (req: IParsedReq) => req.path == "/accounts" && req.method == "POST",
			executorFn: async (req: IParsedReq): Promise<IExecutorResponse> => {
				const newAccount: IShallowAccount = {
					name: req.body.name,
					defaultCurrency: req.body.defaultCurrency,
					userId: req.userId,
					tagIds: Array.isArray(req.body.tagIds) && req.body.tagIds[0] !== null ? req.body.tagIds : undefined
				}
				
				try {
					const res = await account.add(newAccount);
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
	},

	() => {
		router.register({
			authorized: true,
			validatorFn: (req: IParsedReq) => req.path.startsWith("/accounts/") && req.method == "PUT",
			executorFn: async (req: IParsedReq): Promise<IExecutorResponse> => {
				const pathParts = req.path.split("/");
				const id = parseInt(pathParts[pathParts.length - 1]);

				if(typeof id != "number") {
					return {
						status: 400,
						body: {error: "No valid id in url path found"}
					}
				}

				const newAccount: IShallowAccount = {
					id: id,
					name: req.body.name,
					defaultCurrency: req.body.defaultCurrency,
					userId: req.userId,
					tagIds: Array.isArray(req.body.tagIds) && req.body.tagIds[0] !== null ? req.body.tagIds : undefined
				}

				try {
					const res = await account.update(newAccount);

					return {
						status: 200,
						body: res
					}
				} catch(e) {
					return {
						status: 500,
						body: {error: e.message}
					}
				}
			}
		})
	}
];