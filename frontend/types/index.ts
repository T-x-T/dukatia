export default {};

declare global {
	type Account = {
		id?: string,
		name: string,
		default_currency_id?: string,
		user_id?: number,
		tag_ids?: string[],
		default_currency?: Currency,
		balance?: number,
	}

	type User = {
		id?: number,
		name: string,
		secret?: string,
		superuser: boolean,
		active: boolean,
		last_logon?: string,
	}

	type Asset = {
		id?: string,
		user_id?: number,
		name: string,
		description?: string,
		currency_id: string,
		value_per_unit?: Money,
		amount?: number,
		tag_ids?: string[],
		currency?: Currency,
		total_cost_of_ownership?: TotalCostOfOwnership,
	}

	type TotalCostOfOwnership = {
		total: Money,
		monthly: Money,
		yearly: Money,
	}

	type Currency = {
		id?: string,
		name: string,
		minor_in_major: number,
		symbol: string,
	}

	type Recipient = {
		id?: string,
		name: string,
		user_id?: number,
		tag_ids?: string[],
	}

	type Tag = {
		id?: string,
		name: string,
		user_id?: number,
		parent_id?: string,
	}
	type Transaction = {
		id?: string,
		user_id?: number,
		currency_id?: string,
		account_id?: string,
		recipient_id?: string,
		status: TransactionStatus,
		timestamp: string,
		total_amount: Money,
		comment?: string,
		tag_ids?: string[],
		asset?: Asset,
		account?: Account,
		currency?: Currency,
		recipient?: Recipient,
		asset_id?: string,
		positions: Position[],
		timestamp_string?: string,
	}

	type Position = {
		id?: number,
		amount: Money,
		comment?: string,
		tag_id?: string,
	}

	type Budget = {
		id?: string,
		name: string,
		user_id?: number,
		amount: Money,
		rollover: boolean,
		period: number,
		filter_tag_ids: string[],
		currency_id: string,
		active_from: Date,
		active_to?: Date,
		active_from_string?: string,
		active_to_string?: string,
		used_amount?: Money,
		available_amount?: Money,
		utilization?: number,
	}

	type Money = {
		major: number,
		minor: number,
		minor_in_major: number,
		symbol: string,
		is_negative?: boolean,
	}

	enum TransactionStatus {
		Withheld, Completed
	}

	type SelectData = {
		options: {id: number | string, name: string}[],
		selected?: number[] | string[],
		label: string,
		openTop?: boolean
	}

	type AssetValuation = {
		value_per_unit: Money,
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
		options?: {id?: number | string, name: string}[],
		sortable?: boolean,
		no_filter?: boolean,
		hidden?: boolean,
	}

	type TableData = {
		multiSelect: boolean,
		defaultSort: TableSort,
		columns: Column[],
		rows: Row[],
		row_count?: number,
		total_amount?: number,
		disable_pagination?: boolean,
		auto_sizing?: boolean,
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
		type: "number" | "string" | "tags" | "currency" | "singleTag" | "timestamp" | "account" | "recipient" | "asset" | "positions" | "money" | "boolean" | "choice" | "break",
		disabled?: boolean,
		step?: string,
		initial?: number,
		suffix?: "currencyOfAccountSymbol",
		addNew?: boolean,
		choices?: {value: any, display: string}[],
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
		id?: string,
		user_id: number,
		name: string,
		description?: string,
	}

	type ChartOptions = {
		id?: string,
		chart_type: "table" | "pie" | "line",
		title: string,
		text_template?: string,
		date_period: "daily" | "monthly" | "quarterly" | "yearly",
		asset_id?: string,
		filter_collection: string,
		max_items?: number,
		date_range: number,
		top_left_x: number,
		top_left_y: number,
		bottom_right_x: number,
		bottom_right_y: number,
		disabled?: boolean,
		only_positive: boolean,
		only_negative: boolean,
		dashboard_id?: string,
		start_at_zero?: boolean,
	}

	type QueryParameters = {
		max_results?: number,
		skip_results?: number,
		sort_property?: string,
		sort_direction?: "asc" | "desc",
		filter_id?: number,
		filter_mode_id?: string,
		filter_asset_id?: string,
		filter_mode_asset_id?: string,
		filter_user_id?: number,
		filter_mode_user_id?: string,
		filter_currency_id?: string,
		filter_mode_currency_id?: string,
		filter_account_id?: string,
		filter_mode_account_id?: string,
		filter_recipient_id?: number,
		filter_mode_recipient_id?: string,
		filter_tag_id?: string,
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
		filter_rollover?: boolean,
		filter_mode_rollover?: string,
		filter_filter_tag_id?: string,
		filter_mode_filter_tag_id?: string,
		filter_lower_active_from?: Date,
		filter_upper_active_from?: Date,
		filter_mode_active_from?: string,
		filter_lower_active_to?: Date,
		filter_upper_active_to?: Date,
		filter_mode_active_to?: string,
	}
}