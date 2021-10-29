import assert, { strictEqual } from "assert"
import { Pool, PoolClient } from "pg";
import database from "../../database/postgresql/index.js";
import account from "../../lib/account/index.js";
import recipient from "../../lib/recipient/index.js";
import transaction, { IShallowTransaction } from "../../lib/transaction/index.js";
import user from "../../lib/user/index.js";

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
	});

	describe("getAll", function() {
		it("returns all rows", async function() {
			await transaction.add(testTransaction);
			await transaction.add(testTransaction);
			await transaction.add(testTransaction);

			const res = await transaction.getAll();
			
			assert.strictEqual(res.length, 3);
		});
	});

	describe("getById", function() {
		it("returns the correct row", async function() {
			await transaction.add(testTransaction);
			await transaction.add({...testTransaction, comment: "test"});
			await transaction.add(testTransaction);

			const res = await transaction.getAll();

			assert.strictEqual(res[1].comment, "test");
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

			
			await account.add({name: "another account", defaultCurrency: 1});
			await recipient.add({name: "another recipient"});

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

			
			await account.add({name: "another account", defaultCurrency: 1});
			await recipient.add({name: "another recipient"});

			await transaction.update({...testTransaction2, id: 0});
			
			const res = (await transaction.getById(0))[0];

			assert.strictEqual(res.accountId, testTransaction2.accountId);
			assert.strictEqual(res.amount, testTransaction2.amount);
			assert.strictEqual(res.comment, testTransaction2.comment);
			assert.strictEqual(res.currencyId, testTransaction2.currencyId);
			assert.strictEqual(res.recipientId, testTransaction2.recipientId);
			assert.strictEqual(res.status, testTransaction2.status);
			assert.deepStrictEqual(res.timestamp, testTransaction2.timestamp);
		});
	})

});