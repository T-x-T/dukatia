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
		tagData: null
	}),

	async created() {
		const tags: any = await $fetch("/api/v1/tags/all");
		
		if(useRoute().path.split("/")[2] == "new") {
			(this as any).tagData = {
				id: "",
				name: ""
			};
		} else {
			const id = Number(useRoute().path.split("/")[2]);
			const tag = tags.filter((x: any) => x.id == id)[0];
			this.tagData = {...tag};
		}
	}
}
</script>