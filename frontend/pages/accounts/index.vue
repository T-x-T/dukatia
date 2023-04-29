<template>
	<div>
		<button class="green" @click="newAccount">Add</button>
		<CustomTable
			:tableData="tableData"
			v-on:rowClick="rowClick"
		/>
	</div>
</template>

<script lang="ts" setup>
const accounts = (await useFetch("/api/v1/accounts/all")).data.value as Account[];
const currencies = (await useFetch("/api/v1/currencies/all")).data.value as Currency[];
const tags = (await useFetch("/api/v1/tags/all")).data.value as Tag[];
const transactions = (await useFetch("/api/v1/transactions/all")).data.value as Transaction[];

const tableData: TableData = {
	multiSelect: false,
	defaultSort: {
		column: 0,
		sort: "asc"
	},
	columns: [
		{name: "ID", type: "number"},
		{name: "Name", type: "string"},
		{name: "Currency", type: "choice", options: currencies.map(x => x.name)},
		{name: "Tags", type: "choice", options: [...new Set(tags.map(x => x.name))]},
		{name: "Balance", type: "number"}
	],
	rows: accounts.map(x => ([
		x.id,
		x.name,
		currencies.filter(c => c.id == x.default_currency_id)[0].name,
		tags.filter(t => x.tag_ids?.includes(t.id ? t.id : -1)).map(t => t.name).join(", "),
		transactions
			.filter(t => t.account_id == x.id)
			.reduce((a, b) => a + b.amount, 0) 
			/ currencies.filter(c => c.id == x.default_currency_id)[0].minor_in_mayor 
			+ currencies.filter(c => c.id == x.default_currency_id)[0].symbol
	]))
};

async function rowClick(row: Row) {
await useRouter().push(`/accounts/${row[0]}`);
};

async function newAccount() {
await useRouter().push("/accounts/new");
};
</script>