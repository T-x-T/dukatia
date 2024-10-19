<template>
	<div id="wrapper">
		<TransactionForm
			v-if="Object.keys(transaction).length > 0"
			:data="transaction"
			:default_data="default_transaction"
			@back="$emit('back')"
			@data_saved="$emit('updateData')"
		/>
	</div>
</template>

<script lang="ts">
export default {
	emits: ["back", "updateData"],

	data: () => ({
		transaction: {} as Transaction,
	}),

	props: {
		prop_transaction: {
			type: Object as PropType<Transaction>,
			required: true,
		},
		default_transaction: {
			type: Object as PropType<Transaction>,
			required: true,
		}
	},

	async created() {
		this.transaction = this.prop_transaction;
		this.transaction.asset_id = this.transaction.asset?.id;
		this.transaction.timestamp = new Date(new Date(this.transaction.timestamp).valueOf() - (new Date(this.transaction.timestamp).getTimezoneOffset() * 60000)).toISOString();
	},
}
</script>