export default {};

declare global {
	type Account = {
		id?: number,
		name: string,
		default_currency_id?: number,
		user_id?: number,
		tag_ids?: number[],
		default_currency?: Currency,
	}

	type Asset = {
		id?: number,
		user_id: number,
		name: string,
		description?: string,
		currency_id: number,
		value_per_unit?: number,
		amount?: number,
		tag_ids?: number[],
		currency?: Currency,
	}

	type Currency = {
		id?: number,
		name: string,
		minor_in_mayor: number,
		symbol: string,
	}

	type Recipient = {
		id?: number,
		name: string,
		user_id?: number,
		tag_ids?: number[],
	}

	type Tag = {
		id?: number,
		name: string,
		user_id?: number,
		parent_id?: number,
	}

	type Transaction = {
		id?: number,
		user_id?: number,
		currency_id?: number,
		account_id: number,
		recipient_id: number,
		status: TransactionStatus,
		timestamp: string,
		amount: number,
		comment?: string,
		tag_ids?: number[],
		asset?: Asset,
		account?: Account,
		currency?: Currency,
		recipient?: Recipient,
		asset_id?: number,
	}

	enum TransactionStatus {
		Withheld, Completed
	}

	type SelectData = {
		options: {id: number, name: string}[],
		selected?: number[],
		label: string,
		openTop?: boolean
	}

	type Row = any[]
}