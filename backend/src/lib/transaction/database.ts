import { Pool } from "pg";
import { IShallowTransaction } from "./index.js";

let pool: Pool = null;

export default {
	init(_pool: Pool) {
		pool = _pool;
	},

	async add(transaction: IShallowTransaction) {
		const db = await pool.connect();
		const res = (await db.query(
			"INSERT INTO public.\"Transactions\" (id, \"user\", account, currency, recipient, status, timestamp, amount, comment) VALUES (DEFAULT, $1, $2, $3, $4, $5, $6, $7, $8) RETURNING *;",
			[transaction.userId, transaction.accountId, transaction.currencyId, transaction.recipientId, transaction.status, transaction.timestamp, transaction.amount, transaction.comment]
		)).rows[0];
		db.release();
		return res;
	},

	async getAll() {
		const db = await pool.connect();
		const res = (await db.query("SELECT * FROM public.\"Transactions\";")).rows;
		db.release();
		return res;
	},

	async getById(id: number) {
		const db = await pool.connect();
		const res = (await db.query("SELECT * FROM public.\"Transactions\" WHERE id=$1;", [id])).rows;
		db.release();
		return res;
	}
}