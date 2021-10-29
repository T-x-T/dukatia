import { IParsedReq } from "../../restApi/index.js";
import router, { IExecutorResponse } from "../../restApi/router.js";
import transaction, { IShallowTransaction } from "./index.js";

export default () => {
	routes.forEach(x => x());
}

const routes = [
	() => {
		router.register({
			authorized: true,
			validatorFn: (req: IParsedReq) => req.path == "/transactions/all" && req.method == "GET",
			executorFn: async (req: IParsedReq): Promise<IExecutorResponse> => {
				return {
					body: await transaction.getAll(),
					status: 200
				}
			}
		})
	},
	() => {
		router.register({
			authorized: true,
			validatorFn: (req: IParsedReq) => req.path == "/transactions" && req.method == "POST",
			executorFn: async (req: IParsedReq): Promise<IExecutorResponse> => {
				const newTransaction: IShallowTransaction = {
					userId: req.userId,
					accountId: req.body.accountId,
					currencyId: req.body.currencyId,
					recipientId: req.body.recipientId,
					status: req.body.status,
					timestamp: req.body.timestamp ? req.body.timestamp : new Date(),
					amount: Number(req.body.amount),
					comment: req.body.comment
				}
				
				try {
					const res = await transaction.add(newTransaction);
					return {
						status: 201,
						body: res
					}
				} catch(e) {
					console.error(e)
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
			validatorFn: (req: IParsedReq) => req.path.startsWith("/transactions/") && req.method == "DELETE",
			executorFn: async (req: IParsedReq): Promise<IExecutorResponse> => {
				const pathParts = req.path.split("/");
				const id = parseInt(pathParts[pathParts.length - 1]);

				if(typeof id != "number") {
					return {
						status: 400,
						body: {error: "No valid id in url path found"}
					}
				}

				const res = await transaction.deleteById(id);

				return {
					status: 200,
					body: {deleted: res}
				}
			}
		})
	},

	() => {
		router.register({
			authorized: true,
			validatorFn: (req: IParsedReq) => req.path.startsWith("/transactions/") && req.method == "PUT",
			executorFn: async (req: IParsedReq): Promise<IExecutorResponse> => {
				const pathParts = req.path.split("/");
				const id = parseInt(pathParts[pathParts.length - 1]);

				if(typeof id != "number") {
					return {
						status: 400,
						body: {error: "No valid id in url path found"}
					}
				}

				const newTransaction: IShallowTransaction = {
					id: id,
					accountId: req.body.accountId,
					currencyId: req.body.currencyId,
					recipientId: req.body.recipientId,
					status: req.body.status,
					timestamp: req.body.timestamp ? req.body.timestamp : new Date(),
					amount: Number(req.body.amount),
					comment: req.body.comment
				}

				try {
					const res = await transaction.update(newTransaction);

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