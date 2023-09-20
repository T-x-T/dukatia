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

<script lang="ts">
export default {
	data: () => ({
		assetValuations: [] as AssetValuation[],
		originalAssetValuations: [] as AssetValuation[],
		applyUpdatesForwards: true
	}),

	props: {
		assetId: Number
	},

	async created() {
		const asset: Asset = await $fetch(`/api/v1/assets/${this.assetId}`);
		const minor_in_major: number = (await $fetch(`/api/v1/currencies/${asset.currency_id}`) as Currency).minor_in_major;

		this.assetValuations = (await $fetch(`/api/v1/assets/${this.assetId}/valuation_history`) as AssetValuation[]).map(x => {
			x.value_per_unit /= minor_in_major;
			x.deleted = false;
			return x;
		});
		this.originalAssetValuations = structuredClone(toRaw(this.assetValuations));
	},

	methods: {
		update(i: number, prop: "value_per_unit" | "amount" | "timestamp" | "deleted") {
			if(this.applyUpdatesForwards) {
				if(Number.isNaN(Number(this.assetValuations[i][prop])) || prop == "timestamp" || prop == "deleted") return;
				const difference = Number(this.assetValuations[i][prop]) - Number(this.originalAssetValuations[i][prop]);
				this.assetValuations = this.assetValuations.map((x, j) => {
					if (j > i) {
						x[prop] += difference;
					}
					return x;
				});
			}
			this.originalAssetValuations = structuredClone(toRaw(this.assetValuations).map(x => toRaw(x)));
		},

		async save() {
			const asset: Asset = await $fetch(`/api/v1/assets/${this.assetId}`);
			const minor_in_major: number = (await $fetch(`/api/v1/currencies/${asset.currency_id}`) as Currency).minor_in_major;

			try {
				await $fetch(`/api/v1/assets/${this.assetId}/valuation_history`, {
					method: "POST",
					body: this.assetValuations
						.filter(x => !x.deleted)
						.map(x => ({
							amount: Number(x.amount),
							value_per_unit: Math.round(Number(x.value_per_unit) * minor_in_major),
							timestamp: x.timestamp
						}))
				});
			} catch(e: any) {
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