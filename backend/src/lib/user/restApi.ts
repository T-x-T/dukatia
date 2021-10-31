import { IParsedReq } from "../../restApi/index.js";
import router, { IExecutorResponse } from "../../restApi/router.js";
import accessToken from "../accessToken/index.js";
import user from "./index.js";

export default () => {
	routes.forEach(x => x());
}

const routes = [
	() => {
		router.register({
			validatorFn: (req: IParsedReq) => req.path == "/login" && req.method == "POST",
			executorFn: async (req: IParsedReq): Promise<IExecutorResponse> => {
				const credentials = {
					name: req.body.name,
					secret: req.body.secret
				}

				const userId = await user.login(credentials);
				
				if(typeof userId == "number") {
					return {
						status: 200,
						body: {
							accessToken: await accessToken.add({id: userId, ...credentials})
						}
					}
				} else {
					return {
						status: 400,
						body: {
							error: "The provided credentials didnt match any registered user"
						}
					}
				}
			}
		})
	}
];
