import { Pool } from "pg";

let pool: Pool = null;

export default {
	init(_pool: Pool) {
		pool = _pool;
	},

	async add(userId: number, token: string) {
		const db = await pool.connect();
		const res = (await db.query("INSERT INTO public.\"AccessTokens\" (id, \"user\", token) VALUES (DEFAULT, $1, $2) RETURNING token;", [userId, token])).rows[0].token;
		db.release();
		return res ? res : null;
	},

	async getUserOfToken(token: string) {
		const db = await pool.connect();
		const res = (await db.query("SELECT \"user\" FROM public.\"AccessTokens\" WHERE token=$1", [token])).rows[0]?.user;
		db.release();
		return typeof res == "number" ? res : null;
	}
}