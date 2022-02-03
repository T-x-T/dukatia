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
		if(transaction.tagIds) {
			await Promise.all(transaction.tagIds.map((tagId) => db.query("INSERT INTO public.\"TransactionTags\" (transaction, tag) VALUES ($1, $2);", [res.id, tagId])));
		}
		db.release();
		return res;
	},

	async getAll() {
		const db = await pool.connect();
		const res = (await db.query("SELECT tr.id, tr.account, tr.currency, tr.recipient, tr.status, tr.user, tr.timestamp, tr.amount, tr.comment, array_agg(t.tag) as tags FROM public.\"Transactions\" tr LEFT JOIN public.\"TransactionTags\" t ON tr.id = t.transaction GROUP BY tr.id;")).rows;
		db.release();
		return res;
	},

	async getById(id: number) {
		const db = await pool.connect();
		const res = (await db.query("SELECT tr.id, tr.account, tr.currency, tr.recipient, tr.status, tr.user, tr.timestamp, tr.amount, tr.comment, array_agg(t.tag) as tags FROM public.\"Transactions\" tr LEFT JOIN public.\"TransactionTags\" t ON tr.id = t.transaction WHERE tr.id=$1 GROUP BY tr.id;", [id])).rows[0];
		db.release();
		return res;
	},

	async deleteById(id: number) {
		const db = await pool.connect();
		const res = (await db.query("DELETE FROM public.\"Transactions\" WHERE id=$1;", [id])).rowCount;
		db.release();
		return res;
	},

	async update(transaction: IShallowTransaction) {
		const db = await pool.connect();
		const res = (await db.query("UPDATE public.\"Transactions\" SET account=$1, currency=$2, recipient=$3, status=$4, timestamp=$5, amount=$6, comment=$7 WHERE id=$8 RETURNING *;", [transaction.accountId, transaction.currencyId, transaction.recipientId, transaction.status, transaction.timestamp, transaction.amount, transaction.comment, transaction.id])).rows;
		await db.query("DELETE FROM public.\"TransactionTags\" WHERE transaction=$1", [transaction.id]);
		if(transaction.tagIds) {
			await Promise.all(transaction.tagIds.map((tagId) => db.query("INSERT INTO public.\"TransactionTags\" (transaction, tag) VALUES ($1, $2);", [transaction.id, tagId])));
		}
		db.release();
		return res;
	}
}