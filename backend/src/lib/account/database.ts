import { Pool } from "pg";
import { IShallowAccount } from "./index.js";

let pool: Pool = null;

export default {
	init(_pool: Pool) {
		pool = _pool;
	},

	async add(account: IShallowAccount) {
		const db = await pool.connect();
		const res = (await db.query("INSERT INTO public.\"Accounts\" (id, name, defaultcurrency, \"user\") VALUES (DEFAULT, $1, $2, $3) RETURNING *;", [account.name, account.defaultCurrency, account.userId])).rows[0];
		if(account.tagIds) {
			await Promise.all(account.tagIds.map(tagId => db.query("INSERT INTO public.\"AccountTags\" (account, tag) VALUES ($1, $2);", [res.id, tagId])));
		}
		db.release();
		return res;
	},

	async getById(id: number) {
		const db = await pool.connect();
		const res = (await db.query("SELECT a.id, a.name, a.defaultcurrency, a.user, array_agg(t.tag) as tags FROM public.\"Accounts\" a LEFT JOIN public.\"AccountTags\" t ON a.id = t.account WHERE a.id = $1 GROUP BY a.id;", [id])).rows[0];
		db.release();
		return res;
	},

	async getAll() {
		const db = await pool.connect();
		const res = (await db.query("SELECT a.id, a.name, a.defaultcurrency, a.user, array_agg(t.tag) as tags FROM public.\"Accounts\" a LEFT JOIN public.\"AccountTags\" t ON a.id = t.account GROUP BY a.id;")).rows;
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
		await db.query("DELETE FROM public.\"AccountTags\" WHERE account=$1;", [account.id]);
		if(account.tagIds) {
			await Promise.all(account.tagIds.map(tagId => db.query("INSERT INTO public.\"AccountTags\" (account, tag) VALUES ($1, $2);", [account.id, tagId])));
		}
		db.release();
		return res;
	},
}