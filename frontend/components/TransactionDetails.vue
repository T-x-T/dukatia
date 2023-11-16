<template>
	<div id="wrapper">
		<TransactionForm
			v-if="Object.keys(transaction).length > 0"
			:data="transaction"
			:default_data="default_transaction"
			@back="$emit('back')"
			@data_saved="async () => {await update(); $emit('updateData');}"
		/>
	</div>
</template>

<script lang="ts">
export default {
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
			transaction.asset_id = transaction.asset ? transaction.asset.id : transaction.asset_id ? transaction.asset_id : undefined;
		}
	}
}
</script>