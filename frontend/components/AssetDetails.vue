<template>
	<div>
		<button @click="$emit('back')">Back</button>
		<button @click="showAssetValuationEditor = true">Edit Asset Valuations</button>
		<div id="grid">
			<div class="gridItem form">
				<div id="inner">
					<h3>Asset data</h3>
					<DetailsForm
						v-if="config"
						:config="config"
						v-on:back="$emit('back')"
						v-on:updateData="reload"
					/>
				</div>
			</div>
				
			<div v-if="(asset as any)?.id !== '' && renderCharts" class="gridItem form">
				<div id="inner">
					<h3>Buy/Sell with transaction</h3>
					<div id="transactionForm">
						<label for="transactionAmount">Amount change:</label>
						<input type="number" id="transactionAmount" v-model="(transactionData as any).amount" @input="updateTransactionTotal">
						<br>
						<label for="transactionValue">Value per unit:</label>
						<input type="number" id="transactionValue" v-model="(transactionData as any).value_per_unit" @input="updateTransactionTotal">
						<br>
						<label for="transactionCost">Additional cost:</label>
						<input type="number" id="transactionCost" v-model="(transactionData as any).cost" @input="updateTransactionTotal">
						<br>
						<label for="transactionAccount">Account:</label>
						<select id="transactionAccount" v-model="(transactionData as any).account_id">
							<option v-for="(account, index) in (accounts as any).filter((x: any) => x.default_currency_id === (asset as any).currency_id)" :key="index" :value="account.id">{{account.name}}</option>
						</select>
						<br>
						<label for="transactionTimestamp">Timestamp:</label>
						<input type="datetime-local" id="transactionTimestamp" v-model="(transactionData as any).timestamp">
						<br>
						<label for="transactionTotal">Total:</label>
						<input type="number" id="transactionTotal" v-model="(transactionData as any).total" @change="(transactionData as any).total_manually_changed = true">
						<br>
						<button class="green" @click="saveTransaction">Save</button>
					</div>
				</div>
			</div>
			<div v-if="(asset as any)?.id !== '' && renderCharts" class="gridItem form">
				<div id="inner">
					<h3>Update without transaction</h3>
					<div id="updateForm">
						<label for="updateAmount">New amount:</label>
						<input type="number" id="updateAmount" v-model="(updateData as any).amount">
						<br>
						<label for="updateValue">Value per unit:</label>
						<input type="number" id="updateValue" v-model="(updateData as any).value_per_unit">
						<br>
						<label for="updateTimestamp">Timestamp:</label>
						<input type="datetime-local" id="updateTimestamp" v-model="(updateData as any).timestamp">
						<br>
						<button class="green" @click="saveUpdate">Save</button>
					</div>
				</div>
			</div>
			<div v-if="(asset as any)?.id !== '' && renderCharts" class="gridItem chart">
				<CustomLineChart
					:api_data="(api_data_total_value as any)"
					title="Total value over time"
					type="simple_monetary"
					:no_controls="true"
					:currency_id="(asset as any).currency_id"
				/>
			</div>
			<div v-if="(asset as any)?.id !== '' && renderCharts" class="gridItem chart">
				<CustomLineChart
					:api_data="(api_data_value as any)"
					title="Value over time per single unit"
					type="simple_monetary"
					:no_controls="true"
					:currency_id="(asset as any).currency_id"
				/>
			</div>
			<div v-if="(asset as any)?.id !== '' && renderCharts" class="gridItem chart">
				<CustomLineChart
					:api_data="(api_data_amount as any)"
					title="Amount over time"
					type="simple"
					:no_controls="true"
				/>
			</div>
		</div>

		<div v-if="showAssetValuationEditor">
			<Popup v-on:close="closeAssetValuationEditor">
				<AssetValuationsEditor 
					:assetId="(asset as any).id"
				/>
			</Popup>
		</div>
	</div>
</template>

<script lang="ts">
export default {
	data: () => ({
		asset: null,
		config: null,
		transactionData: {},
		updateData: {},
		renderCharts: false,
		showAssetValuationEditor: false,
		api_data: null,
		api_data_value: null,
		api_data_amount: null,
		api_data_total_value: null,
		currencies: null,
		assets: null,
		accounts: null,
	}),

	props: {
		propAsset: Object
	},

	created() {
		this.update();
	},

	methods: {
		async update() {
			try {
				this.currencies = await $fetch("/api/v1/currencies/all");
				this.assets = await $fetch("/api/v1/assets/all");
				this.accounts = await $fetch("/api/v1/accounts/all");
			} catch(e: any) {
				console.error(e?.data?.data);
				window.alert(e?.data?.data?.error);
			}

			(this as any).asset = this.asset ? this.asset : (this as any).propAsset.name === null ? {...this.propAsset, id: ""} : this.propAsset;
			
			if((this as any).asset.id !== '') {
				this.api_data = await $fetch(`/api/v1/reports/daily_valuation_of_asset/${(this as any).asset.id}`);
				(this as any).api_data_value = {};
				(this as any).api_data_amount = {};
				(this as any).api_data_total_value = {};
				for (let k in (this as any).api_data) {
					(this as any).api_data_value[k] = (this as any).api_data[k][0];
					(this as any).api_data_amount[k] = (this as any).api_data[k][1];
					(this as any).api_data_total_value[k] = (this as any).api_data[k][1] * (this as any).api_data[k][0];
				}
			}

			const minor_in_mayor = (this as any).currencies.filter((x: any) => x.id == (this as any).asset.currency_id)[0].minor_in_mayor;

			(this as any).config = {
				...this.$detailPageConfig().asset,
				data: {
					...(this as any).asset,
					value_per_unit: (this as any).asset.value_per_unit / minor_in_mayor,
				},
			};
			this.transactionData = {
				amount: 0,
				value_per_unit: (this as any).asset.value_per_unit / minor_in_mayor,
				timestamp: new Date(Date.now() - new Date().getTimezoneOffset() * 60000).toISOString().slice(0, -8),
				account_id: 0,
				cost: 0
			};

			this.updateData = {
				amount: (this as any).asset.amount,
				value_per_unit: (this as any).asset.value_per_unit / minor_in_mayor,
				timestamp: new Date(Date.now() - new Date().getTimezoneOffset() * 60000).toISOString().slice(0, -8)
			};

			this.renderCharts = true;
		},

		async reload(res?: any) {
			this.assets = await $fetch("/api/v1/assets/all");

			if (res?.id) (this as any).asset.id = res.id;
			this.asset = (this as any).assets.filter((x: any) => x.id == (this as any).asset.id)[0];
			
			await this.update();
			this.renderCharts = false;
			this.$nextTick(() => this.renderCharts = true);
		},

		async saveTransaction() {
			const minor_in_mayor = (this as any).currencies.filter((x: any) => x.id == (this as any).asset.currency_id)[0].minor_in_mayor;

			try {
				await $fetch(`/api/v1/assets/${(this as any).asset.id}/valuations`, {
					method: "POST",
					body: {
						amount_change: Number((this as any).transactionData.amount),
						value_per_unit: Math.round((this as any).transactionData.value_per_unit * minor_in_mayor),
						timestamp: new Date((this as any).transactionData.timestamp),
						account_id: (this as any).transactionData.account_id,
						cost: Math.round((this as any).transactionData.cost * minor_in_mayor),
						total_value: (this as any).transactionData.total_manually_changed ? Math.round((this as any).transactionData.total * minor_in_mayor) : null
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
			(this as any).transactionData.total_manually_changed = false;
			(this as any).transactionData.total = Math.round(((Number((this as any).transactionData.amount) * Number((this as any).transactionData.value_per_unit)) + Number((this as any).transactionData.cost)) * -100 + Number.EPSILON) / 100;
		},

		async saveUpdate() {
			const minor_in_mayor = (this as any).currencies.filter((x: any) => x.id == (this as any).asset.currency_id)[0].minor_in_mayor;

			try {
				await $fetch(`/api/v1/assets/${(this as any).asset.id}/valuations`, {
					method: "POST",
					body: {
						amount: Number((this as any).updateData.amount),
						value_per_unit: Math.round((this as any).updateData.value_per_unit * minor_in_mayor),
						timestamp: new Date((this as any).updateData.timestamp)
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