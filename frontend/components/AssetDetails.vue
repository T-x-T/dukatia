<template>
	<div id="wrapper">
		<button @click="$emit('back')">Back</button>
		<button @click="showAssetValuationEditor = true" class="mobile_hidden">Edit Asset Valuations</button>
		<div id="grid">
			<div class="gridItem form">
				<h3>Asset data</h3>
				<AssetForm
					v-if="Object.keys(asset).length > 0"
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
				<Chart
					:chart_options="asset_total_value_chart"
				/>
			</div>
			<div v-if="asset?.id !== undefined && renderCharts" class="gridItem chart">
				<Chart
					:chart_options="asset_single_value_chart"
				/>
			</div>
			<div v-if="asset?.id !== undefined && renderCharts" class="gridItem chart">
				<Chart
					:chart_options="asset_amount_chart"
				/>
			</div>
		</div>

		<div v-if="showAssetValuationEditor">
			<Popup @close="closeAssetValuationEditor">
				<AssetValuationsEditor 
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
		asset: {} as Asset,
		renderCharts: false,
		showAssetValuationEditor: false,
		asset_total_value_chart: {} as ChartOptions,
		asset_single_value_chart: {} as ChartOptions,
		asset_amount_chart: {} as ChartOptions,
	}),

	props: {
		propAsset: {
			type: Object as PropType<Asset>,
			required: true,
		}
	},

	async created() {
		await this.update();
	},

	methods: {
		async update() {			
			this.asset = Object.keys(this.asset).length > 0 ? this.asset : this.propAsset;

			if(!this.asset) {
				console.error("this.asset isnt defined!")
				return;
			} else {
				if(this.asset.value_per_unit === undefined) this.asset.value_per_unit = {major: 0, minor: 0, minor_in_major: 100, symbol: "€"};
			}

			this.asset_total_value_chart = await $fetch("/api/v1/charts/7");
			this.asset_total_value_chart.asset_id = this.asset.id;
			this.asset_single_value_chart = await $fetch("/api/v1/charts/8");
			this.asset_single_value_chart.asset_id = this.asset.id;
			this.asset_amount_chart = await $fetch("/api/v1/charts/9");
			this.asset_amount_chart.asset_id = this.asset.id;

			this.renderCharts = true;
		},

		async reload(res?: any) {
			if (res?.id) this.asset.id = res.id;
			this.asset = await $fetch(`/api/v1/assets/${this.asset.id}`);
			
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