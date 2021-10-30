import assert from "assert"
import { Pool, PoolClient } from "pg";
import database from "../../database/postgresql/index.js";
import recipient from "../../lib/recipient/index.js";

let db: PoolClient = null;
let pool: Pool = null;
let config: any = null;

const testRecipient = {
	name: "test"
}

const testRecipient2 = {
	name: "test2"
}

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
			await assert.doesNotReject(() => recipient.add(testRecipient));
		});

		it("returns the newly added entry including the id", async function() {
			const res = await recipient.add(testRecipient);
			
			assert.strictEqual(res.name, "test");
			assert.strictEqual(typeof res.id, "number");
		});
	});

	describe("getAll", function() {
		it("returns all rows", async function() {
			await recipient.add(testRecipient);
			await recipient.add(testRecipient2);

			const res = await recipient.getAll();
			assert.strictEqual(res.length, 2);
		});
	});
	
	describe("update", function() {
		it("throws without an id", async function() {
			await recipient.add(testRecipient);

			await assert.rejects(() => recipient.update(testRecipient), new Error("no valid id specified"));
		});

		it("doesnt throw with correct input", async function() {
			await recipient.add(testRecipient);

			await assert.doesNotReject(() => recipient.update({...testRecipient, id: 0}));
		});

		it("throws when no recipient with id exists", async function() {
			await recipient.add(testRecipient);

			await assert.rejects(() => recipient.update({...testRecipient, id: 1}), new Error("no row with id: 1"));
		});

		it("correctly returns new values", async function() {
			await recipient.add(testRecipient);

			const res = await recipient.update({...testRecipient2, id: 0});

			assert.strictEqual(res.name, testRecipient2.name);
		});

		it("correctly sets new values", async function() {
			await recipient.add(testRecipient);

			await recipient.update({...testRecipient2, id: 0});
			
			const res = (await recipient.getAll())[0];

			assert.strictEqual(res.name, testRecipient2.name);
		});
	});

});