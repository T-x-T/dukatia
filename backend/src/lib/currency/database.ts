import { Pool } from "pg";
import { ICurrency } from "./index.js";

let pool: Pool = null;

export default {
	init(_pool: Pool) {
		pool = _pool;
	},

	async getAll() {
		const db = await pool.connect();
		const res = (await db.query("SELECT * FROM public.\"Currencies\"")).rows;
		db.release();
		return res;
	},

	async add(currency: ICurrency) {
		const db = await pool.connect();
		const res = (await db.query("INSERT INTO public.\"Currencies\" (id, name, minorinmayor, symbol) VALUES (DEFAULT, $1, $2, $3) RETURNING *;", [currency.name, currency.minorInMayor, currency.symbol])).rows[0];
		db.release();
		return res;
	}
}