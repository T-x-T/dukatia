<template>
	<div>
		<button class="green" @click="newTag()">Add</button>
		<CustomTable
			v-if="tableData"
			:tableData="tableData"
			v-on:rowClick="rowClick"
		/>
	</div>
</template>

<script lang="ts" setup>
const tags = (await useFetch("/api/v1/tags/all")).data.value;

const tableData = {
	multiSelect: false,
	defaultSort: {
		column: 0,
		sort: "asc"
	},
	columns: [
		{name: "ID", type: "number"},
		{name: "Name", type: "string"},
		{name: "Parent", type: "string"}
	],
	rows: (tags as any).map((x: any) => ([
		x.id,
		x.name,
		((tags as any).filter((y: any) => y.id === x.parent_id)[0] as any)?.name
	]))

};

async function rowClick(row: any) {
	await useRouter().push(`/tags/${row[0]}`);
};

async function newTag() {
	await useRouter().push("/tags/new");
};
</script>