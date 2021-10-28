import { Pool } from "pg";
import { IUser } from "./index.js";

let pool: Pool = null;

export default {
	init(_pool: Pool) {
		pool = _pool;
	},

	async add(user: IUser, encryptedSecret: string) {
		const db = await pool.connect();
		const res = (await db.query("INSERT INTO public.\"Users\" (id, name, secret, permissions, superuser) VALUES (DEFAULT, $1, $2, $3, $4) RETURNING *;", [user.name, encryptedSecret, user.permissions, user.superuser])).rows[0];
		db.release();
		return res;
	},

	async userCount() {
		const db = await pool.connect();
		const res = ((await db.query("SELECT * FROM public.\"Users\";")).rowCount);
		db.release();
		return res;
	},

	async hasUserSecret(user: IUser, encryptedSecret: string) {
		const db = await pool.connect();
		const res = (await db.query("SELECT id FROM public.\"Users\" WHERE name=$1 AND secret=$2;", [user.name, encryptedSecret])).rows[0]?.id;
		db.release();
		return typeof res === "number" ? res : null;
	}
}