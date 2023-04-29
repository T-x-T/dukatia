<template>
	<div>
		<button class="green" @click="newCurrency">Add</button>
		<CustomTable
			:tableData="tableData"
			v-on:rowClick="rowClick"
		/>
	</div>
</template>

<script lang="ts" setup>
const currencies = (await useFetch("/api/v1/currencies/all")).data.value as Currency[];

const tableData: TableData = {
	multiSelect: false,
	defaultSort: {
		column: 0,
		sort: "asc"
	},
	columns: [
		{name: "ID", type: "number"},
		{name: "Name", type: "string"},
		{name: "Symbol", type: "string"},
		{name: "Minor in Mayor", type: "number"},
	],
	rows: currencies.map(x => ([
		x.id,
		x.name,
		x.symbol,
		x.minor_in_mayor
	]))
};

async function rowClick(row: Row) {
	await useRouter().push(`/currencies/${row[0]}`);
};

async function newCurrency() {
	await useRouter().push("/currencies/new");
};
</script>
