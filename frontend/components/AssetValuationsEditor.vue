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
				<tr v-for="(item, index) in assetValuations" :key="index" :class="(item as any).deleted ? 'deleted' : ''">
					<td>{{(item as any).timestamp}}</td>
					<td>
						<input type="text" v-model="(item as any).amount" @input="update(index, 'amount')">
					</td>
					<td>
						<input type="text" v-model="(item as any).value_per_unit" @input="update(index, 'value_per_unit')">
					</td>
					<td><input type="checkbox" v-model="(item as any).deleted"></td>
				</tr>
			</tbody>
		</table>
		<button class="green" @click="save">Save</button>
	</div>
</template>

<script lang="ts">
export default {
	data: () => ({
		assetValuations: [],
		originalAssetValuations: [],
		applyUpdatesForwards: true,
		assets: [],
		currencies: [],
	}),

	props: {
		assetId: Number
	},

	async created() {
		this.assets = await $fetch("/api/v1/assets/all");
		this.currencies = await $fetch("/api/v1/currencies/all");

		const asset: any = this.assets.filter((x: any) => x.id == this.assetId)[0];
		const minor_in_mayor = (this as any).currencies.filter((x: any) => x.id == asset.currency_id)[0].minor_in_mayor;

		this.assetValuations = (await $fetch(`/api/v1/assets/${this.assetId}/valuation_history`) as any).map((x: any) => {
			x.value_per_unit /= minor_in_mayor;
			x.deleted = false;
			return x;
		});
		this.originalAssetValuations = structuredClone(toRaw(this.assetValuations));
	},

	methods: {
		update(i: number, prop: string) {
			if(this.applyUpdatesForwards) {
				if(Number.isNaN(Number(this.assetValuations[i][prop]))) return;
				const difference = Number(this.assetValuations[i][prop]) - Number(this.originalAssetValuations[i][prop]);
				this.assetValuations = this.assetValuations.map((x, j) => {
					if (j > i) {
						(x[prop] as any) += difference;
					}
					return x;
				});
			}
			this.originalAssetValuations = structuredClone(toRaw(this.assetValuations).map(x => toRaw(x)));
		},

		async save() {
			const asset: any = this.assets.filter((x: any) => x.id == this.assetId)[0];
			const minor_in_mayor = (this as any).currencies.filter((x: any) => x.id == asset.currency_id)[0].minor_in_mayor;

			try {
				await $fetch(`/api/v1/assets/${this.assetId}/valuation_history`, {
					method: "POST",
					body: this.assetValuations
						.filter((x: any) => !x.deleted)
						.map((x: any) => ({
							amount: Number(x.amount),
							value_per_unit: Math.round(Number(x.value_per_unit) * minor_in_mayor),
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