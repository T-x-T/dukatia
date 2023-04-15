<template>
	<div>
		<button class="green" @click="newAsset">Add</button>
		<CustomTable
			v-if="tableData"
			:tableData="tableData"
			v-on:rowClick="rowClick"
		/>
	</div>
</template>

<script>
export default {
	data: () => ({
		tableData: null
	}),

	async fetch() {
		await this.updateAssets();
	},

	methods: {
		async updateAssets() {
			await this.$store.dispatch("fetchAssets");
			const assetsForDisplay = this.$store.state.assets.map(x => {
				x.currency = this.$store.state.currencies.filter(c => c.id == x.currency_id)[0];
				return x;
			})

			this.tableData = {
				multiSelect: false,
				displaySum: true,
				sumColumn: 5,
				defaultSort: {
					column: 5,
					sort: "desc"
				},
				columns: [
					{name: "ID", type: "number"},
					{name: "Name", type: "string"},
					{name: "Description", type: "string"},
					{name: "Amount", type: "number"},
					{name: "Value per Unit", type: "number"},
					{name: "Total value", type: "number"},
					{name: "Tags", type: "choice", options: [...new Set(this.$store.state.tags.map(x => x.name))]}
				],
				rows: assetsForDisplay.map(x => ([
					x.id,
					x.name,
					x.description,
					Math.round(x.amount * 10000 + Number.EPSILON) / 10000,
					`${x.value_per_unit / x.currency.minor_in_mayor}${x.currency.symbol}`,
					`${Math.round(((x.amount * x.value_per_unit) / x.currency.minor_in_mayor) * 100 + Number.EPSILON) / 100}${x.currency.symbol}`,
					this.$store.state.tags.filter(y => x.tag_ids?.includes(y.id)).map(y => y.name).join(", ")
				]))
			}
		},

		async rowClick(row) {
			await useRouter().push(`/assets/${row[0]}`);
		},

		async newAsset() {
			await useRouter().push("/assets/new");
		}
	}
}
</script>