import assert from "assert"
import { Pool, PoolClient } from "pg";
import database from "../../database/postgresql/index.js";
import account from "../../lib/account/index.js";
import { ICurrency } from "../../lib/currency/index.js";

let db: PoolClient = null;
let pool: Pool = null;
let config: any = null;

const currency: ICurrency = {
	id: 0,
	name: "Euro",
	minorInMayor: 100,
	symbol: "€"
}

export default (_config: any) => config = _config;

describe("account", function() {

	this.beforeAll("clear Database", async function() {
		const poolConfig = {...config.database};
		poolConfig.database = "postgres";
		const con = await database.getPostgresqlConnection(poolConfig);
		await con.query("DROP DATABASE IF EXISTS txts_treasury_testing;");
		pool = await database.getPostgresqlConnection(config.database);
		db = await pool.connect();
		await account.init(pool);
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
			await assert.doesNotReject(() => account.add({name: "testAccount", defaultCurrency: 0}));
		});

		it("returns the newly added entry including the id", async function() {
			const res = await account.add({name: "testAccount", defaultCurrency: 0});
			
			assert.strictEqual(res.name, "testAccount");
			assert.strictEqual(res.defaultCurrency, 0);
			assert.strictEqual(typeof res.id, "number");
		});
	});

	describe("getByName", function() {
		it("returns only matching rows", async function() {
			await account.add({name: "testAccount", defaultCurrency: 0});
			await account.add({name: "testAccount2", defaultCurrency: 0});

			const res = await account.getByName("testAccount");
			assert.ok(res.length > 0);
			assert.strictEqual(res[0].name, "testAccount");
		});
	});

	describe("getAll", function() {
		it("returns all rows", async function() {
			await account.add({name: "testAccount", defaultCurrency: 0});
			await account.add({name: "testAccount2", defaultCurrency: 0});

			const res = await account.getAll();
			assert.strictEqual(res.length, 2);
		});
	});
	
	describe("deleteByName", function() {
		it("only deletes rows with matching name", async function() {
			await account.add({name: "testAccount", defaultCurrency: 0});
			await account.add({name: "testAccount2", defaultCurrency: 0});
			await account.add({name: "testAccount", defaultCurrency: 0});
			await account.add({name: "testAccount2", defaultCurrency: 0});

			await account.deleteByName("testAccount");

			const res = await account.getAll();
			assert.strictEqual(res.length, 2);
		});

		it("returns number of deleted rows", async function() {
			await account.add({name: "testAccount", defaultCurrency: 0});
			await account.add({name: "testAccount2", defaultCurrency: 0});
			await account.add({name: "testAccount", defaultCurrency: 0});
			await account.add({name: "testAccount2", defaultCurrency: 0});

			const res = await account.deleteByName("testAccount");

			assert.strictEqual(res, 2);
		});
	});

	describe("deleteByid", function() {
		it("only delete row with matching id", async function() {
			await account.add({name: "testAccount", defaultCurrency: 0});
			await account.add({name: "testAccount2", defaultCurrency: 0});
			await account.add({name: "testAccount", defaultCurrency: 0});
			await account.add({name: "testAccount2", defaultCurrency: 0});

			await account.deleteById(2);

			const res = await account.getAll();
			assert.strictEqual(res.length, 3);
			assert.strictEqual(res[0].id, 0);
			assert.strictEqual(res[1].id, 1);
			assert.strictEqual(res[2].id, 3);
		});

		it("returns number of deleted rows", async function() {
			await account.add({name: "testAccount", defaultCurrency: 0});
			await account.add({name: "testAccount2", defaultCurrency: 0});
			await account.add({name: "testAccount", defaultCurrency: 0});
			await account.add({name: "testAccount2", defaultCurrency: 0});

			const res = await account.deleteById(2);

			assert.strictEqual(res, 1);
		});
	});

	describe("deleteAll", function() {
		it("deletes all rows", async function() {
			await account.add({name: "testAccount", defaultCurrency: 0});
			await account.add({name: "testAccount2", defaultCurrency: 0});

			await account.deleteAll();

			const res = await account.getAll();
			assert.strictEqual(res.length, 0);
		});

		it("returns number of deleted rows", async function() {
			await account.add({name: "testAccount", defaultCurrency: 0});
			await account.add({name: "testAccount2", defaultCurrency: 0});

			const res = await account.deleteAll();

			assert.strictEqual(res, 2);
		});
	});

});