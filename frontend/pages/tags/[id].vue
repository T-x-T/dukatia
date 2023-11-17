<template>
	<div id="main">
		<TagDetails
			v-if="loaded"
			:tag="tagData"
			@back="useRouter().push('/tags')"
		/>
	</div>
</template>

<script lang="ts">
export default {
	data: () => ({
		tagData: {} as Tag,
		loaded: false,
	}),

	async created() {		
		if(useRoute().path.split("/")[2] == "new") {
			this.loaded = true;
		} else {
			const id = Number(useRoute().path.split("/")[2]);
			this.tagData = await $fetch(`/api/v1/tags/${id}`);
			this.loaded = true;
		}
	}
}
</script>