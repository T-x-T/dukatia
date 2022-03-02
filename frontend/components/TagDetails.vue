<template>
	<div id="wrapper">
		<DetailsPage
			:config="config"
			v-on:back="$emit('back')"
		/>
	</div>
</template>

<script>
export default {
	data: () => ({
		config: {}
	}),

	props: {
		tag: Object
	},

	created() {
		this.config = {
			fields: [
				{
					label: "ID",
					property: "id",
					type: "number",
					disabled: true
				},
				{
					label: "Name",
					property: "name",
					type: "string"
				},
				{
					label: "Parent",
					property: "parentId",
					type: "singleTag"
				}
			],
			data: this.tag,
			apiEndpoint: "/api/v1/tags",
			prepareForApi: (x) => ({
				name: x.name,
				parentId: typeof x.parentId == "number" ? x.parentId : undefined
			}),
			defaultData: {
				id: "",
				name: ""
			}
		}
	}
}
</script>