<template>
	<div id="main">
		<div id="table" v-if="mode == 'table'">
			<button class="green" @click="newAsset">Add</button>
			<CustomTable
				:tableData="tableData"
				v-on:rowClick="rowClick"
			/>
		</div>

		<div id="details" v-if="mode == 'details'">
			<AssetDetails
				:propAsset="selectedRow"
				v-on:back="updateAndLoadTable"
			/>
		</div>
	</div>
</template>

<script>
export default {
	data: () => ({
		tableData: {},
		selectedRow: {},
		mode: "table"
	}),

	async fetch() {
		await this.updateAssets();
	},

	methods: {
		async updateAssets() {
			const assetsForDisplay = this.$store.state.assets.map(x => {
				x.currency = this.$store.state.currencies.filter(c => c.id == x.currency_id)[0];
				return x;
			})

			this.tableData = {
				multiSelect: false,
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

		rowClick(row) {
			this.selectedRow = {
				...this.$store.state.assets.filter(x => x.id == row[0])[0],
				timestamp: new Date(Date.now() - new Date().getTimezoneOffset() * 60000).toISOString().slice(0, -8)
			};
			this.mode = "details";
		},

		async newAsset() {
			this.selectedRow = null;
			this.selectedRow = this.$detailPageConfig.asset.defaultData;
			this.mode = "details";
		},

		async updateAndLoadTable() {
			await this.$store.dispatch("fetchAssets");
			await this.$store.dispatch("fetchTransactions");
			await this.updateAssets();
			this.mode = "table";
		}
	}
}
</script>