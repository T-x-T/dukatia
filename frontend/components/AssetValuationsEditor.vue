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
		this.assetValuations = (await this.$axios.$get(`/api/v1/assets/${this.assetId}/valuation_history`)).map(x => {
			x.value_per_unit /= 100;  //TODO: use minor in mayor
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
			try {
				await this.$axios.$post(`/api/v1/assets/${this.assetId}/valuation_history`, this.assetValuations.filter(x => !x.deleted).map(x => ({amount: Number(x.amount), value_per_unit: Math.round(Number(x.value_per_unit) * 100), timestamp: x.timestamp}))); //TODO: use minor in mayor
			} catch(e) {
				console.error(e.response);
				window.alert(e.response.data);
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