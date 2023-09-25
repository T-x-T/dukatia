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
			})
    }
  }
})