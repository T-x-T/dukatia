import { Pool } from "pg";
import { IAccount } from "../account";
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
	tags: ITag[],
	currency: ICurrency,
	recipient: IRecipient,
	status: ETransactionStatus
	timestamp: Date,
	amount: number,
	comment?: string,
}

type IShallowTransaction = {
	id?: number,
	userId?: number,
	accountId: number,
	currencyId: number,
	recipientId: number,
	status: ETransactionStatus,
	timestamp: Date,
	amount: number,
	comment?: string
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

	async add(transaction: IShallowTransaction) {
		if(!Number.isInteger(transaction.amount)) throw new Error("Amount must be a non floating point integer");
		return turnRowIntoShallowTransaction(await database.add(transaction));
	},

	async getAll() {
		return (await database.getAll()).map(x => turnRowIntoShallowTransaction(x));
	},

	async getById(id: number) {
		return (await database.getById(id)).map(x => turnRowIntoShallowTransaction(x));
	},

	async deleteById(id: number) {
		return await database.deleteById(id);
	},

	async update(transaction: IShallowTransaction) {
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
		comment: row.comment
	}
}