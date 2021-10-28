import assert from "assert"
import { Pool, PoolClient } from "pg";
import database from "../../database/postgresql/index.js";
import recipient from "../../lib/recipient/index.js";

let db: PoolClient = null;
let pool: Pool = null;
let config: any = null;

export default (_config: any) => config = _config;

describe("recipient", function() {

	this.beforeAll("clear Database", async function() {
		const poolConfig = {...config.database};
		poolConfig.database = "postgres";
		const con = await database.getPostgresqlConnection(poolConfig);
		await con.query("DROP DATABASE IF EXISTS txts_treasury_testing;");
		pool = await database.getPostgresqlConnection(config.database);
		db = await pool.connect();
		await recipient.init(pool);
	});

	this.afterAll("close database connection", async function() {
		db.release();	
		await pool.end();
	});

	beforeEach("clear Recipients table", async function() {
		await db.query("DELETE FROM public.\"Recipients\"; ALTER SEQUENCE public.\"Recipients_id_seq\" RESTART WITH 0");
	});

	describe("add", function() {
		it("doesnt reject", async function() {
			await assert.doesNotReject(() => recipient.add({name: "test"}));
		});

		it("returns the newly added entry including the id", async function() {
			const res = await recipient.add({name: "test"});
			
			assert.strictEqual(res.name, "test");
			assert.strictEqual(typeof res.id, "number");
		});
	});

	describe("getAll", function() {
		it("returns all rows", async function() {
			await recipient.add({name: "test"});
			await recipient.add({name: "test2"});

			const res = await recipient.getAll();
			assert.strictEqual(res.length, 2);
		});
	});

});