import { Pool } from "pg";
import { IAccount, IShallowAccount } from "./index.js";

let pool: Pool = null;

export default {
	init(_pool: Pool) {
		pool = _pool;
	},

	async add(account: IShallowAccount) {
		const db = await pool.connect();
		const res = (await db.query("INSERT INTO public.\"Accounts\" (id, name, defaultcurrency) VALUES (DEFAULT, $1, $2) RETURNING *;", [account.name, account.defaultCurrency])).rows[0];
		db.release();
		return res;
	},

	async getFiltered(key: string, value: string) {
		const db = await pool.connect();
		//TODO: Fix possible SQL injection
		const res = (await db.query("SELECT * FROM public.\"Accounts\" WHERE " + key + " = $1;", [value])).rows;
		db.release();
		return res;
	},

	async getAll() {
		const db = await pool.connect();
		const res = (await db.query("SELECT * FROM public.\"Accounts\";")).rows;
		db.release();
		return res;
	},

	async deleteFiltered(key: string, value: string | number) {
		const db = await pool.connect();
		//TODO: Fix possible SQL injection
		const res = (await db.query("DELETE FROM public.\"Accounts\" WHERE " + key + " = $1;", [value])).rowCount;
		db.release();
		return res;
	},

	async deleteAll() {
		const db = await pool.connect();
		const res = (await db.query("DELETE FROM public.\"Accounts\";")).rowCount;
		db.release();
		return res;
	}
}