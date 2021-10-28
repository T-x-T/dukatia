import { Pool } from "pg";
import { ITag } from "../tag";
import database from "./database.js";
import restApi from "./restApi.js";

type IRecipient = {
	id?: number,
	name: string,
	tags?: ITag[]
}

export type { IRecipient };

export default {
	async init(db: Pool) {
		database.init(db);
		restApi();
	},

	async getAll() {
		return await database.getAll();
	},

	async add(recipient: IRecipient): Promise<IRecipient> {
		const res = await database.add(recipient);
		return {
			id: res.id,
			name: res.name
		};
	}
}