import assert from "assert"
import { Pool, PoolClient } from "pg";
import database from "../../database/postgresql/index.js";
import tag from "../../lib/tag/index.js";
import user from "../../lib/user/index.js";

let db: PoolClient = null;
let pool: Pool = null;
let config: any = null;

const testTag = {
	name: "test",
	userId: 0
}

const testTag2 = {
	name: "test2",
	userId: 0
}

export default (_config: any) => config = _config;

describe("tag", function() {

	this.beforeAll("clear Database", async function() {
		const poolConfig = {...config.database};
		poolConfig.database = "postgres";
		const con = await database.getPostgresqlConnection(poolConfig);
		await con.query("DROP DATABASE IF EXISTS txts_treasury_testing;");
		pool = await database.getPostgresqlConnection(config.database);
		db = await pool.connect();
		await tag.init(pool);
		await user.init(pool, config);
	});

	this.afterAll("close database connection", async function() {
		db.release();	
		await pool.end();
	});

	beforeEach("clear Recipients table", async function() {
		await db.query("DELETE FROM public.\"Tags\"; ALTER SEQUENCE public.\"Tags_id_seq\" RESTART WITH 0");
	});

	describe("add", function() {
		it("doesnt reject", async function() {
			await assert.doesNotReject(() => tag.add(testTag));
		});

		it("doesnt reject when containing a valid parentId", async function() {
			await tag.add(testTag);
			await assert.doesNotReject(() => tag.add({...testTag2, parentId: 0}));
		});

		it("rejects when parentId doesnt exist", async function() {
			await assert.rejects(() => tag.add({...testTag, parentId: 1}), new Error("specified parentTagId is invalid"));
		});
		
		it("returns the newly added entry including the id", async function() {
			const res = await tag.add(testTag);
			
			assert.strictEqual(res.name, "test");
			assert.strictEqual(res.id, 0);
		});

		it("returns the newly added entry with parentId", async function() {
			await tag.add(testTag);
			const res = await tag.add({...testTag2, parentId: 0});

			assert.strictEqual(res.name, "test2");
			assert.strictEqual(res.id, 1);
			assert.strictEqual(res.parentId, 0);
		});
	});

	describe("getAll", function() {
		it("returns all rows", async function() {
			await tag.add(testTag);
			await tag.add(testTag);

			const res = await tag.getAll();
			assert.strictEqual(res.length, 2);
		});
	});

	describe("getById", function() {
		it("returns correct entry", async function() {
			await tag.add(testTag2);
			await tag.add(testTag);
			await tag.add(testTag2);

			const res = await tag.getById(1);
			assert.strictEqual(res.id, 1);
			assert.strictEqual(res.name, "test");
		});
	});

	describe("update", function() {
		it("throws without an id", async function() {
			await tag.add(testTag);

			await assert.rejects(() => tag.update(testTag), new Error("no valid id specified"));
		});

		it("doesnt throw with correct input", async function() {
			await tag.add(testTag);

			await assert.doesNotReject(() => tag.update({...testTag, id: 0}));
		});

		it("throws when no tag with id exists", async function() {
			await tag.add(testTag);

			await assert.rejects(() => tag.update({...testTag, id: 1}), new Error("no row with id: 1"));
		});

		it("correctly returns new values", async function() {
			await tag.add(testTag);

			const res = await tag.update({...testTag2, id: 0});

			assert.strictEqual(res.name, testTag2.name);
		});

		it("correctly returns new values with parentId", async function() {
			await tag.add(testTag);
			await tag.add(testTag);

			const res = await tag.update({...testTag2, id: 1, parentId: 0});

			assert.strictEqual(res.name, testTag2.name);
			assert.strictEqual(res.parentId, 0);
		});

		it("correctly sets new values", async function() {
			await tag.add(testTag);

			await tag.update({...testTag2, id: 0});
			
			const res = (await tag.getAll())[0];

			assert.strictEqual(res.name, testTag2.name);
		});

		it("correctly sets new values with parentId", async function() {
			await tag.add(testTag);
			await tag.add(testTag);

			await tag.update({...testTag2, id: 1, parentId: 0});
			
			const res = await tag.getById(1);

			assert.strictEqual(res.name, testTag2.name);
			assert.strictEqual(res.parentId, 0);
		});

		it("rejects when trying to create circular relationship", async function() {
			await tag.add(testTag);
			await tag.add({...testTag, parentId: 0});
			await tag.add({...testTag, parentId: 1});

			await assert.rejects(() => tag.update({...testTag, id: 0, parentId: 2}), new Error("specified parentTagId is invalid"));
		});
	});

});