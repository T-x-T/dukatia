import { Pool } from "pg";
import { ITag } from "./index.js";

let pool: Pool = null;

export default {
	init(_pool: Pool) {
		pool = _pool;
	},

	async getAll() {
		const db = await pool.connect();
		const res = (await db.query("SELECT * FROM public.\"Tags\";")).rows;
		db.release();
		return res;
	},

	async getById(id: number) {
		const db = await pool.connect();
		const res = (await db.query("SELECT * FROM public.\"Tags\" WHERE id=$1;", [id])).rows[0];
		db.release();
		return res;
	},

	async add(tag: ITag) {
		const db = await pool.connect();
		let res;
		if(typeof tag.parentId == "number") {
			res = (await db.query("INSERT INTO public.\"Tags\" (id, name, parent) VALUES (DEFAULT, $1, $2) RETURNING *;", [tag.name, tag.parentId])).rows[0];
		} else {
			res = (await db.query("INSERT INTO public.\"Tags\" (id, name) VALUES (DEFAULT, $1) RETURNING *;", [tag.name])).rows[0];
		}
		db.release();
		return res;
	},

	async update(tag: ITag) {
		const db = await pool.connect();
		const res = (await db.query("UPDATE public.\"Tags\" SET name=$1, parent=$2 WHERE id=$3 RETURNING *;", [tag.name, typeof tag.parentId == "number" ? tag.parentId : null, tag.id])).rows;
		db.release();
		return res;
	}
}