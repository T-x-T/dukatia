<template>
	<div id="wrapper">
		<DetailsPage
			v-if="Object.keys(config).length > 0"
			:config="config"
			v-on:back="$emit('back')"
			v-on:updateData="$emit('updateData')"
		/>
	</div>
</template>

<script lang="ts">
export default {
	data: () => ({
		config: {} as DetailFormConfig,
	}),

	props: {
		transaction: {
			type: Object as PropType<Transaction>,
			required: true,
		}
	},

	async created() {
		const account: Account = await $fetch(`/api/v1/accounts/${this.transaction.account_id}`);
		const minor_in_major = (await $fetch(`/api/v1/currencies/${account.default_currency_id}`) as Currency).minor_in_major;
		
		this.transaction.tag_ids = Array.isArray(this.transaction.tag_ids) ? [...this.transaction.tag_ids] : [];
		this.transaction.asset_id = this.transaction.asset?.id;
		this.transaction.positions = this.transaction.positions.map(p => ({...p, amount: p.amount / minor_in_major}));
		
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
					label: "Comment",
					property: "comment",
					type: "string"
				},
				{
					label: "Positions",
					property: "positions",
					type: "positions",
				},
				{
					label: "Tags",
					property: "tag_ids",
					type: "tags",
					addNew: true
				}
			],
			data: this.transaction,
			apiEndpoint: "/api/v1/transactions",
			populateTagsUsingRecipient: true,
			prepareForApi: async (x: Transaction) => {
				return {
					account_id: x.account_id,
					recipient_id: x.recipient_id,
					asset_id: x.asset_id,
					currency_id: x.currency_id,
					status: x.status,
					timestamp: new Date(x.timestamp),
					comment: x.comment,
					tag_ids: Array.isArray(x.tag_ids) && typeof x.tag_ids[0] == "number" ? x.tag_ids : undefined,
					positions: x.positions.map(p => ({...p, amount: Number(Number(p.amount * minor_in_major).toFixed(0))})),
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
				currency: (await $fetch("/api/v1/currencies/0") as Currency),
				tag_ids: [],
				positions: [
					{
						amount: 0,
						comment: "",
					}
				],
			},
			deletable: true
		}
	}
}
</script>