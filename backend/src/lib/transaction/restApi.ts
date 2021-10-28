import { IParsedReq } from "../../restApi/index.js";
import router, { IExecutorResponse } from "../../restApi/router.js";
import transaction, { ETransactionStatus, IShallowTransaction } from "./index.js";

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
					status: 1,
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
	}
];