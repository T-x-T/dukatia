import { Pool } from "pg";
import database from "./database.js";
import restApi from "./restApi.js";

type IRecipient = {
	id?: number,
	name: string,
	userId: number,
	tagIds?: number[]
}

export type { IRecipient };

export default {
	async init(db: Pool) {
		database.init(db);
		restApi();
	},

	async getAll(): Promise<IRecipient[]> {
		return (await database.getAll()).map(x => turnRowIntoIRecipient(x));
	},

	async add(recipient: IRecipient): Promise<IRecipient> {
		return turnRowIntoIRecipient(await database.add(recipient));
	},

	async update(recipient: IRecipient): Promise<IRecipient> {
		if(!Number.isInteger(recipient.id)) throw new Error("no valid id specified");
		const res = await database.update(recipient);
		if(res.length === 0) throw new Error("no row with id: " + recipient.id);
		return turnRowIntoIRecipient(res[0]);
	}
}

function turnRowIntoIRecipient(row: any): IRecipient {
	return {
		id: row.id,
		name: row.name,
		userId: row.user,
		tagIds: Array.isArray(row.tags) && row.tags[0] !== null ? row.tags : undefined
	}
}