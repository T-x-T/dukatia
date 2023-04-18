<template>
	<div id="main">
		<TagDetails
			v-if="tagData"
			:tag="tagData"
			v-on:back="useRouter().push('/tags')"
		/>
	</div>
</template>

<script lang="ts">
export default {
	data: () => ({
		tagData: null as Tag | null
	}),

	async created() {
		const tags = await $fetch("/api/v1/tags/all") as Tag[];
		
		if(useRoute().path.split("/")[2] == "new") {
			this.tagData = {
				name: ""
			};
		} else {
			const id = Number(useRoute().path.split("/")[2]);
			const tag = tags.filter(x => x.id == id)[0];
			this.tagData = {...tag};
		}
	}
}
</script>