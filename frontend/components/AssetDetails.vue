<template>
	<div id="wrapper">
		<button @click="$emit('back')">Back</button>
		<button @click="showAssetValuationEditor = true" class="mobile_hidden">Edit Asset Valuations</button>
		<div id="grid">
			<div class="gridItem form">
				<h3>Asset data</h3>
				<AssetForm
					:data="asset"
					@back="$emit('back')"
					@data_saved="reload"
				/>
			</div>
				
			<div v-if="asset?.id !== undefined && renderCharts" class="gridItem form">
				<AssetBuySellForm 
					:asset="asset"
					@saved="reload"
				/>
			</div>
			<div v-if="asset?.id !== undefined && renderCharts" class="gridItem form">
				<AssetUpdateValuationForm
					:asset="asset"
					@saved="reload"
				/>
			</div>
			<div v-if="asset?.id !== undefined && renderCharts" class="gridItem chart">
				<ChartLine
					:line="asset_total_value_chart"
				/>
			</div>
			<div v-if="asset?.id !== undefined && renderCharts" class="gridItem chart">
				<ChartLine
					:line="asset_single_value_chart"
				/>
			</div>
			<div v-if="asset?.id !== undefined && renderCharts" class="gridItem chart">
				<ChartLine
					:line="asset_amount_chart"
				/>
			</div>
		</div>

		<div v-if="showAssetValuationEditor">
			<Popup @close="closeAssetValuationEditor">
				<AssetValuationsEditor
					v-if="asset && Number.isInteger(asset.id)"
					:assetId="asset.id"
					@close="closeAssetValuationEditor"
				/>
			</Popup>
		</div>
	</div>
</template>

<script lang="ts">
export default {
	data: () => ({
		asset: {} as Asset | undefined,
		renderCharts: false,
		showAssetValuationEditor: false,
		asset_total_value_chart: {} as ChartOptions,
		asset_single_value_chart: {} as ChartOptions,
		asset_amount_chart: {} as ChartOptions,
	}),

	emits: ["back"],

	props: {
		propAsset: {
			type: Object as PropType<Asset>,
			required: false,
		}
	},

	async created() {
		await this.update();
	},

	methods: {
		async update() {			
			this.asset = this.asset && Object.keys(this.asset).length > 0 ? this.asset : this.propAsset;

			if(this.asset && Object.keys(this.asset).length > 0) {
				if(this.asset.value_per_unit === undefined) this.asset.value_per_unit = {major: 0, minor: 0, minor_in_major: 100, symbol: "€"};
				
				this.asset_total_value_chart = (await $fetch(`/api/v1/charts/line/asset_total_value/data?asset_id=${this.asset.id}`)).line;
				this.asset_single_value_chart = (await $fetch(`/api/v1/charts/line/asset_single_value/data?asset_id=${this.asset.id}`)).line;
				this.asset_amount_chart = (await $fetch(`/api/v1/charts/line/asset_amount/data?asset_id=${this.asset.id}`)).line;

				this.renderCharts = true;
			}
		},

		async reload(res?: any) {
			if(this.asset && Object.keys(this.asset).length > 0) {
				console.error("this.asset isnt defined in AssetDetails.vue reload method");
				return;
			}

			if (res?.id) (this.asset as Asset).id = res.id;
			this.asset = await $fetch(`/api/v1/assets/${(this.asset as Asset).id}`);
			useRouter().push(`/assets/${(this.asset as Asset).id}`);
			
			await this.update();
			this.renderCharts = false;
			this.$nextTick(() => this.renderCharts = true);
		},

		closeAssetValuationEditor() {
			this.showAssetValuationEditor = false;
			this.reload();
		}
	}
}
</script>

<style lang="sass" scoped>
div#wrapper
	margin: 10px

div#grid
	display: flex
	width: 100%
	justify-content: flex-start
	align-items: flex-start
	align-content: flex-start
	gap: 10px
	flex-wrap: wrap

div.gridItem
	padding: 10px

div.chart
	flex-grow: 1
	width: 50vw
	height: 40vh
</style>