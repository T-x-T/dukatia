<template>
	<div id="wrapper">
		<h3>Buy/Sell with transaction</h3>
		<div>
			<label>
				Amount change:
				<input type="number" v-model="transaction_data.amount" @input="update_transaction_total">
			</label>

			<label>
				Value per unit:
				<InputMoney
					v-if="transaction_data.value_per_unit && Object.keys(transaction_data.value_per_unit).length > 0"
					:initial_value="transaction_data.value_per_unit" 
					@update="((new_value: Money) => {transaction_data.value_per_unit = new_value; update_transaction_total()})"
				/>
			</label>

			<label>
				Additional cost:
				<InputMoney
					v-if="transaction_data.cost && Object.keys(transaction_data.cost).length > 0"
					:initial_value="transaction_data.cost" 
					@update="((new_value: Money) => {transaction_data.cost = new_value; update_transaction_total()})"
				/>
			</label>
			
			<label>
				Account:
				<select v-model="transaction_data.account_id">
					<option v-for="(account, index) in accounts.filter(x => x.default_currency_id === asset.currency_id)" :key="index" :value="account.id">{{account.name}}</option>
				</select>
			</label>
			
			<label>
				Recipient:
				<select v-model="transaction_data.recipient_id">
					<option v-for="(recipient, index) in recipients" :key="index" :value="recipient.id">{{recipient.name}}</option>
				</select>
			</label>
			
			<label>
				Timestamp:
				<input type="datetime-local" v-model="transaction_data.timestamp">
			</label>
			
			<label>
				Total:
				<InputMoney
				v-if="transaction_data.total && Object.keys(transaction_data.total).length > 0"
					:initial_value="transaction_data.total"
					@update="((new_value: Money) => {transaction_data.total_manually_changed = true; transaction_data.total = new_value; update_transaction_total()})"
					/>
			</label>
			
			<button class="green" @click="save">Save</button>
		</div>
	</div>
</template>

<script lang="ts">
export default {
	data: () => ({
		transaction_data: {} as {[key: string]: any},
		accounts: [] as Account[],
		recipients: [] as Recipient[],
	}),

	$emits: ["saved"],

	props: {
		asset: {
			type: Object as PropType<Asset>,
			required: true,
		}
	},

	async created() {
		this.accounts = await $fetch("/api/v1/accounts/all");
		this.recipients = await $fetch("/api/v1/recipients/all");
		this.transaction_data = {
			amount: 0,
			value_per_unit: this.asset.value_per_unit,
			timestamp: new Date(Date.now() - new Date().getTimezoneOffset() * 60000).toISOString().slice(0, -8),
			account_id: 0,
			cost: {major: 0, minor: 0, minor_in_major: this.asset.value_per_unit?.minor_in_major, symbol: this.asset.value_per_unit?.symbol},
			total: {major: 0, minor: 0, minor_in_major: this.asset.value_per_unit?.minor_in_major, symbol: this.asset.value_per_unit?.symbol},
		};
	},

	methods: {
		async save() {
			try {
				await $fetch(`/api/v1/assets/${this.asset.id}/valuations`, {
					method: "POST",
					body: {
						amount_change: Number(this.transaction_data.amount),
						value_per_unit: this.transaction_data.value_per_unit,
						timestamp: new Date(this.transaction_data.timestamp),
						account_id: this.transaction_data.account_id,
						cost: this.transaction_data.cost,
						total_value: this.transaction_data.total_manually_changed ? this.transaction_data.total : null,
						recipient_id: this.transaction_data.recipient_id,
					}
				});
				this.$emit("saved");
			} catch(e: any) {
				console.error(e);
				window.alert(e?.data);
				return;
			}
		},

		update_transaction_total() {
			this.transaction_data.total_manually_changed = false;
			const raw_value_per_unit = (this.transaction_data.value_per_unit.major * this.transaction_data.value_per_unit.minor_in_major) + this.transaction_data.value_per_unit.minor;
			const raw_cost = (this.transaction_data.cost.major * this.transaction_data.cost.minor_in_major) + this.transaction_data.cost.minor;
			const raw_total = Math.round(((Number(this.transaction_data.amount) * raw_value_per_unit) + raw_cost) * -100 + Number.EPSILON) / 100;
			
			this.transaction_data.total = {
				major: (raw_total / this.transaction_data.value_per_unit.minor_in_major).toFixed(0),
				minor: raw_total < 0 ? (raw_total % this.transaction_data.value_per_unit.minor_in_major) * -1 : (raw_total % this.transaction_data.value_per_unit.minor_in_major),
				minor_in_major: this.transaction_data.value_per_unit.minor_in_major,
				symbol: this.transaction_data.value_per_unit.symbol,
				is_negative: raw_total < 0,
			};
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