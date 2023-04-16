<template>
	<div id="wrapper">
		<DetailsPage
			v-if="config"
			:config="config"
			v-on:back="$emit('back')"
			v-on:updateData="$emit('updateData')"
		/>
	</div>
</template>

<script lang="ts">
export default {
	data: () => ({
		config: null,
		accounts: [],
		currencies: []
	}),

	props: {
		transaction: Object
	},

	async created() {
		this.accounts = await $fetch("/api/v1/accounts/all");
		this.currencies = await $fetch("/api/v1/currencies/all");
		(this as any).transaction.tag_ids = Array.isArray((this as any).transaction.tag_ids) ? [...(this as any).transaction.tag_ids] : [null];
		(this as any).transaction.asset_id = (this as any).transaction.asset?.id;
		
		(this as any).config = {
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
					label: "Asset",
					property: "asset_id",
					type: "asset"
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
			prepareForApi: (x: any) => {
				const account: any = this.accounts.filter((y: any) => y.id == x.account_id)[0];
				const minor_in_mayor = (this as any).currencies.filter((y: any) => y.id == account.default_currency_id)[0].minor_in_mayor;
				return {
					account_id: x.account_id,
					recipient_id: x.recipient_id,
					asset_id: x.asset_id,
					currency_id: x.currency_id,
					status: x.status,
					timestamp: new Date(x.timestamp),
					amount: Math.round(x.amount * minor_in_mayor),
					comment: x.comment,
					tag_ids: Array.isArray(x.tag_ids) && typeof x.tag_ids[0] == "number" ? x.tag_ids : undefined
				};
			},
			defaultData: {
				id: "",
				account_id: 0,
				currency_id: 0,
				recipient_id: 0,
				status: 1,
				timestamp: new Date(Date.now() - new Date().getTimezoneOffset() * 60000).toISOString().slice(0, -8),
				amount: 0,
				comment: "",
				currency: this.currencies.filter((x: any) => x.id == 0)[0],
				tag_ids: []
			},
			deletable: true
		}
	}
}
</script>