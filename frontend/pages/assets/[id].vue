<template>
	<div id="main">
		<AssetDetails
			:propAsset="assetData"
			v-on:back="useRouter().push('/assets')"
		/>
	</div>
</template>

<script lang="ts">
export default {
	data: () => ({
		assetData: {}
	}),

	async fetch() {
		if(useRoute().path.split("/")[2] == "new") {
			this.assetData = this.$detailPageConfig().asset.defaultData;
		} else {
			const assets: any = await $fetch("/api/v1/assets/all");
			const id = Number(this.$route.path.split("/")[2]);
			const assetFromStore = assets.filter((x: any) => x.id == id)[0];
			this.assetData = {...assetFromStore};
		}
	}
}
</script>