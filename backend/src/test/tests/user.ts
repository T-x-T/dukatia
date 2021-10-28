import assert from "assert"
import { PoolClient, Pool } from "pg";
import database from "../../database/postgresql/index.js";
import user from "../../lib/user/index.js";

let db: PoolClient = null;
let pool: Pool = null;
let config: any = null;

export default (_config: any) => config = _config;

describe("user", function() {

	this.beforeAll("clear Database", async function() {
		const poolConfig = {...config.database};
		poolConfig.database = "postgres";
		const con = await database.getPostgresqlConnection(poolConfig);
		await con.query("DROP DATABASE IF EXISTS txts_treasury_testing;");
		pool = await database.getPostgresqlConnection(config.database);
		db = await pool.connect();
		await user.init(pool, config);
	});

	this.afterAll("close database connection", async function() {
		db.release();	
		await pool.end();
	});

	describe("login", function() {
		it("can login correct admin user", async function() {
			assert.strictEqual(await user.login({name: config.admin_username, secret: config.admin_password}), 0);
		});
		
		it("doesnt login wrong admin user with correct admin password", async function() {
			assert.strictEqual(await user.login({name: config.admin_username + "a", secret: config.admin_password}), null);
		});
		
		it("doesnt login correct admin user with wrong admin password", async function() {
			assert.strictEqual(await user.login({name: config.admin_username, secret: config.admin_password + "a"}), null);
		});
	});
	
});