import assert from "assert"
import { Pool, PoolClient } from "pg";
import database from "../../database/postgresql/index.js";
import account from "../../lib/account/index.js";
import tag from "../../lib/tag/index.js";

let db: PoolClient = null;
let pool: Pool = null;
let config: any = null;

const testAccount = {
	name: "testAccount",
	defaultCurrency: 0
}

const testAccount2 = {
	name: "testAccount",
	defaultCurrency: 0
}

const testTag = {
	name: "test"
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
		await tag.init(pool);
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
			await assert.doesNotReject(() => account.add(testAccount));
		});

		it("doesnt reject with single tag", async function() {
			await tag.add(testTag);
			await assert.doesNotReject(() => account.add({...testAccount, tagIds: [0]}));
		});

		it("doesnt reject with multiple tags", async function() {
			await tag.add(testTag);
			await tag.add(testTag);
			await tag.add(testTag);
			await assert.doesNotReject(() => account.add({...testAccount, tagIds: [0, 2, 1]}));
		});

		it("returns the newly added entry including the id", async function() {
			const res = await account.add(testAccount);
			
			assert.strictEqual(res.name, "testAccount");
			assert.strictEqual(res.defaultCurrency, 0);
			assert.strictEqual(typeof res.id, "number");
		});

		it("returns the newly added entry without tagIds", async function() {
			await tag.add(testTag);
			const res = await account.add({...testAccount, tagIds: [0]});

			assert.strictEqual(res.id, 0);
			assert.strictEqual(res.tagIds, undefined);
		});
	});

	describe("getById", function() {
		it("returns only matching rows", async function() {
			await account.add(testAccount);
			await account.add(testAccount2);

			const res = await account.getById(0);
			assert.strictEqual(res.name, "testAccount");
		});

		it("returns single tag correctly", async function() {
			await tag.add(testTag);
			await account.add(testAccount);
			await account.add({...testAccount2, tagIds: [0]});

			const res = await account.getById(1);
			assert.strictEqual(res.id, 1);
			assert.deepStrictEqual(res.tagIds, [0]);
		});

		it("returns multiple tags correctly", async function() {
			await tag.add(testTag);
			await tag.add(testTag);
			await tag.add(testTag);
			await account.add(testAccount);
			await account.add({...testAccount2, tagIds: [0, 2, 1]});

			const res = await account.getById(1);
			assert.strictEqual(res.id, 1);
			assert.strictEqual(res.tagIds.length, 3);
			assert.ok(res.tagIds.includes(0));
			assert.ok(res.tagIds.includes(1));
			assert.ok(res.tagIds.includes(2));
		});
	});

	describe("getAll", function() {
		it("returns all rows", async function() {
			await account.add(testAccount);
			await account.add(testAccount2);

			const res = await account.getAll();
			assert.strictEqual(res.length, 2);
		});

		it("returns single tag correctly", async function() {
			await tag.add(testTag);
			await account.add(testAccount);
			await account.add({...testAccount2, tagIds: [0]});

			const res = await account.getAll();
			assert.strictEqual(res[1].id, 1);
			assert.deepStrictEqual(res[1].tagIds, [0]);
		});

		it("returns multiple tags correctly", async function() {
			await tag.add(testTag);
			await tag.add(testTag);
			await tag.add(testTag);
			await account.add(testAccount);
			await account.add({...testAccount2, tagIds: [0, 2, 1]});

			const res = await account.getAll();
			assert.strictEqual(res[1].id, 1);
			assert.strictEqual(res[1].tagIds.length, 3);
			assert.ok(res[1].tagIds.includes(0));
			assert.ok(res[1].tagIds.includes(1));
			assert.ok(res[1].tagIds.includes(2));
		});
	});

	describe("deleteByid", function() {
		it("only delete row with matching id", async function() {
			await account.add(testAccount);
			await account.add(testAccount2);
			await account.add(testAccount);
			await account.add(testAccount2);

			await account.deleteById(2);

			const res = await account.getAll();
			assert.strictEqual(res.length, 3);
			assert.strictEqual(res[0].id, 0);
			assert.strictEqual(res[1].id, 1);
			assert.strictEqual(res[2].id, 3);
		});

		it("returns number of deleted rows", async function() {
			await account.add(testAccount);
			await account.add(testAccount2);
			await account.add(testAccount);
			await account.add(testAccount2);

			const res = await account.deleteById(2);

			assert.strictEqual(res, 1);
		});
	});

	describe("deleteAll", function() {
		it("deletes all rows", async function() {
			await account.add(testAccount);
			await account.add(testAccount2);

			await account.deleteAll();

			const res = await account.getAll();
			assert.strictEqual(res.length, 0);
		});

		it("returns number of deleted rows", async function() {
			await account.add(testAccount);
			await account.add(testAccount2);

			const res = await account.deleteAll();

			assert.strictEqual(res, 2);
		});
	});

	describe("update", function() {
		it("throws without an id", async function() {
			await account.add(testAccount);

			await assert.rejects(() => account.update(testAccount), new Error("no valid id specified"));
		});

		it("doesnt throw with correct input", async function() {
			await account.add(testAccount);

			await assert.doesNotReject(() => account.update({...testAccount, id: 0}));
		});

		it("throws when no account with id exists", async function() {
			await account.add(testAccount);

			await assert.rejects(() => account.update({...testAccount, id: 1}), new Error("no row with id: 1"));
		});

		it("correctly returns new values", async function() {
			await account.add(testAccount);

			const res = await account.update({...testAccount2, id: 0});

			assert.strictEqual(res.name, testAccount2.name);
			assert.strictEqual(res.defaultCurrency, testAccount2.defaultCurrency);
		});

		it("correctly sets new values", async function() {
			await account.add(testAccount);

			await account.update({...testAccount2, id: 0});
			
			const res = (await account.getById(0));

			assert.strictEqual(res.name, testAccount2.name);
			assert.strictEqual(res.defaultCurrency, testAccount2.defaultCurrency);
		});

		it("doesnt set tags when no tags have been set and should be set", async function() {
			await tag.add(testTag);
			await account.add(testAccount);

			await account.update({...testAccount, id: 0});

			const res = (await account.getAll())[0];

			assert.strictEqual(res.tagIds, undefined);
		});

		it("removes tags when tags have been set", async function() {
			await tag.add(testTag);
			await account.add({...testAccount, tagIds: [0]});

			await account.update({...testAccount, id: 0});

			const res = (await account.getAll())[0];

			assert.strictEqual(res.tagIds, undefined);
		});

		it("correctly sets tags when no tags have been set", async function() {
			await tag.add(testTag);
			await account.add(testAccount);

			await account.update({...testAccount, id: 0, tagIds: [0]});

			const res = (await account.getAll())[0];

			assert.deepStrictEqual(res.tagIds, [0]);
		});

		it("correctly sets changed tags", async function() {
			await tag.add(testTag);
			await tag.add(testTag);
			await account.add({...testAccount, tagIds: [0]});

			await account.update({...testAccount, id: 0, tagIds: [1]});

			const res = (await account.getAll())[0];

			assert.deepStrictEqual(res.tagIds, [1]);
		});
	});

});