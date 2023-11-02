<template>
	<div id="wrapper">
		<DetailsPage
			v-if="Object.keys(config).length > 0"
			:config="config"
			v-on:back="$emit('back')"
			v-on:updateData="async () => {await update(); $emit('updateData');}"
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
		},
		default_transaction: {
			type: Object as PropType<Transaction>,
			required: true,
		}
	},

	async created() {
		await this.update();
	},

	methods: {
		async update() {
			let transaction = this.transaction.id ? await $fetch(`/api/v1/transactions/${this.transaction.id}`) : structuredClone(toRaw(this.default_transaction));
			transaction.tag_ids = Array.isArray(transaction.tag_ids) ? [...transaction.tag_ids] : [];
			transaction.asset_id = transaction.asset?.id;
			transaction.timestamp = new Date(new Date(transaction.timestamp).valueOf() - (new Date(transaction.timestamp).getTimezoneOffset() * 60000)).toISOString().slice(0, -8);
			
			(this as any).config = {};
			this.$nextTick(() => {
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
					data: transaction,
					apiEndpoint: "/api/v1/transactions",
					populateTagsUsingRecipient: true,
					prepareForApi: (x: Transaction) => {
						return {
							account_id: x.account_id,
							recipient_id: x.recipient_id,
							asset_id: x.asset_id,
							currency_id: x.currency_id,
							status: x.status,
							timestamp: new Date(x.timestamp),
							comment: x.comment,
							tag_ids: Array.isArray(x.tag_ids) && typeof x.tag_ids[0] == "number" ? x.tag_ids : undefined,
							positions: x.positions,
						};
					},
					defaultData: this.default_transaction,
					deletable: true
				};
			});
		}
	}
}
</script>