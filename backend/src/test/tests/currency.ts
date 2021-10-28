import assert from "assert"
import { Pool, PoolClient } from "pg";
import database from "../../database/postgresql/index.js";
import currency from "../../lib/currency/index.js";

let pool: Pool = null;
let db: PoolClient = null;
let config: any = null;

export default (_config: any) => config = _config;

describe("currency", function() {

	this.beforeAll("clear Database", async function() {
		const poolConfig = {...config.database};
		poolConfig.database = "postgres";
		const con = await database.getPostgresqlConnection(poolConfig);
		await con.query("DROP DATABASE IF EXISTS txts_treasury_testing;");
		pool = await database.getPostgresqlConnection(config.database);
		db = await pool.connect();
		await currency.init(pool);
	});

	this.afterAll("close database connection", async function() {
		db.release();	
		await pool.end();
	});

	beforeEach("clear Currencies and Accounts table", async function() {
		await db.query("DELETE FROM public.\"Accounts\"; ALTER SEQUENCE public.\"Accounts_id_seq\" RESTART WITH 0");
		await db.query("DELETE FROM public.\"Currencies\"; ALTER SEQUENCE public.\"Currencies_id_seq\" RESTART WITH 0");
	});

	describe("add", function() {
		it("doesnt reject", async function() {
			await assert.doesNotReject(() => currency.add({name: "test", minorInMayor: 100, symbol: "@"}));
		});

		it("returns the newly added entry including the id", async function() {
			const res = await currency.add({name: "test", minorInMayor: 100, symbol: "@"});
			
			assert.strictEqual(res.name, "test");
			assert.strictEqual(res.minorInMayor, 100);
			assert.strictEqual(res.symbol, "@");
			assert.strictEqual(typeof res.id, "number");
		});
	});

	describe("getAll", function() {
		it("returns all rows", async function() {
			await currency.add({name: "test", minorInMayor: 100, symbol: "@"});
			await currency.add({name: "test2", minorInMayor: 50, symbol: "!!"});

			const res = await currency.getAll();
			assert.strictEqual(res.length, 2);
		});
	});

});