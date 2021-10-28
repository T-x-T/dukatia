import { Pool } from "pg";
import sha512 from "crypto-js/sha512.js";
import { IUser } from "../user";
import database from "./database.js";

type IAccessToken = {
	id: number,
	user: IUser,
	token: string
}

export type { IAccessToken };

export default {
	async init(db: Pool) {
		database.init(db);
	},

	async add(user: IUser) {
		const token = sha512(user.name + user.secret + (Date.now() * Math.random())).toString();
		return await database.add(user.id, token);
	},

	async getUserOfToken(token: string) {
		return await database.getUserOfToken(token);
	}
}