<template>
	<div id="wrapper">
		<DetailsPage
			:config="config"
			v-on:back="$emit('back')"
			v-on:updateData="$emit('updateData')"
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
		this.transaction.tag_ids = Array.isArray(this.transaction.tag_ids) ? [...this.transaction.tag_ids] : [null]

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
					property: "account_id",
					type: "account",
					addNew: true
				},
				{
					label: "Recipient",
					property: "recipient_id",
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
					propety: "tag_ids",
					type: "tags",
					addNew: true
				}
			],
			data: this.transaction,
			apiEndpoint: "/api/v1/transactions",
			populateTagsUsingRecipient: true,
			prepareForApi: (x) => ({
				account_id: x.account_id,
				recipient_id: x.recipient_id,
				currency_id: x.currency_id,
				status: x.status,
				timestamp: new Date(x.timestamp),
				amount: Math.round(x.amount * 100),
				comment: x.comment,
				tag_ids: Array.isArray(x.tag_ids) && typeof x.tag_ids[0] == "number" ? x.tag_ids : undefined
			}),
			defaultData: {
				id: "",
				account_id: 0,
				currency_id: 0,
				recipient_id: 0,
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