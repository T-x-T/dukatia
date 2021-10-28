import { IParsedReq } from "../../restApi/index.js";
import router, { IExecutorResponse } from "../../restApi/router.js";
import account, { IAccount, IShallowAccount } from "./index.js";

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
					defaultCurrency: req.body.defaultCurrency
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
	}
];