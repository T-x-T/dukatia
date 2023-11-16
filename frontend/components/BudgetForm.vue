<template>
	<div id="wrapper">
		<button class="orange" @click="$emit('back')">Back</button>
		
		<label>
			ID:
			<input type="number" v-model="budget.id" disabled>
		</label>
		<label>
			Name:
			<input type="text" v-model="budget.name" ref="first_input">
		</label>
		<label>
			Total Amount:
			<InputMoney
			v-if="budget.amount && Object.keys(budget.amount).length > 0"
				:initial_value="budget.amount"
				@update="((new_value: Money) => budget.amount = new_value)"
			/>
		</label>
		<label>
			Rollover enabled:
			<input type="checkbox" v-model="budget.rollover">
		</label>
		<label>
			Period:
			<select v-model="budget.period">
				<option value="0">Daily</option>
				<option value="1">Weekly</option>
				<option value="2">Monthly</option>
				<option value="3">Quarterly</option>
				<option value="4">Yearly</option>
			</select>
		</label>
		<label>
			Currency:
			<select v-model="budget.currency_id">
				<option v-for="(currency, index) in currencies" :key="index" :value="currency.id">{{currency.name}}</option>
			</select>
		</label>
		<label>
			Active From:
			<input type="datetime-local" v-model="budget.active_from_string">
		</label>
		<label>
			Active To:
			<input type="datetime-local" v-model="budget.active_to_string">
		</label>
		<InputMultiSelect
			v-if="filter_tags_select_data && Object.keys(filter_tags_select_data).length > 0"
			:selectData="filter_tags_select_data"
			v-on:update="(selected: number[]) => budget.filter_tag_ids = selected"
			style="margin-right: 5px;"
		/>

		<br>
		<button class="green" @click="save">Save</button>
		<button class="orange" @click="$emit('back')">Cancel</button>
		<button class="red" @click="delete_this">Delete</button>
	</div>
</template>

<script lang="ts">
export default {
	data: () => ({
		budget: {} as Budget,
		filter_tags_select_data: {} as SelectData | null,
		tags: [] as Tag[],
		currencies: [] as Currency[],
		default_budget: {
			id: undefined,
			name: "",
			amount: {major: 0, minor: 0, minor_in_major: 100, symbol: "€"},
			rollover: false,
			period: 2,
			filter_tag_ids: [],
			currency_id: 0,
			active_from: new Date(),
		} as Budget,
	}),

	emits: ["back", "data_saved"],

	props: {
		data: {
			type: Object as PropType<Budget>,
			required: false,
		}
	},

	async mounted() {
		this.budget = this.data ? this.data : structuredClone(toRaw(this.default_budget));
		this.budget.active_from_string = new Date(new Date(this.budget.active_from).valueOf() - (new Date(this.budget.active_from).getTimezoneOffset() * 60000)).toISOString().slice(0, -8),
		this.budget.active_to_string = this.budget.active_to ? new Date(new Date(this.budget.active_to).valueOf() - (new Date(this.budget.active_to).getTimezoneOffset() * 60000)).toISOString().slice(0, -8) : undefined,
		
		this.tags = await $fetch("/api/v1/tags/all");
		this.currencies = await $fetch("/api/v1/currencies/all");
		this.update_filter_tags_select_data();
		(this.$refs.first_input as any).focus();
	},

	methods: {
		async save() {
			let res = {} as Budget;

			try {
				if(typeof this.budget.id == "number") {
					res = await $fetch(`/api/v1/budgets/${this.budget.id}`, {
						method: "PUT",
						body: this.get_body(),
					});
				} else {
					res = await $fetch("/api/v1/budgets", {
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
			this.update_filter_tags_select_data();
		},

		async delete_this() {
			try {
				await $fetch(`/api/v1/budgets/${this.budget.id}`, { method: "DELETE" });
				this.$emit("back");
			} catch(e: any) {
				console.error(e);
				window.alert(e?.data);
			}
		},

		update_filter_tags_select_data() {
			this.filter_tags_select_data = null;
			this.$nextTick(() => {
				this.filter_tags_select_data = {
					options: [...this.tags.map(x => ({id: (Number.isInteger(x.id) ? x.id : -1) as number, name: x.name}))],
					selected: this.budget.filter_tag_ids,
					label: "Tags:"
				}
			});
		},
		
		get_body() {
			return {
				id: this.budget.id,
				name: this.budget.name,
				amount: this.budget.amount,
				rollover: this.budget.rollover,
				period: this.budget.period,
				filter_tag_ids: this.budget.filter_tag_ids,
				currency_id: this.budget.currency_id,
				active_from: new Date(this.budget.active_from_string as any).toISOString(),
				active_to: this.budget.active_to_string ? new Date(this.budget.active_to_string as any).toISOString() : null,
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