import { Pool } from "pg";
import { IRecipient } from "./index.js";

let pool: Pool = null;

export default {
	init(_pool: Pool) {
		pool = _pool;
	},

	async getAll() {
		const db = await pool.connect(); 
		const res = (await db.query("SELECT * FROM public.\"Recipients\"")).rows;
		db.release();
		return res;
	},

	async add(recipient: IRecipient) {
		const db = await pool.connect(); 
		const res = (await db.query("INSERT INTO public.\"Recipients\" (id, name) VALUES (DEFAULT, $1) RETURNING *;", [recipient.name])).rows[0];
		db.release();
		return res;
	}
}