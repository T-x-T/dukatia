<template>
	<div>
		<button class="green" @click="newRecipient">Add</button>
		<CustomTable
			:tableDataProp="tableData"
			v-on:rowClick="rowClick"
		/>
	</div>
</template>

<script lang="ts" setup>
const recipients = (await useFetch("/api/v1/recipients/all")).data.value as Recipient[];
const tags = (await useFetch("/api/v1/tags/all")).data.value as Tag[];

const tableData: TableData = {
	multiSelect: false,
	defaultSort: {
		column: 0,
		sort: "asc"
	},
	columns: [
		{name: "ID", type: "number"},
		{name: "Name", type: "string"},
		{name: "Tags", type: "choice", options: [...new Set(tags.map(x => x.name))]}
	],
	rows: recipients.map(x => ([
		x.id,
		x.name,
		tags.filter(y => x.tag_ids?.includes(y.id ? y.id : -1)).map(y => y.name).join(", ")
	]))
};

async function rowClick(row: Row) {
	await useRouter().push(`/recipients/${row[0]}`);
};

async function newRecipient() {
	await useRouter().push("/recipients/new");
};
</script>