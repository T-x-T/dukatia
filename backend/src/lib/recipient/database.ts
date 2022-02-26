import { Pool } from "pg";
import { IRecipient } from "./index.js";

let pool: Pool = null;

export default {
	init(_pool: Pool) {
		pool = _pool;
	},

	async getAll() {
		const db = await pool.connect(); 
		const res = (await db.query("SELECT r.id, r.name, r.user, array_agg(t.tag) as tags FROM public.\"Recipients\" r LEFT JOIN public.\"RecipientTags\" t ON r.id = t.recipient GROUP BY r.id;")).rows;
		db.release();
		return res;
	},

	async add(recipient: IRecipient) {
		const db = await pool.connect(); 
		const res = (await db.query("INSERT INTO public.\"Recipients\" (id, name, \"user\") VALUES (DEFAULT, $1, $2) RETURNING *;", [recipient.name, recipient.userId])).rows[0];
		if(recipient.tagIds) {
			await Promise.all(recipient.tagIds.map((tagId) => db.query("INSERT INTO public.\"RecipientTags\" (recipient, tag) VALUES ($1, $2);", [res.id, tagId])));
		}
		db.release();
		return res;
	},

	async update(recipient: IRecipient) {
		const db = await pool.connect();
		const res = (await db.query("UPDATE public.\"Recipients\" SET name=$1 WHERE id=$2 RETURNING *;", [recipient.name, recipient.id])).rows;
		await db.query("DELETE FROM public.\"RecipientTags\" WHERE recipient=$1", [recipient.id]);
		if(recipient.tagIds) {
			await Promise.all(recipient.tagIds.map((tagId) => db.query("INSERT INTO public.\"RecipientTags\" (recipient, tag) VALUES ($1, $2);", [recipient.id, tagId])));
		}
		db.release();
		return res;
	}
}