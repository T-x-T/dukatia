<template>
	<div>
		<button @click="$emit('back')">Back</button>
		<button @click="showAssetValuationEditor = true" class="mobile_hidden">Edit Asset Valuations</button>
		<div id="grid">
			<div class="gridItem form">
				<div id="inner">
					<h3>Asset data</h3>
					<DetailsPage
						v-if="Object.keys(config).length > 0"
						:config="config"
						v-on:back="$emit('back')"
						v-on:updateData="reload"
					/>
				</div>
			</div>
				
			<div v-if="asset?.id !== undefined && renderCharts" class="gridItem form">
				<div id="inner">
					<h3>Buy/Sell with transaction</h3>
					<div id="transactionForm">
						<label for="transactionAmount">Amount change:</label>
						<input type="number" id="transactionAmount" v-model="transactionData.amount" @input="updateTransactionTotal">
						<br>
						<label for="transactionValue">Value per unit:</label>
						<input type="number" id="transactionValue" v-model="transactionData.value_per_unit" @input="updateTransactionTotal">
						<br>
						<label for="transactionCost">Additional cost:</label>
						<input type="number" id="transactionCost" v-model="transactionData.cost" @input="updateTransactionTotal">
						<br>
						<label for="transactionAccount">Account:</label>
						<select id="transactionAccount" v-model="transactionData.account_id">
							<option v-for="(account, index) in accounts.filter(x => x.default_currency_id === asset.currency_id)" :key="index" :value="account.id">{{account.name}}</option>
						</select>
						<br>
						<label for="transactionTimestamp">Timestamp:</label>
						<input type="datetime-local" id="transactionTimestamp" v-model="transactionData.timestamp">
						<br>
						<label for="transactionTotal">Total:</label>
						<input type="number" id="transactionTotal" v-model="transactionData.total" @change="transactionData.total_manually_changed = true">
						<br>
						<button class="green" @click="saveTransaction">Save</button>
					</div>
				</div>
			</div>
			<div v-if="asset?.id !== undefined && renderCharts" class="gridItem form">
				<div id="inner">
					<h3>Update without transaction</h3>
					<div id="updateForm">
						<label for="updateAmount">New amount:</label>
						<input type="number" id="updateAmount" v-model="updateData.amount">
						<br>
						<label for="updateValue">Value per unit:</label>
						<input type="number" id="updateValue" v-model="updateData.value_per_unit">
						<br>
						<label for="updateTimestamp">Timestamp:</label>
						<input type="datetime-local" id="updateTimestamp" v-model="updateData.timestamp">
						<br>
						<button class="green" @click="saveUpdate">Save</button>
					</div>
				</div>
			</div>
			<div v-if="asset?.id !== undefined && renderCharts" class="gridItem chart">
				<Chart
					:chart_options="asset_total_value_chart"
				/>
				<Chart
					:chart_options="asset_single_value_chart"
				/>
				<Chart
					:chart_options="asset_amount_chart"
				/>
			</div>
		</div>

		<div v-if="showAssetValuationEditor">
			<Popup v-on:close="closeAssetValuationEditor">
				<AssetValuationsEditor 
					:assetId="asset.id"
				/>
			</Popup>
		</div>
	</div>
</template>

<script lang="ts">
export default {
	data: () => ({
		asset: {} as Asset,
		config: {} as DetailFormConfig,
		transactionData: {} as {[key: string]: any},
		updateData: {} as {[key: string]: any},
		renderCharts: false,
		showAssetValuationEditor: false,
		assets: [] as Asset[],
		accounts: [] as Account[],
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

	created() {
		this.update();
	},

	methods: {
		async update() {
			try {
				this.assets = await $fetch("/api/v1/assets/all") as Asset[];
				this.accounts = await $fetch("/api/v1/accounts/all");
			} catch(e: any) {
				console.error(e?.data?.data);
				window.alert(e?.data?.data?.error);
			}
			
			this.asset = Object.keys(this.asset).length > 0 ? this.asset : this.propAsset;
			
			const minor_in_major: number = (await $fetch(`/api/v1/currencies/${this.asset.currency_id}`) as Currency).minor_in_major;

			if(!this.asset) {
				console.error("this.asset isnt defined!")
				return;
			} else {
				if(this.asset.value_per_unit === undefined) this.asset.value_per_unit = 0;
			}

			this.config = {
				...this.$detailPageConfig().asset,
				data: {
					...this.asset,
					value_per_unit: this.asset.value_per_unit / minor_in_major,
				},
			};

			this.transactionData = {
				amount: 0,
				value_per_unit: this.asset.value_per_unit / minor_in_major,
				timestamp: new Date(Date.now() - new Date().getTimezoneOffset() * 60000).toISOString().slice(0, -8),
				account_id: 0,
				cost: 0
			};

			this.updateData = {
				amount: this.asset.amount,
				value_per_unit: this.asset.value_per_unit / minor_in_major,
				timestamp: new Date(Date.now() - new Date().getTimezoneOffset() * 60000).toISOString().slice(0, -8)
			};

			this.asset_total_value_chart = await $fetch("/api/v1/charts/7");
			this.asset_total_value_chart.asset_id = this.asset.id;
			this.asset_single_value_chart = await $fetch("/api/v1/charts/8");
			this.asset_single_value_chart.asset_id = this.asset.id;
			this.asset_amount_chart = await $fetch("/api/v1/charts/9");
			this.asset_amount_chart.asset_id = this.asset.id;

			this.renderCharts = true;
		},

		async reload(res?: any) {
			this.assets = await $fetch("/api/v1/assets/all");

			if (res?.id) this.asset.id = res.id;
			this.asset = this.assets.filter(x => x.id == this.asset.id)[0];
			
			await this.update();
			this.renderCharts = false;
			this.$nextTick(() => this.renderCharts = true);
		},

		async saveTransaction() {
			const minor_in_major: number = (await $fetch(`/api/v1/currencies/${this.asset.currency_id}`) as Currency).minor_in_major;

			try {
				await $fetch(`/api/v1/assets/${this.asset.id}/valuations`, {
					method: "POST",
					body: {
						amount_change: Number(this.transactionData.amount),
						value_per_unit: Math.round(this.transactionData.value_per_unit * minor_in_major),
						timestamp: new Date(this.transactionData.timestamp),
						account_id: this.transactionData.account_id,
						cost: Math.round(this.transactionData.cost * minor_in_major),
						total_value: this.transactionData.total_manually_changed ? Math.round(this.transactionData.total * minor_in_major) : null
					}
				})
			} catch(e: any) {
				console.error(e?.data?.data);
				window.alert(e?.data?.data?.error);
				return;
			}

			this.reload();
		},

		updateTransactionTotal() {
			this.transactionData.total_manually_changed = false;
			this.transactionData.total = Math.round(((Number(this.transactionData.amount) * Number(this.transactionData.value_per_unit)) + Number(this.transactionData.cost)) * -100 + Number.EPSILON) / 100;
		},

		async saveUpdate() {
			const minor_in_major: number = (await $fetch(`/api/v1/currencies/${this.asset.currency_id}`) as Currency).minor_in_major;

			try {
				await $fetch(`/api/v1/assets/${this.asset.id}/valuations`, {
					method: "POST",
					body: {
						amount: Number(this.updateData.amount),
						value_per_unit: Math.round(this.updateData.value_per_unit * minor_in_major),
						timestamp: new Date(this.updateData.timestamp)
					}
				})
			} catch(e: any) {
				console.error(e?.data?.data);
				window.alert(e?.data?.data?.error);
				return;
			}

			this.reload();
		},

		closeAssetValuationEditor() {
			this.showAssetValuationEditor = false;
			this.reload();
		}
	}
}
</script>

<style lang="sass" scoped>

div#grid
	display: flex
	width: 100%
	justify-content: flex-start
	align-items: flex-start
	align-content: flex-start
	gap: 10px
	flex-wrap: wrap

div.form
	display: flex
	align-items: center
	justify-content: center
	height: max-content

div.gridItem
	padding: 10px

div.chart
	flex-grow: 1

div.chart
	width: 50vw
	height: 40vh
</style>