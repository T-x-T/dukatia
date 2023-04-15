<template>
	<div>
		<button class="green" @click="newRecipient">Add</button>
		<CustomTable
			v-if="tableData"
			:tableData="tableData"
			v-on:rowClick="rowClick"
		/>
	</div>
</template>

<script lang="ts" setup>
const recipients: any = (await useFetch("/api/v1/recipients/all")).data.value;
const tags: any = (await useFetch("/api/v1/tags/all")).data.value;

const tableData = {
	multiSelect: false,
	defaultSort: {
		column: 0,
		sort: "asc"
	},
	columns: [
		{name: "ID", type: "number"},
		{name: "Name", type: "string"},
		{name: "Tags", type: "choice", options: [...new Set(tags.map((x: any) => x.name))]}
	],
	rows: recipients.map((x: any) => ([
		x.id,
		x.name,
		tags.filter((y: any) => x.tag_ids?.includes(y.id)).map((y: any) => y.name).join(", ")
	]))
};

async function rowClick(row: any) {
	await useRouter().push(`/recipients/${row[0]}`);
};

async function newRecipient() {
	await useRouter().push("/recipients/new");
};
</script>