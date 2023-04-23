<template>
	<div id="main">
		<AssetDetails
			v-if="Object.keys(assetData).length > 0"
			:propAsset="assetData"
			v-on:back="useRouter().push('/assets')"
		/>
	</div>
</template>

<script lang="ts">
export default {
	data: () => ({
		assetData: {} as Asset
	}),

	async created() {
		if(useRoute().path.split("/")[2] == "new") {
			this.assetData = this.$detailPageConfig().asset.defaultData as Asset;
		} else {
			const id = Number(useRoute().path.split("/")[2]);
			this.assetData = await $fetch(`/api/v1/assets/${id}`);
		}
	}
}
</script>