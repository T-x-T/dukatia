import { Pool } from "pg";
import database from "./database.js";
import restApi from "./restApi.js";

type ICurrency = {
	id?: number,
	name: string,
	minorInMayor: number,
	symbol: string
}

export type { ICurrency };

export default {
	async init(db: Pool) {
		database.init(db);
		restApi();
	},

	async getAll() {
		return await database.getAll();
	},

	async add(currency: ICurrency): Promise<ICurrency> {
		const res = await database.add(currency);
		return {
			id: res.id,
			name: res.name,
			minorInMayor: res.minorinmayor,
			symbol: res.symbol
		};
	}
}