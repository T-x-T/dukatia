import assert, { strictEqual } from "assert"
import { Pool, PoolClient } from "pg";
import database from "../../database/postgresql/index.js";
import account from "../../lib/account/index.js";
import recipient from "../../lib/recipient/index.js";
import transaction, { IShallowTransaction } from "../../lib/transaction/index.js";
import user from "../../lib/user/index.js";
import tag from "../../lib/tag/index.js";

let db: PoolClient = null;
let pool: Pool = null;
let config: any = null;

export default (_config: any) => config = _config;

const testTransaction: IShallowTransaction = {
	userId: 0,
	accountId: 0,
	currencyId: 0,
	recipientId: 0,
	status: 0,
	timestamp: new Date(),
	amount: 42069,
	comment: "this is a comment"
}

const testTransaction2: IShallowTransaction = {
	userId: 0,
	accountId: 1,
	currencyId: 1,
	recipientId: 1,
	status: 1,
	timestamp: new Date("2021-01-01"),
	amount: 123456,
	comment: "this is another comment"
}

const testTag = {
	name: "test",
	userId: 0
}

describe("transaction", function() {

	this.beforeAll("clear Database", async function() {
		const poolConfig = {...config.database};
		poolConfig.database = "postgres";
		const con = await database.getPostgresqlConnection(poolConfig);
		await con.query("DROP DATABASE IF EXISTS txts_treasury_testing;");
		pool = await database.getPostgresqlConnection(config.database);
		db = await pool.connect();
		await user.init(pool, config);
		await transaction.init(pool);
		await recipient.init(pool);
		await account.init(pool);
		await tag.init(pool);
	});

	this.afterAll("close database connection", async function() {
		db.release();	
		await pool.end();
	});


	beforeEach("clear Transactions table", async function() {
		await db.query("DELETE FROM public.\"Transactions\"; ALTER SEQUENCE public.\"Transactions_id_seq\" RESTART WITH 0");
	});

	describe("add", function() {
		it("doesnt reject with correct input", async function() {
			await assert.doesNotReject(() => transaction.add(testTransaction));
		});

		it("rejects with floating point number as amount", async function() {
			await assert.rejects(() => transaction.add({...testTransaction, amount: 123.45}), new Error("Amount must be a non floating point integer"));
		});

		it("doesnt reject with single tag", async function() {
			await tag.add(testTag);
			await assert.doesNotReject(() => transaction.add({...testTransaction, tagIds: [0]}));
		});

		it("doesnt reject with multiple tags", async function() {
			await tag.add(testTag);
			await tag.add(testTag);
			await tag.add(testTag);
			await assert.doesNotReject(() => transaction.add({...testTransaction, tagIds: [0, 2, 1]}));
		});

		it("returns the newly added entry including the id", async function() {
			const res = await transaction.add(testTransaction);
			
			assert.strictEqual(res.id, 0);
		});

		it("returns the newly added entry without tagIds", async function() {
			await tag.add(testTag);
			const res = await transaction.add({...testTransaction, tagIds: [0]});

			assert.strictEqual(res.id, 0);
			assert.strictEqual(res.tagIds, undefined);
		});
	});

	describe("getAll", function() {
		it("returns all rows", async function() {
			await transaction.add(testTransaction);
			await transaction.add(testTransaction);
			await transaction.add(testTransaction);

			const res = await transaction.getAll();
			
			assert.strictEqual(res.length, 3);
		});

		it("returns single tag correctly", async function() {
			await tag.add(testTag);
			await transaction.add(testTransaction);
			await transaction.add({...testTransaction, tagIds: [0]});

			const res = await transaction.getAll();
			assert.strictEqual(res[1].id, 1);
			assert.deepStrictEqual(res[1].tagIds, [0]);
		});

		it("returns multiple tags correctly", async function() {
			await tag.add(testTag);
			await tag.add(testTag);
			await tag.add(testTag);
			await transaction.add(testTransaction);
			await transaction.add({...testTransaction, tagIds: [0, 2, 1]});

			const res = await transaction.getAll();
			assert.strictEqual(res[1].id, 1);
			assert.strictEqual(res[1].tagIds.length, 3);
			assert.ok(res[1].tagIds.includes(0));
			assert.ok(res[1].tagIds.includes(1));
			assert.ok(res[1].tagIds.includes(2));
		});
	});

	describe("getById", function() {
		it("returns the correct row", async function() {
			await transaction.add(testTransaction);
			await transaction.add({...testTransaction, comment: "test"});
			await transaction.add(testTransaction);

			const res = await transaction.getById(1);

			assert.strictEqual(res.comment, "test");
		});

		it("returns single tag correctly", async function() {
			await tag.add(testTag);
			await transaction.add(testTransaction);
			await transaction.add({...testTransaction, tagIds: [0]});

			const res = await transaction.getById(1);
			assert.strictEqual(res.id, 1);
			assert.deepStrictEqual(res.tagIds, [0]);
		});

		it("returns multiple tags correctly", async function() {
			await tag.add(testTag);
			await tag.add(testTag);
			await tag.add(testTag);
			await transaction.add(testTransaction);
			await transaction.add({...testTransaction, tagIds: [0, 2, 1]});

			const res = await transaction.getById(1);
			assert.strictEqual(res.id, 1);
			assert.strictEqual(res.tagIds.length, 3);
			assert.ok(res.tagIds.includes(0));
			assert.ok(res.tagIds.includes(1));
			assert.ok(res.tagIds.includes(2));
		});
	});

	describe("deleteById", function() {
		it("deletes correct transaction", async function() {
			await transaction.add({...testTransaction, comment: "test1"});
			await transaction.add({...testTransaction, comment: "test2"});
			await transaction.add({...testTransaction, comment: "test3"});

			await transaction.deleteById(1);

			const res = await transaction.getAll();
			
			assert.strictEqual(res[1].comment, "test3");
		});

		it("returns 1 when transaction was deleted", async function() {
			await transaction.add(testTransaction);

			const res = await transaction.deleteById(0);

			assert.strictEqual(res, 1);
		});

		it("returns 0 when no transaction was deleted", async function() {
			await transaction.add(testTransaction);

			const res = await transaction.deleteById(1);

			assert.strictEqual(res, 0);
		});
	});

	describe("update", function() {
		it("throws without an id", async function() {
			await transaction.add(testTransaction);

			await assert.rejects(() => transaction.update(testTransaction), new Error("no valid id specified"));
		});

		it("doesnt throw with correct input", async function() {
			await transaction.add(testTransaction);

			await assert.doesNotReject(() => transaction.update({...testTransaction, id: 0}));
		});

		it("throws when no transaction with id exists", async function() {
			await transaction.add(testTransaction);

			await assert.rejects(() => transaction.update({...testTransaction, id: 1}), new Error("no row with id: 1"));
		});

		it("correctly returns new values", async function() {
			await transaction.add(testTransaction);

			
			await account.add({name: "another account", defaultCurrency: 1, userId: 0});
			await recipient.add({name: "another recipient", userId: 0});

			const res = await transaction.update({...testTransaction2, id: 0});

			assert.strictEqual(res.accountId, testTransaction2.accountId);
			assert.strictEqual(res.amount, testTransaction2.amount);
			assert.strictEqual(res.comment, testTransaction2.comment);
			assert.strictEqual(res.currencyId, testTransaction2.currencyId);
			assert.strictEqual(res.recipientId, testTransaction2.recipientId);
			assert.strictEqual(res.status, testTransaction2.status);
			assert.deepStrictEqual(res.timestamp, testTransaction2.timestamp);
		});

		it("correctly sets new values", async function() {
			await transaction.add(testTransaction);

			
			await account.add({name: "another account", defaultCurrency: 1, userId: 0});
			await recipient.add({name: "another recipient", userId: 0});

			await transaction.update({...testTransaction2, id: 0});
			
			const res = (await transaction.getById(0));

			assert.strictEqual(res.accountId, testTransaction2.accountId);
			assert.strictEqual(res.amount, testTransaction2.amount);
			assert.strictEqual(res.comment, testTransaction2.comment);
			assert.strictEqual(res.currencyId, testTransaction2.currencyId);
			assert.strictEqual(res.recipientId, testTransaction2.recipientId);
			assert.strictEqual(res.status, testTransaction2.status);
			assert.deepStrictEqual(res.timestamp, testTransaction2.timestamp);
		});

		it("doesnt set tags when no tags have been set and should be set", async function() {
			await tag.add(testTag);
			await transaction.add(testTransaction);

			await transaction.update({...testTransaction, id: 0});

			const res = (await transaction.getAll())[0];

			assert.strictEqual(res.tagIds, undefined);
		});

		it("removes tags when tags have been set", async function() {
			await tag.add(testTag);
			await transaction.add({...testTransaction, tagIds: [0]});

			await transaction.update({...testTransaction, id: 0});

			const res = (await transaction.getAll())[0];

			assert.strictEqual(res.tagIds, undefined);
		});

		it("correctly sets tags when no tags have been set", async function() {
			await tag.add(testTag);
			await transaction.add(testTransaction);

			await transaction.update({...testTransaction, id: 0, tagIds: [0]});

			const res = (await transaction.getAll())[0];

			assert.deepStrictEqual(res.tagIds, [0]);
		});

		it("correctly sets changed tags", async function() {
			await tag.add(testTag);
			await tag.add(testTag);
			await transaction.add({...testTransaction, tagIds: [0]});

			await transaction.update({...testTransaction, id: 0, tagIds: [1]});

			const res = (await transaction.getAll())[0];

			assert.deepStrictEqual(res.tagIds, [1]);
		});
	})

});