import assert from "assert"
import { Pool, PoolClient } from "pg";
import database from "../../database/postgresql/index.js";
import recipient from "../../lib/recipient/index.js";
import tag from "../../lib/tag/index.js";

let db: PoolClient = null;
let pool: Pool = null;
let config: any = null;

const testRecipient = {
	name: "test"
}

const testRecipient2 = {
	name: "test2"
}

const testTag = {
	name: "test"
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
		await tag.init(pool);
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

		it("doesnt reject with single tag", async function() {
			await tag.add(testTag);
			await assert.doesNotReject(() => recipient.add({...testRecipient, tagIds: [0]}));
		});

		it("doesnt reject with multiple tags", async function() {
			await tag.add(testTag);
			await tag.add(testTag);
			await tag.add(testTag);
			await assert.doesNotReject(() => recipient.add({...testRecipient, tagIds: [0, 2, 1]}));
		});

		it("returns the newly added entry including the id", async function() {
			const res = await recipient.add(testRecipient);
			
			assert.strictEqual(res.name, "test");
			assert.strictEqual(res.id, 0);
		});

		it("returns the newly added entry without tagIds", async function() {
			await tag.add(testTag);
			const res = await recipient.add({...testRecipient, tagIds: [0]});

			assert.strictEqual(res.id, 0);
			assert.strictEqual(res.tagIds, undefined);
		});
	});

	describe("getAll", function() {
		it("returns all rows", async function() {
			await recipient.add(testRecipient);
			await recipient.add(testRecipient2);

			const res = await recipient.getAll();
			assert.strictEqual(res.length, 2);
			assert.strictEqual(res[1].id, 1);
		});

		it("returns single tag correctly", async function() {
			await tag.add(testTag);
			await recipient.add(testRecipient);
			await recipient.add({...testRecipient2, tagIds: [0]});

			const res = await recipient.getAll();
			assert.strictEqual(res[1].id, 1);
			assert.deepStrictEqual(res[1].tagIds, [0]);
		});

		it("returns multiple tags correctly", async function() {
			await tag.add(testTag);
			await tag.add(testTag);
			await tag.add(testTag);
			await recipient.add(testRecipient);
			await recipient.add({...testRecipient2, tagIds: [0, 2, 1]});

			const res = await recipient.getAll();
			assert.strictEqual(res[1].id, 1);
			assert.strictEqual(res[1].tagIds.length, 3);
			assert.ok(res[1].tagIds.includes(0));
			assert.ok(res[1].tagIds.includes(1));
			assert.ok(res[1].tagIds.includes(2));
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

		it("doesnt set tags when no tags have been set and should be set", async function() {
			await tag.add(testTag);
			await recipient.add(testRecipient);

			await recipient.update({...testRecipient, id: 0});

			const res = (await recipient.getAll())[0];

			assert.strictEqual(res.tagIds, undefined);
		});

		it("removes tags when tags have been set", async function() {
			await tag.add(testTag);
			await recipient.add({...testRecipient, tagIds: [0]});

			await recipient.update({...testRecipient, id: 0});

			const res = (await recipient.getAll())[0];

			assert.strictEqual(res.tagIds, undefined);
		});

		it("correctly sets tags when no tags have been set", async function() {
			await tag.add(testTag);
			await recipient.add(testRecipient);

			await recipient.update({...testRecipient, id: 0, tagIds: [0]});

			const res = (await recipient.getAll())[0];

			assert.deepStrictEqual(res.tagIds, [0]);
		});

		it("correctly sets changed tags", async function() {
			await tag.add(testTag);
			await tag.add(testTag);
			await recipient.add({...testRecipient, tagIds: [0]});

			await recipient.update({...testRecipient, id: 0, tagIds: [1]});

			const res = (await recipient.getAll())[0];

			assert.deepStrictEqual(res.tagIds, [1]);
		});
	});

});