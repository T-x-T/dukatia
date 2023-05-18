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

	type AssetValuation = {
		value_per_unit: number,
		amount: number,
		timestamp: string,
		deleted?: boolean,
	}

	type Row = any[]

	type TableSort = {
		column: number,
		sort: "asc" | "desc",
	}

	type ColumnType =
		"number" |
		"string" |
		"choice" |
		"date"

	type Column = {
		name: string,
		type: ColumnType,
		options?: string[],
	}

	type TableData = {
		multiSelect: boolean,
		defaultSort: TableSort,
		columns: Column[],
		rows: Row[],
		displaySum?: boolean,
		sumColumn?: number,
	}

	type TableFilterOption = 
		"is" |
		"isnt" |
		"contains" |
		"between" |
		"outside" |
		"exact" |
		"less" |
		"more" |
		"begins" |
		"ends" |
		"doesntcontain" |
		"notempty" |
		"empty" |
		"anything"

	type TableFilter = {
		type: ColumnType,
		option: TableFilterOption,
		empty: string,
		value?: string | number,
		start?: string,
		end?: string,
	}

	type DetailFormField = {
		label: string,
		property: string,
		type: "number" | "string" | "tags" | "currency" | "singleTag" | "timestamp" | "account" | "recipient" | "asset",
		disabled?: boolean,
		step?: string,
		initial?: number,
		suffix?: "currencyOfAccountSymbol",
		addNew?: boolean,
	}

	type DetailFormConfig = {
		fields: DetailFormField[],
		data: any,
		apiEndpoint: string,
		prepareForApi: (x: any) => any,
		defaultData: Object,
		deletable?: boolean,
		noSaveAndNew?: boolean,
		noGoBackOnSave?: boolean,
		reset_default_currency_id?: boolean,
		populateTagsUsingRecipient?: boolean,
		tableData?: TableData,
	}

	type Dashboard = {
		id?: number,
		user_id: number,
		name: string,
		description?: string,
	}

	type ChartOptions = {
		id: number,
		grid_size: "small" | "medium" | "large",
		chart_type: "text" | "pie" | "line",
		title: string,
		text_template?: string,
		date_period: "daily" | "monthly" | "quarterly" | "yearly",
	}
}