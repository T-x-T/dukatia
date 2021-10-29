import { Pool } from "pg";
import { IShallowAccount } from "./index.js";

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

	async deleteById(id: number) {
		const db = await pool.connect();
		const res = (await db.query("DELETE FROM public.\"Accounts\" WHERE id=$1;", [id])).rowCount;
		db.release();
		return res;
	},

	async deleteAll() {
		const db = await pool.connect();
		const res = (await db.query("DELETE FROM public.\"Accounts\";")).rowCount;
		db.release();
		return res;
	},

	async update(account: IShallowAccount) {
		const db = await pool.connect();
		const res = (await db.query("UPDATE public.\"Accounts\" SET name=$1, defaultcurrency=$2 WHERE id=$3 RETURNING *;", [account.name, account.defaultCurrency, account.id])).rows;
		db.release();
		return res;
	},
}