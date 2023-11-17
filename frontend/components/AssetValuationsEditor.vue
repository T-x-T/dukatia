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
						<input type="text" v-model="item.amount" @input="update_amount(index)">
					</td>
					<td>
						<InputMoney :initial_value="item.value_per_unit" @update="(new_value: Money) => {item.value_per_unit = new_value; update_value_per_unit(index)}" />
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
		applyUpdatesForwards: false
	}),

	emits: ["close"],

	props: {
		assetId: Number
	},

	async created() {
		this.assetValuations = (await $fetch(`/api/v1/assets/${this.assetId}/valuation_history`) as AssetValuation[]).map(x => {
			x.deleted = false;
			return x;
		});
		this.originalAssetValuations = structuredClone(toRaw(this.assetValuations));
	},

	methods: {
		update_value_per_unit(i: number) {
			if(this.applyUpdatesForwards) {
				const difference_major = Number(this.assetValuations[i].value_per_unit.major) - Number(this.originalAssetValuations[i].value_per_unit.major);
				const difference_minor = Number(this.assetValuations[i].value_per_unit.minor) - Number(this.originalAssetValuations[i].value_per_unit.minor);
				this.assetValuations = this.assetValuations.map((x, j) => {
					if (j > i) {
						x.value_per_unit.major += difference_major;
						x.value_per_unit.minor += difference_minor;
					}
					return x;
				});
			}
			this.originalAssetValuations = structuredClone(toRaw(this.assetValuations).map(x => toRaw(x)));
		},
		
		update_amount(i: number) {
			if(this.applyUpdatesForwards) {
				if(Number.isNaN(Number(this.assetValuations[i].amount))) return;
				const difference = Number(this.assetValuations[i].amount) - Number(this.originalAssetValuations[i].amount);
				this.assetValuations = this.assetValuations.map((x, j) => {
					if (j > i) {
						x.amount += difference;
					}
					return x;
				});
			}
			this.originalAssetValuations = structuredClone(toRaw(this.assetValuations).map(x => toRaw(x)));
		},

		async save() {
			try {
				await $fetch(`/api/v1/assets/${this.assetId}/valuation_history`, {
					method: "POST",
					body: this.assetValuations
						.filter(x => !x.deleted)
						.map(x => ({
							amount: Number(x.amount),
							value_per_unit: x.value_per_unit,
							timestamp: x.timestamp
						}))
				});
				this.$emit("close");
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
	td
		background: red !important
</style>