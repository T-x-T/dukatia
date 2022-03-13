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
					name: req.body.name,
					userId: req.userId,
					tagIds: Array.isArray(req.body.tagIds) && req.body.tagIds[0] !== null ? req.body.tagIds : undefined
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
	},

	() => {
		router.register({
			authorized: true,
			validatorFn: (req: IParsedReq) => req.path.startsWith("/recipients/") && req.method == "PUT",
			executorFn: async (req: IParsedReq): Promise<IExecutorResponse> => {
				const pathParts = req.path.split("/");
				const id = parseInt(pathParts[pathParts.length - 1]);

				if(typeof id != "number") {
					return {
						status: 400,
						body: {error: "No valid id in url path found"}
					}
				}

				const newRecipient: IRecipient = {
					id: id,
					name: req.body.name,
					userId: req.userId,
					tagIds: Array.isArray(req.body.tagIds) && req.body.tagIds[0] !== null ? req.body.tagIds : undefined
				}

				try {
					const res = await recipient.update(newRecipient);

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