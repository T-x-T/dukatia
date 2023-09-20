export default {};

declare global {
	type Account = {
		id?: number,
		name: string,
		default_currency_id?: number,
		user_id?: number,
		tag_ids?: number[],
		default_currency?: Currency,
		balance?: number,
	}
	
	type DeepAccount = {
		id: number,
		name: string,
		default_currency: Currency,
		user: User,
		tags: DeepTag[],
	}

	type User = {
		id?: number,
		name: string,
		secret?: string,
		superuser: boolean,
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
		total_cost_of_ownership?: TotalCostOfOwnership,
	}

	type DeepAsset = {
		id: number,
		name: string,
		description?: string,
		value_per_unit: number,
		amount: number,
		user: User,
		currency: Currency,
		tags: DeepTag[],
		total_cost_of_ownership?: TotalCostOfOwnership,
	}

	type TotalCostOfOwnership = {
		total: number,
		monthly: number,
		yearly: number,
	}

	type Currency = {
		id?: number,
		name: string,
		minor_in_major: number,
		symbol: string,
	}

	type Recipient = {
		id?: number,
		name: string,
		user_id?: number,
		tag_ids?: number[],
	}

	type DeepRecipient = {
		id: number,
		name: string,
		user?: User,
		tags: DeepTag[],
	}

	type Tag = {
		id?: number,
		name: string,
		user_id?: number,
		parent_id?: number,
	}

	type DeepTag = {
		id: number,
		name: string,
		user: User,
		parent?: Tag,
	}

	type Transaction = {
		id?: number,
		user_id?: number,
		currency_id?: number,
		account_id: number,
		recipient_id: number,
		status: TransactionStatus,
		timestamp: string,
		total_amount: number,
		comment?: string,
		tag_ids?: number[],
		asset?: Asset,
		account?: Account,
		currency?: Currency,
		recipient?: Recipient,
		asset_id?: number,
		positions: Position[],
	}

	type DeepTransaction = {
		id: number,
		status: TransactionStatus,
		timestamp: string,
		total_amount: number,
		comment?: string,
		currency: Currency,
		user: User,
		account: DeepAccount,
		recipient: DeepRecipient,
		tags: DeepTag[],
		asset?: DeepAsset,
		positions: Position[],
	}

	type Position = {
		id?: number,
		amount: number,
		comment?: string,
		tag_id?: number,
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
		options?: {id?: number, name: string}[],
		sortable?: boolean,
		no_filter?: boolean,
	}

	type TableData = {
		multiSelect: boolean,
		defaultSort: TableSort,
		columns: Column[],
		rows: Row[],
		displaySum?: boolean,
		sumColumn?: number,
		row_count?: number,
		total_amount?: number,
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
		type: "number" | "string" | "tags" | "currency" | "singleTag" | "timestamp" | "account" | "recipient" | "asset" | "positions" | "break",
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
		id?: number,
		chart_type: "text" | "pie" | "line",
		title: string,
		text_template?: string,
		date_period: "daily" | "monthly" | "quarterly" | "yearly",
		asset_id?: number,
		filter_collection: string,
		max_items?: number,
		date_range: number,
		top_left_x: number,
		top_left_y: number,
		bottom_right_x: number,
		bottom_right_y: number,
	}

	type QueryParameters = {
		max_results?: number,
		skip_results?: number,
		sort_property?: string,
		sort_direction?: "asc" | "desc",
		filter_id?: number,
		filter_mode_id?: string,
		filter_asset_id?: number,
		filter_mode_asset_id?: string,
		filter_user_id?: number,
		filter_mode_user_id?: string,
		filter_currency_id?: number,
		filter_mode_currency_id?: string,
		filter_account_id?: number,
		filter_mode_account_id?: string,
		filter_recipient_id?: number,
		filter_mode_recipient_id?: string,
		filter_tag_id?: number,
		filter_mode_tag_id?: string,
		filter_total_amount?: number,
		filter_mode_total_amount?: string,
		filter_comment?: string,
		filter_mode_comment?: string,
		filter_time_range_lower?: Date,
		filter_time_range_upper?: Date,
		filter_mode_time_range?: string,
		filter_name?: string,
		filter_mode_name?: string,
		filter_symbol?: string,
		filter_mode_symbol?: string,
		filter_minor_in_major?: number,
		filter_mode_minor_in_major?: string,
		filter_parent_id?: number,
		filter_mode_parent_id?: string,
		filter_balance?: number,
		filter_mode_balance?: string,
		filter_description?: string,
		filter_mode_description?: string,
		filter_amount?: number,
		filter_mode_amount?: string,
		filter_value_per_unit?: number,
		filter_mode_value_per_unit?: string,
	}
}