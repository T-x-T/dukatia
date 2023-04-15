<template>
	<div>
		<h3>Update Asset Valuation History</h3>
		<label for="applyUpdatesForwards">Apply updates forwards?</label>
		<input type="checkbox" id="applyUpdatesForwards" v-model="applyUpdatesForwards">
		<table>
			<thead>
				<tr>
					<th>Timestamp</th>
					<th>Amount</th>
					<th>Value per Unit</th>
					<th>Delete</th>
				</tr>
			</thead>
			<tbody>
				<tr v-for="(item, index) in assetValuations" :key="index" :class="item.deleted ? 'deleted' : ''">
					<td>{{item.timestamp}}</td>
					<td>
						<input type="text" v-model="item.amount" @input="update(index, 'amount')">
					</td>
					<td>
						<input type="text" v-model="item.value_per_unit" @input="update(index, 'value_per_unit')">
					</td>
					<td><input type="checkbox" v-model="item.deleted"></td>
				</tr>
			</tbody>
		</table>
		<button class="green" @click="save">Save</button>
	</div>
</template>

<script>
export default {
	data: () => ({
		assetValuations: [],
		originalAssetValuations: [],
		applyUpdatesForwards: true,
	}),

	props: {
		assetId: Number
	},

	async fetch() {
		const asset = this.$store.state.assets.filter(x => x.id == this.assetId)[0];
		const minor_in_mayor = this.$store.state.currencies.filter(x => x.id == asset.currency_id)[0].minor_in_mayor;

		this.assetValuations = (await $fetch(`/api/v1/assets/${this.assetId}/valuation_history`)).map(x => {
			x.value_per_unit /= minor_in_mayor;
			x.deleted = false;
			return x;
		});
		this.originalAssetValuations = structuredClone(this.assetValuations);
	},

	methods: {
		update(i, prop) {
			if(this.applyUpdatesForwards) {
				if(Number.isNaN(Number(this.assetValuations[i][prop]))) return;
				const difference = Number(this.assetValuations[i][prop]) - Number(this.originalAssetValuations[i][prop]);
				this.assetValuations = this.assetValuations.map((x, j) => {
					if (j > i) {
						x[prop] += difference;
					}
					return x;
				});
			}
			this.originalAssetValuations = structuredClone(this.assetValuations);
		},

		async save() {
			const asset = this.$store.state.assets.filter(x => x.id == this.assetId)[0];
			const minor_in_mayor = this.$store.state.currencies.filter(x => x.id == asset.currency_id)[0].minor_in_mayor;

			try {
				await $fetch(`/api/v1/assets/${this.assetId}/valuation_history`, {
					method: "POST",
					body: this.assetValuations
						.filter(x => !x.deleted)
						.map(x => ({
							amount: Number(x.amount),
							value_per_unit: Math.round(Number(x.value_per_unit) * minor_in_mayor),
							timestamp: x.timestamp
						}))
				});
			} catch(e) {
				console.error(e?.data?.data);
				window.alert(e?.data?.data?.error);
				return;
			}
		}
	}
}
</script>

<style lang="sass" scoped>
.deleted
	color: red
</style>