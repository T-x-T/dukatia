<template>
	<div id="wrapper">
		<DetailsPage
			v-if="config"
			:config="config"
			v-on:back="$emit('back')"
		/>
	</div>
</template>

<script lang="ts">

export default {
	data: () => ({
		config: null
	}),

	props: {
		account: Object
	},

	async created() {
		const transactions: any = await $fetch("/api/v1/transactions/all");
		const accounts: any = await $fetch("/api/v1/accounts/all");
		const currencies: any = await $fetch("/api/v1/currencies/all");
		const recipients: any = await $fetch("/api/v1/recipients/all");
		const tags: any = await $fetch("/api/v1/tags/all");

		(this as any).account.tag_ids = Array.isArray((this as any).account.tag_ids) ? [...(this as any).account.tag_ids] : [null];

		const transactionsForDisplay = transactions.filter((x: any) => x.account_id == (this as any).account.id).map((x: any) => {
			x.account = accounts.filter((a: any) => a.id == x.account_id)[0];
			x.currency = currencies.filter((c: any) => c.id == x.currency_id)[0];
			x.recipient = recipients.filter((r: any) => r.id == x.recipient_id)[0];
			return x;
		});

		(this as any).config = {
			...this.$detailPageConfig().account,
			data: this.account,
			resetdefault_currency_id: true,
			tableData : {
				multiSelect: false,
				defaultSort: {
					column: 0,
					sort: "asc"
				},
				columns: [
					{name: "ID", type: "number"},
					{name: "Account", type: "choice", options: [...new Set(accounts.map((x: any) => x.name))]},
					{name: "Recipient", type: "choice", options: [...new Set(recipients.map((x: any) => x.name))]},
					{name: "Timestamp", type: "date"},
					{name: "Amount", type: "number"},
					{name: "Comment", type: "string"},
					{name: "Tags", type: "choice", options: [...new Set(tags.map((x: any) => x.name))]}
				],
				rows: transactionsForDisplay.map((x: any) => ([
					x.id,
					x.account.name,
					x.recipient.name,
					new Date(x.timestamp).toISOString().substring(0, 10),
					`${x.amount / x.currency.minor_in_mayor}${x.currency.symbol}`,
					x.comment,
					tags.filter((y: any) => x.tag_ids?.includes(y.id)).map((y: any) => y.name).join(", ")
				]))
			}
		}
	}
}
</script>