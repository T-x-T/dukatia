//const envoirnment = process.env.NODE_ENV ? process.env.NODE_ENV : "staging";

const config = {
	"database": {
		"user": "postgres",
		"password": "password",
		"host": "127.0.0.1",
		"port": 5432,
		"database": "txts_treasury_staging"
	},
	"admin_username": "admin",
	"admin_password": "password",
	"pepper": "supersecret",
	"api_port": 4000
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