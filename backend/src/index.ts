//const envoirnment = process.env.NODE_ENV ? process.env.NODE_ENV : "staging";

const config = {
	"database": {
		"user": process.env.POSTGRES_USER ? process.env.POSTGRES_USER : "postgres",
		"password": process.env.POSTGRES_PASSWORD ? process.env.POSTGRES_PASSWORD : "password",
		"host": process.env.POSTGRES_HOST ? process.env.POSTGRES_HOST : "127.0.0.1",
		"port": process.env.POSTGRES_PORT ? parseInt(process.env.POSTGRES_PORT) : 5432,
		"database": process.env.DATABASE_NAME ? process.env.DATABASE_NAME : "txts_treasury_staging"
	},
	"admin_username": process.env.ADMIN_USERNAME ? process.env.ADMIN_USERNAME : "admin",
	"admin_password": process.env.ADMIN_PASSWORD ? process.env.ADMIN_PASSWORD : "password",
	"pepper": process.env.PEPPER ? process.env.PEPPER : "supersecret",
	"api_port": process.env.PORT ? parseInt(process.env.PORT) : 4000
}

import database from "./database/index.js";
const db = await database(config.database);
console.log((await db.query("SELECT now()")).rows[0]);

import restApi from "./restApi/index.js";
restApi(config.api_port);

import currency from "./lib/currency/index.js";
import user from "./lib/user/index.js";
import accessToken from "./lib/accessToken/index.js"; 
import account from "./lib/account/index.js";
import recipient from "./lib/recipient/index.js";
import transaction from "./lib/transaction/index.js";

await Promise.all([
	currency.init(db),
	user.init(db, config),
	accessToken.init(db),
	account.init(db),
	recipient.init(db),
	transaction.init(db)
]);