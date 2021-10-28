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
		return turnRowIntoIAccount(await database.add(account));
	},

	async getByName(name: string) {
		return (await database.getFiltered("name", name)).map(x => turnRowIntoIAccount(x));
	},

	async getAll() {
		return (await database.getAll()).map(x => turnRowIntoIAccount(x));
	},

	async deleteByName(name: string) {
		return await database.deleteFiltered("name", name);
	},

	async deleteById(id: number) {
		return await database.deleteFiltered("id", id);
	},

	async deleteAll() {
		return await database.deleteAll();
	}
}

function turnRowIntoIAccount(row: any): IAccount {
	return {
		id: row.id,
		name: row.name,
		defaultCurrency: row.defaultcurrency
	}
}