<template>
	<div id="main">
		<AssetDetails
			:propAsset="assetData"
			v-on:back="$router.push('/assets')"
		/>
	</div>
</template>

<script>
export default {
	data: () => ({
		assetData: {}
	}),

	async fetch() {
		if(this.$route.path.split("/")[2] == "new") {
			this.assetData = this.$detailPageConfig.asset.defaultData;
		} else {
			const id = Number(this.$route.path.split("/")[2]);
			const assetFromStore = this.$store.state.assets.filter(x => x.id == id)[0];
			this.assetData = {...assetFromStore};
		}
	}
}
</script>