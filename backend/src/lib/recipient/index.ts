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
		return turnRowIntoIRecipient(res);
	},

	async update(recipient: IRecipient) {
		if(!Number.isInteger(recipient.id)) throw new Error("no valid id specified");
		const res = await database.update(recipient);
		if(res.length === 0) throw new Error("no row with id: " + recipient.id);
		return turnRowIntoIRecipient(res[0]);
	}
}

function turnRowIntoIRecipient(row: any): IRecipient {
	return {
		id: row.id,
		name: row.name
	}
}