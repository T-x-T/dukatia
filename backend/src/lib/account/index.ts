import { Pool } from "pg";
import { ICurrency } from "../currency";
import { ITag } from "../tag";
import database from "./database.js";
import restApi from "./restApi.js";

type IAccount = {
	id?: number,
	name: string,
	defaultCurrency: ICurrency,
	tags?: ITag[]
}

type IShallowAccount = {
	id?: number,
	name: string,
	defaultCurrency: number
}

export type { IAccount, IShallowAccount };

export default {
	async init(pool: Pool) {
		database.init(pool);
		restApi();
	},

	async add(account: IShallowAccount) {
		return turnRowIntoIShallowAccount(await database.add(account));
	},

	async getByName(name: string) {
		return (await database.getFiltered("name", name)).map(x => turnRowIntoIShallowAccount(x));
	},

	async getAll() {
		return (await database.getAll()).map(x => turnRowIntoIShallowAccount(x));
	},

	async deleteById(id: number) {
		return await database.deleteById(id);
	},

	async deleteAll() {
		return await database.deleteAll();
	},

	async update(account: IShallowAccount) {
		if(!Number.isInteger(account.id)) throw new Error("no valid id specified");
		const res = await database.update(account);
		if(res.length === 0) throw new Error("no row with id: " + account.id);
		return turnRowIntoIShallowAccount(res[0]);
	}
}

function turnRowIntoIShallowAccount(row: any): IShallowAccount {
	return {
		id: row.id,
		name: row.name,
		defaultCurrency: row.defaultcurrency
	}
}