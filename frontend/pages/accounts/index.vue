<template>
	<div>
		<button class="green" @click="newAccount">Add</button>
		<CustomTable
			v-if="tableData"
			:tableData="tableData"
			v-on:rowClick="rowClick"
		/>
	</div>
</template>

<script lang="ts" setup>
const accounts: any = (await useFetch("/api/v1/accounts/all")).data.value;
const currencies: any = (await useFetch("/api/v1/currencies/all")).data.value;
const tags: any = (await useFetch("/api/v1/tags/all")).data.value;
const transactions: any = (await useFetch("/api/v1/transactions/all")).data.value;

const tableData = {
	multiSelect: false,
	defaultSort: {
		column: 0,
		sort: "asc"
	},
	columns: [
		{name: "ID", type: "number"},
		{name: "Name", type: "string"},
		{name: "Currency", type: "choice", options: currencies.map((x: any) => x.name)},
		{name: "Tags", type: "choice", options: [...new Set(tags.map((x: any) => x.name))]},
		{name: "Balance", type: "number"}
	],
	rows: accounts.map((x: any) => ([
		x.id,
		x.name,
		currencies.filter((c: any) => c.id == x.default_currency_id)[0].name,
		tags.filter((t: any) => x.tag_ids?.includes(t.id)).map((t: any) => t.name).join(", "),
		transactions
			.filter((t: any) => t.account_id == x.id)
			.reduce((a: any, b: any) => a + b.amount, 0) 
			/ currencies.filter((c: any) => c.id == x.default_currency_id)[0].minor_in_mayor 
			+ currencies.filter((c: any) => c.id == x.default_currency_id)[0].symbol
	]))
};

async function rowClick(row: any) {
await useRouter().push(`/accounts/${row[0]}`);
};

async function newAccount() {
await useRouter().push("/accounts/new");
};
</script>