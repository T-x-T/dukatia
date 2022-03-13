import { IParsedReq } from "../../restApi/index.js";
import router, { IExecutorResponse } from "../../restApi/router.js";
import tag, { ITag } from "./index.js";

export default () => {
	routes.forEach(x => x());
}

const routes = [
	() => {
		router.register({
			authorized: true,
			validatorFn: (req: IParsedReq) => req.path == "/tags/all" && req.method == "GET",
			executorFn: async (req: IParsedReq): Promise<IExecutorResponse> => {
				return {
					body: await tag.getAll(),
					status: 200
				}
			}
		})
	},
	() => {
		router.register({
			authorized: true,
			validatorFn: (req: IParsedReq) => req.path == "/tags" && req.method == "POST",
			executorFn: async (req: IParsedReq): Promise<IExecutorResponse> => {
				const newTag: ITag = {
					name: req.body.name,
					userId: req.userId,
					parentId: req.body.parentId ? req.body.parentId : undefined
				}

				try {
					const res = await tag.add(newTag);
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
			validatorFn: (req: IParsedReq) => req.path.startsWith("/tags/") && req.method == "PUT",
			executorFn: async (req: IParsedReq): Promise<IExecutorResponse> => {
				const pathParts = req.path.split("/");
				const id = parseInt(pathParts[pathParts.length - 1]);
				
				if(typeof id != "number") {
					return {
						status: 400,
						body: {error: "No valid id in url path found"}
					}
				}
				
				const newTag: ITag = {
					id: id,
					name: req.body.name,
					userId: req.userId,
					parentId: req.body.parentId ? req.body.parentId : undefined
				}
				
				try {
					const res = await tag.update(newTag);
					
					return {
						status: 200,
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