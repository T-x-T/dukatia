<template>
	<div id="main">
		<TagDetails
			v-if="tagData"
			:tag="tagData"
			v-on:back="$router.push('/tags')"
		/>
	</div>
</template>

<script>
export default {
	data: () => ({
		tagData: null
	}),

	async created() {
		const tags = (await useFetch("/api/v1/tags/all")).data.value;
		if(this.$route.path.split("/")[2] == "new") {
			this.tagData = {
				id: "",
				name: ""
			};
		} else {
			const id = Number(this.$route.path.split("/")[2]);
			const tag = tags.filter(x => x.id == id)[0];
			this.tagData = {...tag};
		}
	}
}
</script>