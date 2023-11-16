<template>
	<div id="wrapper" v-if="transaction && Object.keys(transaction).length > 1">
		<button class="orange" @click="$emit('back')">Back</button>
		
		<label>
			ID:
			<input type="number" v-model="transaction.id" disabled>
		</label>
		<label>
			Account:
			<select ref="first_input" v-model="transaction.account_id">
				<option v-for="(account, index) in accounts" :key="index" :value="account.id">{{account.name}}</option>
			</select>
		</label>
		<button tabindex="-1">+</button>
		<label>
			Recipient:
			<select v-model="transaction.recipient_id">
				<option v-for="(recipient, index) in recipients" :key="index" :value="recipient.id">{{recipient.name}}</option>
			</select>
		</label>
		<button tabindex="-1">+</button>
		<label>
			Asset:
			<select v-model="transaction.asset_id">
				<option value=""></option>
				<option v-for="(asset, index) in assets" :key="index" :value="asset.id">{{asset.name}}</option>
			</select>
		</label>
		<label>
			Timestamp:
			<input type="datetime-local" v-model="transaction.timestamp_string">
		</label>
		<label>
			Comment:
			<input type="string" v-model="transaction.comment">
		</label>
		<div v-if="transaction.positions.length > 1" v-for="(position_data, position_index) in transaction.positions">
			<label>
				Amount:
				<InputMoney
					v-if="position_data && Object.keys(position_data).length > 0"
					:initial_value="position_data.amount"
					@update="((new_value: Money) => position_data.amount = new_value)"
				/>
			</label>
			<label>
				Comment:
				<input type="string" v-model="position_data.comment">
			</label>
			<label>
				Tag:
				<select v-model="position_data.tag_id">
					<option value=""></option>
					<option v-for="(tag, index) in tags" :key="index" :value="tag.id">{{tag.name}}</option>
				</select>
			</label>
			<button class="red" @click="transaction.positions.splice(position_index, 1)">Delete Position</button>
		</div>
		<div v-else>
			<label>
				Amount:
				<InputMoney
					v-if="transaction.positions[0].amount && Object.keys(transaction.positions[0].amount).length > 0"
					:initial_value="transaction.positions[0].amount"
					@update="((new_value: Money) => transaction.positions[0].amount = new_value)"
				/>
			</label>
		</div>
		<button @click="transaction.positions.push({...default_data.positions[0]})">Add Position</button>
		<br>
		<InputMultiSelect
			v-if="tags_select_data && Object.keys(tags_select_data).length > 0"
			:selectData="tags_select_data"
			v-on:update="(selected: number[]) => transaction.tag_ids = selected"
			style="margin-right: 5px;"
		/>
		<button tabindex="-1">+</button>

		<br>
		<button class="green" @click="save(true)">Save</button>
		<button class="orange" @click="$emit('back')">Cancel</button>
		<button class="green" @click="save(false)">Save and New</button>
		<button class="red" @click="delete_this">Delete</button>
	</div>
</template>

<script lang="ts">
export default {
	data: () => ({
		transaction: {} as Transaction,
		tags_select_data: {} as SelectData | null,
		tags: [] as Tag[],
		currencies: [] as Currency[],
		assets: [] as Asset[],
		recipients: [] as Recipient[],
		accounts: [] as Account[],
	}),

	emits: ["back", "data_saved"],

	props: {
		data: {
			type: Object as PropType<Transaction>,
			required: false,
		},

		default_data: {
			type: Object as PropType<Transaction>,
			required: true,
		},
	},

	async mounted() {		
		this.transaction = this.data ? this.data : this.default_data;
		this.transaction.timestamp_string = new Date(new Date(this.transaction.timestamp).valueOf() - (new Date(this.transaction.timestamp).getTimezoneOffset() * 60000)).toISOString().slice(0, -8);
		this.tags = await $fetch("/api/v1/tags/all");
		this.currencies = await $fetch("/api/v1/currencies/all");
		this.assets = await $fetch("/api/v1/assets/all");
		this.recipients = await $fetch("/api/v1/recipients/all");
		this.accounts = await $fetch("/api/v1/accounts/all");

		this.update_tags_select_data();
		(this.$refs.first_input as any).focus();
	},

	methods: {
		async save(go_back: boolean) {
			let res = {} as Budget;

			try {
				if(typeof this.transaction.id == "number") {
					res = await $fetch(`/api/v1/transactions/${this.transaction.id}`, {
						method: "PUT",
						body: this.get_body(),
					});
				} else {
					res = await $fetch("/api/v1/transactions", {
						method: "POST",
						body: this.get_body(),
					});
				}
			} catch(e: any) {
				console.error(e);
				window.alert(e?.data);
				return;
			}

			this.$emit("data_saved", res);
			
			if(go_back) {
				this.$emit("back");
			} else {
				(this as any).transaction = null;
				this.$nextTick(() => {
					this.transaction = structuredClone(toRaw(this.default_data));
					this.transaction.timestamp_string = new Date(new Date(this.transaction.timestamp).valueOf() - (new Date(this.transaction.timestamp).getTimezoneOffset() * 60000)).toISOString().slice(0, -8);
					this.update_tags_select_data();
					this.$nextTick(() => (this.$refs.first_input as any).focus());
				});
			}
		},

		async delete_this() {
			try {
				await $fetch(`/api/v1/transactions/${this.transaction.id}`, { method: "DELETE" });
				this.$emit("back");
			} catch(e: any) {
				console.error(e);
				window.alert(e?.data);
			}
		},

		update_tags_select_data() {
			this.tags_select_data = null;
			this.$nextTick(() => {
				this.tags_select_data = {
					options: [...this.tags.map(x => ({id: (Number.isInteger(x.id) ? x.id : -1) as number, name: x.name}))],
					selected: this.transaction.tag_ids,
					label: "Tags:"
				}
			});
		},
		
		get_body() {
			return {
				account_id: this.transaction.account_id,
				recipient_id: this.transaction.recipient_id,
				asset_id: this.transaction.asset_id ? this.transaction.asset_id : undefined,
				currency_id: this.transaction.currency_id,
				status: this.transaction.status,
				timestamp: new Date(this.transaction.timestamp_string as any),
				comment: this.transaction.comment,
				tag_ids: Array.isArray(this.transaction.tag_ids) && typeof this.transaction.tag_ids[0] == "number" ? this.transaction.tag_ids : undefined,
				positions: this.transaction.positions,
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