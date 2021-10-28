import assert from "assert"
import { Pool, PoolClient } from "pg";
import database from "../../database/postgresql/index.js";
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
});