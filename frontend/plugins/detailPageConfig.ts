export default defineNuxtPlugin(_nuxtApp => {
	return {
    provide: {
      detailPageConfig: () => ({
				account: {
					fields: [
						{
							label: "ID",
							property: "id",
							type: "number",
							disabled: true
						},
						{
							label: "Name",
							property: "name",
							type: "string",
						},
						{
							label: "Currency",
							property: "default_currency_id",
							type: "currency"
						},
						{
							label: "Tags",
							property: "tag_ids",
							type: "tags"
						}
					],
					data: {
						id: "",
						name: "",
						tag_ids: []
					},
					apiEndpoint: "/api/v1/accounts",
					prepareForApi: (x: any) => ({
						name: x.name,
						default_currency_id: x.default_currency_id,
						tag_ids: Array.isArray(x.tag_ids) && typeof x.tag_ids[0] == "number" ? x.tag_ids : undefined
					}),
					defaultData: {
						name: "",
						tag_ids: []
					},
				} as DetailFormConfig,
				recipient: {
					fields: [
						{
							label: "ID",
							property: "id",
							type: "number",
							disabled: true
						},
						{
							label: "Name",
							property: "name",
							type: "string",
						},
						{
							label: "Tags",
							property: "tag_ids",
							type: "tags"
						}
					],
					data: {
						id: "",
						name: "",
						tag_ids: []
					},
					apiEndpoint: "/api/v1/recipients",
					prepareForApi: (x: any) => ({
						id: x.id,
						name: x.name,
						tag_ids: Array.isArray(x.tag_ids) && typeof x.tag_ids[0] == "number" ? x.tag_ids : undefined
					}),
					defaultData: {
						id: "",
						name: "",
						tag_ids: []
					}
				} as DetailFormConfig,
				tags: {
					fields: [
						{
							label: "ID",
							property: "id",
							type: "number",
							disabled: true
						},
						{
							label: "Name",
							property: "name",
							type: "string"
						},
						{
							label: "Parent",
							property: "parent_id",
							type: "singleTag"
						}
					],
					data: {
						id: "",
						name: "",
						parent_id: null
					},
					apiEndpoint: "/api/v1/tags",
					prepareForApi: (x: any) => ({
						name: x.name,
						parent_id: typeof x.parent_id == "number" ? x.parent_id : undefined
					}),
					defaultData: {
						name: "",
						parent_id: null
					},
					deletable: true
				} as DetailFormConfig,
				asset: {
					fields: [
						{
							label: "ID",
							property: "id",
							type: "number",
							disabled: true
						},
						{
							label: "Name",
							property: "name",
							type: "string",
						},
						{
							label: "Description",
							property: "description",
							type: "string",
						},
						{
							label: "Amount",
							property: "amount",
							type: "number",
							disabled: true
						},
						{
							label: "Value per unit",
							property: "value_per_unit",
							type: "money",
							disabled: true
						},
						{
							label: "Currency",
							property: "currency_id",
							type: "currency"
						},
						{
							label: "Tags",
							property: "tag_ids",
							type: "tags"
						},
					],
					data: {
						id: null,
						name: "",
						description: "",
						amount: 0,
						value_per_unit: 0,
						currency_id: 0,
						tag_ids: [],
						timestamp: new Date(),
						account: 0,
						cost: 0
					},
					apiEndpoint: "/api/v1/assets",
					prepareForApi: (x: any) => ({
						name: x.name,
						description: x.description,
						currency_id: x.currency_id, 
						tag_ids: Array.isArray(x.tag_ids) && typeof x.tag_ids[0] == "number" ? x.tag_ids : undefined,
						timestamp: new Date(x.timestamp),
					}),
					defaultData: {
						name: "",
						description: "",
						amount: 0,
						value_per_unit: 0,
						currency_id: 0,
						tag_ids: [],
						timestamp: new Date(Date.now() - new Date().getTimezoneOffset() * 60000).toISOString().slice(0, -8),
						account: 0,
						cost: 0
					},
					deletable: true,
					noSaveAndNew: true,
					noGoBackOnSave: true
				} as DetailFormConfig,
				currency: {
					fields: [
						{
							label: "ID",
							property: "id",
							type: "number",
							disabled: true
						},
						{
							label: "Name",
							property: "name",
							type: "string",
						},
						{
							label: "Minor in major",
							property: "minor_in_major",
							type: "number"
						},
						{
							label: "Symbol",
							property: "symbol",
							type: "string"
						}
					],
					data: {
						id: "",
						name: "",
						minor_in_major: 100,
						symbol: ""
					},
					apiEndpoint: "/api/v1/currencies",
					prepareForApi: (x: any) => ({
						id: x.id,
						name: x.name,
						minor_in_major: Number(x.minor_in_major),
						symbol: x.symbol
					}),
					defaultData: {
						name: "",
						minor_in_major: 100,
						symbol: ""
					}
				} as DetailFormConfig,
				budget: {
					fields: [
						{
							label: "ID",
							property: "id",
							type: "number",
							disabled: true
						},
						{
							label: "Name",
							property: "name",
							type: "string",
						},
						{
							label: "Total Amount",
							property: "amount",
							type: "money",
						},
						{
							label: "Rollover enabled",
							property: "rollover",
							type: "boolean",
						},
						{
							label: "Period",
							property: "period",
							type: "choice",
							choices: [
								{value: 0, display: "Daily"},
								{value: 1, display: "Weekly"},
								{value: 2, display: "Monthly"},
								{value: 3, display: "Quarterly"},
								{value: 4, display: "Yearly"},
							]
						},
						{
							label: "Currency",
							property: "currency_id",
							type: "currency",
						},
						{
							label: "Active From",
							property: "active_from",
							type: "timestamp",
						},
						{
							label: "Active To",
							property: "active_to",
							type: "timestamp",
						},
						{
							label: "Filter Tags",
							property: "filter_tag_ids",
							type: "tags"
						}
					],
					data: {
						name: "",
						amount: {major: 0, minor: 0, minor_in_major: 100, symbol: "€"},
						rollover: false,
						period: 2,
						filter_tag_ids: [],
						currency_id: 0,
						active_from: new Date(),
					},
					apiEndpoint: "/api/v1/budgets",
					prepareForApi: (x: any) => ({
						id: x.id,
						name: x.name,
						amount: x.amount,
						rollover: x.rollover,
						period: x.period,
						filter_tag_ids: x.filter_tag_ids,
						currency_id: x.currency_id,
						active_from: new Date(new Date(x.active_from).valueOf() - new Date(x.active_from).getTimezoneOffset() * 60000).toISOString(),
						active_to: x.active_to ? new Date(new Date(x.active_to).valueOf() - new Date(x.active_to).getTimezoneOffset() * 60000).toISOString() : null,
					}),
					defaultData: {
						name: "",
						amount: {major: 0, minor: 0, minor_in_major: 100, symbol: "€"},
						rollover: false,
						period: 2,
						filter_tag_ids: [],
						currency_id: 0,
						active_from: new Date(),
					}
				} as DetailFormConfig,
			})
    }
  }
})