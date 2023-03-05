export default function(context, inject) {
	inject("detailPageConfig", {
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
				default_currency: context.store.state.currencies.filter(x => x.id == 0)[0],
				tag_ids: []
			},
			apiEndpoint: "/api/v1/accounts",
			prepareForApi: (x) => ({
				name: x.name,
				default_currency_id: x.default_currency_id,
				tag_ids: Array.isArray(x.tag_ids) && typeof x.tag_ids[0] == "number" ? x.tag_ids : undefined
			}),
			defaultData: {
				id: "",
				name: "",
				default_currency_id: context.store.state.currencies.filter(x => x.id == 0)[0],
				tag_ids: []
			},
		},
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
			prepareForApi: (x) => ({
				id: x.id,
				name: x.name,
				tag_ids: Array.isArray(x.tag_ids) && typeof x.tag_ids[0] == "number" ? x.tag_ids : undefined
			}),
			defaultData: {
				id: "",
				name: "",
				tag_ids: []
			}
		},
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
			prepareForApi: (x) => ({
				name: x.name,
				parent_id: typeof x.parent_id == "number" ? x.parent_id : undefined
			}),
			defaultData: {
				id: "",
				name: "",
				parent_id: null
			},
			deletable: true
		},
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
					type: "number",
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
				currency_id: context.store.state.currencies.filter(x => x.id == 0)[0],
				tag_ids: [],
				timestamp: new Date(),
				account: 0,
				cost: 0
			},
			apiEndpoint: "/api/v1/assets",
			prepareForApi: (x) => ({
				name: x.name,
				description: x.description,
				currency_id: x.currency_id, 
				tag_ids: Array.isArray(x.tag_ids) && typeof x.tag_ids[0] == "number" ? x.tag_ids : undefined,
				timestamp: new Date(x.timestamp),
			}),
			defaultData: {
				id: "",
				name: null,
				description: "",
				amount: 0,
				value_per_unit: 0,
				currency_id: context.store.state.currencies.filter(x => x.id == 0)[0],
				tag_ids: [],
				timestamp: new Date(Date.now() - new Date().getTimezoneOffset() * 60000).toISOString().slice(0, -8),
				account: 0,
				cost: 0
			},
			deletable: true,
			noSaveAndNew: true,
			noGoBackOnSave: true
		},
	})
}