import { Pool } from "pg";
import account, { IAccount } from "../account/index.js";
import { ICurrency } from "../currency";
import { IRecipient } from "../recipient";
import { ITag } from "../tag";
import { IUser } from "../user";
import database from "./database.js";
import restApi from "./restApi.js";

type ITransaction = {
	id: number,
	user: IUser,
	account: IAccount,
	currency: ICurrency,
	recipient: IRecipient,
	status: ETransactionStatus
	timestamp: Date,
	amount: number,
	comment?: string,
	tagIds?: number[]
}

type IShallowTransaction = {
	id?: number,
	userId?: number,
	currencyId?: number,
	accountId: number,
	recipientId: number,
	status: ETransactionStatus,
	timestamp: Date,
	amount: number,
	comment?: string,
	tagIds?: number[]
}

enum ETransactionStatus {
	withheld,
	completed
}

export type { ITransaction, IShallowTransaction, ETransactionStatus };

export default {
	async init(db: Pool) {
		database.init(db);
		restApi();
	},

	async add(transaction: IShallowTransaction): Promise<IShallowTransaction> {
		if(!Number.isInteger(transaction.amount)) throw new Error("Amount must be a non floating point integer");
		const accountOfTransaction = await account.getById(transaction.accountId);
		return turnRowIntoShallowTransaction(await database.add({...transaction, currencyId: accountOfTransaction.defaultCurrency}));
	},

	async getAll(): Promise<IShallowTransaction[]> {
		return (await database.getAll()).map(x => turnRowIntoShallowTransaction(x));
	},

	async getById(id: number): Promise<IShallowTransaction> {
		return turnRowIntoShallowTransaction(await database.getById(id));
	},

	async deleteById(id: number): Promise<number> {
		return await database.deleteById(id);
	},

	async update(transaction: IShallowTransaction): Promise<IShallowTransaction> {
		if(!Number.isInteger(transaction.id)) throw new Error("no valid id specified");
		const res = await database.update(transaction);
		if(res.length === 0) throw new Error("no row with id: " + transaction.id);
		return turnRowIntoShallowTransaction(res[0]);
	}
}

function turnRowIntoShallowTransaction(row: any): IShallowTransaction {
	return {
		id: row.id,
		userId: row.user,
		accountId: row.account,
		currencyId: row.currency,
		recipientId: row.recipient,
		status: row.status,
		timestamp: row.timestamp,
		amount: row.amount,
		comment: row.comment,
		tagIds: Array.isArray(row.tags) && row.tags[0] !== null ? row.tags : undefined
	}
}