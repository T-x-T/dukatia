<template>
	<div>
		<button @click="$emit('back')">Back</button>
		<div id="grid">
			<div id="wrapper" class="gridItem">
				<div id="inner">
					<h3>Asset data</h3>
					<DetailsForm
						:config="config"
						v-on:back="$emit('back')"
						v-on:updateData="reload"
					/>
				</div>
			</div>
			<div v-if="asset.id !== '' && renderCharts" id="wrapper" class="gridItem">
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
							<option v-for="(account, index) in $store.state.accounts.filter(x => x.default_currency_id === asset.currency_id)" :key="index" :value="account.id">{{account.name}}</option>
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
			<div v-if="asset.id !== '' && renderCharts" id="wrapper" class="gridItem">
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
			<div v-if="asset.id !== '' && renderCharts" class="gridItem chart">
				<CustomLineChart
					:api_path="`/api/v1/reports/value_per_unit_over_time_for_asset/${asset.id}`"
					title="Value over time per single unit"
					type="simple_monetary"
					:no_controls="true"
				/>
			</div>
			<div v-if="asset.id !== '' && renderCharts" class="gridItem chart">
				<CustomLineChart
					:api_path="`/api/v1/reports/amount_over_time_for_asset/${asset.id}`"
					title="Amount over time"
					type="simple"
					:no_controls="true"
				/>
			</div>
		</div>
	</div>
</template>

<script>
export default {
	data: () => ({
		asset: null,
		config: {},
		transactionData: {},
		updateData: {},
		renderCharts: true
	}),

	props: {
		propAsset: Object
	},

	created() {
		this.update();
	},

	methods: {
		update() {
			this.asset = this.asset ? this.asset : this.propAsset.name === null ? {...this.propAsset, id: ""} : this.propAsset;

			this.config = {
				...this.$detailPageConfig.asset,
				data: {
					...this.asset,
					value_per_unit: this.asset.value_per_unit / 100, //TODO: use minor_in_mayor
				},
			};
			this.transactionData = {
				amount: 0,
				value_per_unit: this.asset.value_per_unit / 100, //TODO: use minor_in_mayor
				timestamp: new Date(Date.now() - new Date().getTimezoneOffset() * 60000).toISOString().slice(0, -8),
				account_id: 0,
				cost: 0
			};

			this.updateData = {
				amount: this.asset.amount,
				value_per_unit: this.asset.value_per_unit / 100, //TODO: use minor_in_mayor,
				timestamp: new Date(Date.now() - new Date().getTimezoneOffset() * 60000).toISOString().slice(0, -8)
			};
		},

		async reload(res) {
			await this.$store.dispatch("fetchAssets");
			await this.$store.dispatch("fetchTransactions");
			if (res?.id) this.asset.id = res.id;
			this.asset = this.$store.state.assets.filter(x => x.id == this.asset.id)[0];
			
			this.update();
			this.renderCharts = false;
			this.$nextTick(() => this.renderCharts = true);
		},

		async saveTransaction() {
			try {
				await this.$axios.$post(`/api/v1/assets/${this.asset.id}/valuations`, {
					amount: Number(this.asset.amount) + Number(this.transactionData.amount),
					value_per_unit: Math.round(this.transactionData.value_per_unit * 100), //TODO: use minor_in_mayor
					timestamp: new Date(this.transactionData.timestamp),
					account_id: this.transactionData.account_id,
					cost: Math.round(this.transactionData.cost * 100), //TODO: use minor_in_mayor
					total_value: this.transactionData.total_manually_changed ? Math.round(this.transactionData.total * 100) : null //TODO: use minor_in_mayor
				})
			} catch(e) {
				console.error(e.response);
				window.alert(e.response.data);
				return;
			}

			this.reload();
		},

		updateTransactionTotal() {
			this.transactionData.total_manually_changed = false;
			this.transactionData.total = Math.round(((Number(this.transactionData.amount) * Number(this.transactionData.value_per_unit)) + Number(this.transactionData.cost)) * -100 + Number.EPSILON) / 100;
		},

		async saveUpdate() {
			try {
				await this.$axios.$post(`/api/v1/assets/${this.asset.id}/valuations`, {
					amount: Number(this.updateData.amount),
					value_per_unit: Math.round(this.updateData.value_per_unit * 100), //TODO: use minor_in_mayor
					timestamp: new Date(this.updateData.timestamp)
				})
			} catch(e) {
				console.error(e.response);
				window.alert(e.response.data);
				return;
			}

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

div#wrapper
	display: flex
	align-items: center
	justify-content: center

div.gridItem
	padding: 10px

div.chart
	width: 50vw
	height: 40vh
</style>