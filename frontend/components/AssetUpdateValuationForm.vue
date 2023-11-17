<template>
	<div id="wrapper">
		<h3>Update without transaction</h3>
		<div>
			<label>
				New amount:
				<input type="number" v-model="update_data.amount">
			</label>
			
			<label>
				Value per unit:
				<InputMoney 
					:initial_value="update_data.value_per_unit"
					@update="((new_value: Money) => update_data.value_per_unit = new_value)"
					/>
			</label>
			
			<label>
				Timestamp:
				<input type="datetime-local" v-model="update_data.timestamp">
			</label>
			
			<button class="green" @click="save">Save</button>
		</div>
	</div>
</template>

<script lang="ts">
export default {
	data: () => ({
		update_data: {} as {[key: string]: any},
	}),

	$emits: ["saved"],

	props: {
		asset: {
			type: Object as PropType<Asset>,
			required: true,
		}
	},

	created() {
		this.update_data = {
			amount: this.asset.amount,
			value_per_unit: this.asset.value_per_unit,
			timestamp: new Date(Date.now() - new Date().getTimezoneOffset() * 60000).toISOString().slice(0, -8)
		};
	},

	methods: {
		async save() {
			try {
				await $fetch(`/api/v1/assets/${this.asset.id}/valuations`, {
					method: "POST",
					body: {
						amount: Number(this.update_data.amount),
						value_per_unit: this.update_data.value_per_unit,
						timestamp: new Date(this.update_data.timestamp)
					}
				});
				this.$emit("saved");
			} catch(e: any) {
				console.error(e);
				window.alert(e?.data);
				return;
			}
		},
	},
}
</script>

<style lang="sass" scoped>
div#wrapper
	width: 350px

label
	display: flex
	input, select
		flex-grow: 1
</style>