<template>
	<div>
		<button class="green" @click="newAsset">Add</button>
		<CustomTable
			:tableDataProp="tableData"
			v-on:rowClick="rowClick"
		/>
	</div>
</template>

<script lang="ts" setup>
const assets = (await useFetch("/api/v1/assets/all")).data.value as Asset[];
const currencies = (await useFetch("/api/v1/currencies/all")).data.value as Currency[];
const tags = (await useFetch("/api/v1/tags/all")).data.value as Tag[];

const assetsForDisplay = assets.map(x => {
	x.currency = currencies.filter(c => c.id == x.currency_id)[0];
	return x;
});

const tableData: TableData = {
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
		{name: "Tags", type: "choice", options: [...new Set(tags.map(x => x.name))]}
	],
	rows: assetsForDisplay.map(x => {
		x.amount = x.amount ? x.amount : 0;
		x.value_per_unit = x.value_per_unit ? x.value_per_unit : 0;
		x.currency = x.currency ? x.currency : {name: "Euro", minor_in_mayor: 100, symbol: "€"};

		return [
			x.id,
			x.name,
			x.description,
			Math.round(x.amount * 10000 + Number.EPSILON) / 10000,
			`${x.value_per_unit / x.currency.minor_in_mayor}${x.currency.symbol}`,
			`${Math.round(((x.amount * x.value_per_unit) / x.currency.minor_in_mayor) * 100 + Number.EPSILON) / 100}${x.currency.symbol}`,
			tags.filter(y => x.tag_ids?.includes(y.id ? y.id : -1)).map(y => y.name).join(", ")
		];
	})
};

async function rowClick(row: Row) {
	await useRouter().push(`/assets/${row[0]}`);
};

async function newAsset() {
	await useRouter().push("/assets/new");
};
</script>