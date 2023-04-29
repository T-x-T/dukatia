<template>
	<div id="main">
		<TagDetails
			v-if="Object.keys(tagData).length > 0"
			:tag="tagData"
			v-on:back="useRouter().push('/tags')"
		/>
	</div>
</template>

<script lang="ts">
export default {
	data: () => ({
		tagData: {} as Tag
	}),

	async created() {		
		if(useRoute().path.split("/")[2] == "new") {
			this.tagData = {
				name: ""
			};
		} else {
			const id = Number(useRoute().path.split("/")[2]);
			this.tagData = await $fetch(`/api/v1/tags/${id}`);
		}
	}
}
</script>