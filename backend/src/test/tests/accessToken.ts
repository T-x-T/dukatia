import assert from "assert";
import { PoolClient, Pool } from "pg";
import database from "../../database/postgresql/index.js";
import user from "../../lib/user/index.js";
import accessToken from "./../../lib/accessToken/index.js";

let db: PoolClient = null;
let pool: Pool = null;
let config: any = null;

let adminUser: any = null;

export default (_config: any) => {
	config = _config;
	adminUser = {
		id: 0, 
		name: config.admin_username, 
		secret: config.admin_password
	};
}

describe("accessToken", function() {

	this.beforeAll("clear Database", async function() {
		const poolConfig = {...config.database};
		poolConfig.database = "postgres";
		const con = await database.getPostgresqlConnection(poolConfig);
		await con.query("DROP DATABASE IF EXISTS txts_treasury_testing;");
		pool = await database.getPostgresqlConnection(config.database);
		db = await pool.connect();
		await user.init(pool, config);
		await accessToken.init(pool);
	});

	this.afterAll("close database connection", async function() {
		db.release();	
		await pool.end();
	});

	beforeEach("clear Accounts table", async function() {
		await db.query("DELETE FROM public.\"Accounts\"; ALTER SEQUENCE public.\"Accounts_id_seq\" RESTART WITH 0");
	});


	describe("add", function() {
		it("doesnt reject", async function() {
			await assert.doesNotReject(() => accessToken.add(adminUser));
		});

		it("returns an access token", async function() {
			const res = await accessToken.add(adminUser);
			assert.strictEqual(typeof res, "string");
			assert.strictEqual(res.length, 128);
		});
	});

	describe("getUserOfToken", function() {
		it("returns id of only user", async function() {
			const token = await accessToken.add(adminUser);
			const res = await accessToken.getUserOfToken(token);
			assert.strictEqual(res, 0);
		});

		it("returns null on non existent token", async function() {
			const res = await accessToken.getUserOfToken("eiugheiurghe");
			assert.strictEqual(res, null);
		});

		it("returns id of only user with multiple tokens", async function() {
			await accessToken.add(adminUser);
			const token = await accessToken.add(adminUser);
			const res = await accessToken.getUserOfToken(token);
			assert.strictEqual(res, 0);
		});
	});

});