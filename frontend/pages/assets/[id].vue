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
			this.assetData = {
				name: "",
				description: "",
				amount: 0,
				value_per_unit: {major: 0, minor: 0, minor_in_major: 100, symbol: ""},
				currency_id: 0,
				tag_ids: [],
				user_id: 0
			};
		} else {
			const id = Number(useRoute().path.split("/")[2]);
			this.assetData = await $fetch(`/api/v1/assets/${id}`);
		}
	}
}
</script>