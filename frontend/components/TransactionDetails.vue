<template>
	<div id="wrapper">
		<DetailsPage
			:config="config"
			v-on:back="$emit('back')"
		/>
	</div>
</template>

<script>
export default {
	data: () => ({
		config: {}
	}),

	props: {
		transaction: Object
	},

	created() {
		this.transaction.tagIds = Array.isArray(this.transaction.tagIds) ? [...this.transaction.tagIds] : [null]

		this.config = {
			fields: [
				{
					label: "ID",
					property: "id",
					type: "number",
					disabled: true
				},
				{
					label: "Account",
					property: "accountId",
					type: "account",
					addNew: true
				},
				{
					label: "Recipient",
					property: "recipientId",
					type: "recipient",
					addNew: true
				},
				{
					label: "Timestamp",
					property: "timestamp",
					type: "timestamp"
				},
				{
					label: "Amount",
					property: "amount",
					type: "number",
					step: "0.01",
					suffix: "currencyOfAccountSymbol"
				},
				{
					label: "Comment",
					property: "comment",
					type: "string"
				},
				{
					label: "Tags",
					propety: "tagIds",
					type: "tags",
					addNew: true
				}
			],
			data: this.transaction,
			apiEndpoint: "/api/v1/transactions",
			prepareForApi: (x) => ({
				accountId: x.accountId,
				recipientId: x.recipientId,
				currencyId: x.currencyId,
				status: x.status,
				timestamp: x.timestamp,
				amount: x.amount * 100,
				comment: x.comment,
				tagIds: Array.isArray(x.tagIds) && typeof x.tagIds[0] == "number" ? x.tagIds : undefined
			}),
			defaultData: {
				id: "",
				accountId: 0,
				currencyId: 0,
				recipientId: 0,
				status: 1,
				timestamp: new Date(Date.now() - new Date().getTimezoneOffset() * 60000).toISOString().slice(0, -8),
				amount: 0,
				comment: "",
				currency: this.$store.state.currencies.filter(x => x.id == 0)[0]
			},
			deletable: true
		}
	}
}
</script>