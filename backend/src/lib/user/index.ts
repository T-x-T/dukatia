import { Pool } from "pg";
import database from "./database.js";
import restApi from "./restApi.js";
import sha512 from "crypto-js/sha512.js";

type IUser = {
	id?: number,
	name: string,
	secret: string,
	permissions?: any,
	superuser?: boolean
}

export type { IUser };

let config: any = null;

export default {
	async init(db: Pool, _config: any) {
		config = _config;
		database.init(db);
		restApi();

		if (await database.userCount() === 0) {
			const adminUser: IUser = {
				name: config.admin_username,
				secret: config.admin_password,
				permissions: {},
				superuser: true
			}
			await database.add(adminUser, sha512(`${adminUser.name}${adminUser.secret}${config.pepper}`).toString());
			return;
		}
	},

	async login(user: IUser): Promise<number> {
		return await database.hasUserSecret(user, sha512(`${user.name}${user.secret}${config.pepper}`).toString());
	}
}