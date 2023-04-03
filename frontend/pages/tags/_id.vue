<template>
	<div id="main">
		<TagDetails
			:tag="tagData"
			v-on:back="$router.push('/tags')"
		/>
	</div>
</template>

<script>
export default {
	data: () => ({
		tagData: {}
	}),

	async fetch() {
		if(this.$route.path.split("/")[2] == "new") {
			this.tagData = {
				id: "",
				name: ""
			};
		} else {
			const id = Number(this.$route.path.split("/")[2]);
			const tagFromStore = this.$store.state.tags.filter(x => x.id == id)[0];
			this.tagData = {...tagFromStore};
		}
	}
}
</script>