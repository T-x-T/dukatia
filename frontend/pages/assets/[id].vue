<template>
	<div id="main">
		<AssetDetails
			v-if="loaded"
			:propAsset="assetData"
			@back="useRouter().push('/assets')"
		/>
	</div>
</template>

<script lang="ts">
export default {
	data: () => ({
		assetData: {} as Asset,
		loaded: false,
	}),

	async created() {
		if(useRoute().path.split("/")[2] == "new") {
			this.loaded = true;
		} else {
			const id = Number(useRoute().path.split("/")[2]);
			this.assetData = await $fetch(`/api/v1/assets/${id}`);
			this.loaded = true;
		}
	}
}
</script>