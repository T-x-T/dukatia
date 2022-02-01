//To throw on unhadledRejections
process.on("unhandledRejection", (reason, promise) => { 
  console.log(reason);
  throw promise 
});

const config = {
	"database": {
		"user": "postgres",
		"password": process.env.POSTGRES_PASSWORD ? process.env.POSTGRES_PASSWORD : "password",
		"host": process.env.POSTGRES_HOST ? process.env.POSTGRES_HOST : "127.0.0.1",
		"port": 5432,
		"database": "txts_treasury_testing"
	},
	"admin_username": "admin",
	"admin_password": "password",
	"pepper": "supersecret",
	"api_port": 4000
}

import testUser from "./tests/user.js";
testUser(config);
import testAccessToken from "./tests/accessToken.js";
testAccessToken(config);
import testAccount from "./tests/account.js";
testAccount(config);
import testCurrency from "./tests/currency.js";
testCurrency(config);
import testRecipient from "./tests/recipient.js";
testRecipient(config);
import testTag from "./tests/tag.js";
testTag(config);
import testTransaction from "./tests/transaction.js";
testTransaction(config);
