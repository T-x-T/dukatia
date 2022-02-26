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
	defaultCurrency: number,
	userId: number,
	tagIds?: number[]
}

export type { IAccount, IShallowAccount };

export default {
	async init(pool: Pool) {
		database.init(pool);
		restApi();
	},

	async add(account: IShallowAccount): Promise<IShallowAccount> {
		return turnRowIntoIShallowAccount(await database.add(account));
	},

	async getById(id: number): Promise<IShallowAccount> {
		return turnRowIntoIShallowAccount(await database.getById(id));
	},

	async getAll(): Promise<IShallowAccount[]> {
		return (await database.getAll()).map(x => turnRowIntoIShallowAccount(x));
	},

	async deleteById(id: number): Promise<number> {
		return await database.deleteById(id);
	},

	async deleteAll(): Promise<number> {
		return await database.deleteAll();
	},

	async update(account: IShallowAccount): Promise<IShallowAccount> {
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
		defaultCurrency: row.defaultcurrency,
		userId: row.user,
		tagIds: Array.isArray(row.tags) && row.tags[0] !== null ? row.tags : undefined
	}
}