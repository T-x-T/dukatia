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
					property: "defaultCurrencyId",
					type: "currency"
				},
				{
					label: "Tags",
					property: "tagIds",
					type: "tags"
				}
			],
			data: {
				id: "",
				name: "",
				defaultCurrency: context.store.state.currencies.filter(x => x.id == 0)[0],
				tagIds: []
			},
			apiEndpoint: "/api/v1/accounts",
			prepareForApi: (x) => ({
				name: x.name,
				defaultCurrency: x.defaultCurrencyId,
				tagIds: Array.isArray(x.tagIds) && typeof x.tagIds[0] == "number" ? x.tagIds : undefined
			}),
			defaultData: {
				id: "",
				name: "",
				defaultCurrency: context.store.state.currencies.filter(x => x.id == 0)[0],
				tagIds: []
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
					property: "tagIds",
					type: "tags"
				}
			],
			data: {
				id: "",
				name: "",
				tagIds: []
			},
			apiEndpoint: "/api/v1/recipients",
			prepareForApi: (x) => ({
				id: x.id,
				name: x.name,
				tagIds: Array.isArray(x.tagIds) && typeof x.tagIds[0] == "number" ? x.tagIds : undefined
			}),
			defaultData: {
				id: "",
				name: "",
				tagIds: []
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
					property: "parentId",
					type: "singleTag"
				}
			],
			data: {
				id: "",
				name: "",
				parentId: null
			},
			apiEndpoint: "/api/v1/tags",
			prepareForApi: (x) => ({
				name: x.name,
				parentId: typeof x.parentId == "number" ? x.parentId : undefined
			}),
			defaultData: {
				id: "",
				name: "",
				parentId: null
			}
		}
	})
}