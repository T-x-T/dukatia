<template>
	<div>
		<CustomTable
			v-if="table_data.rows.length > 0"
			:tableDataProp="table_data"
		/>
	</div>
</template>

<script lang="ts">
export default {
	data: () => ({
		table_data: {
			multiSelect: false,
			defaultSort: {
				column: 1,
				sort: "desc"
			},
			columns: [
				{
					name: "Currency",
					type: "string",
					no_filter: true,
				},
				{
					name: "Amount",
					type: "number",
					no_filter: true,
				},
			],
			rows: [],
			disable_pagination: true,
			auto_sizing: true,
		} as TableData,
	}),

	props: {
		table: {
			type: Object,
			required: true,
		}
	},

	created() {
		Object.keys(this.table.datasets).forEach((x: any) => {
			this.table_data.rows.push([
				this.table.datasets[x].label,
				this.table.datasets[x].data[this.table.datasets[x].data.length - 1].label,
			]);
		});
	},
}
</script>

<style lang="sass" scoped>
div
	margin-top: 10px
	height: 100%
</style>