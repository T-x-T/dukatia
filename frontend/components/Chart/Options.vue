<template>
	<div id="wrapper">
		<h5>Options</h5>

		<label for="title">Title:</label>
		<input type="text" v-model="options.title" name="title" @input="change_options" />
		<br>

		<label for="max_items">Maximum Number of Items:</label>
		<input type="number" v-model="options.max_items" name="max_items" @input="change_options" />
		<br>

		<label for="chart_type">Chart Type:</label>
		<select v-model="options.chart_type" name="chart_type" @change="change_options">
			<option value="table">Table</option>
			<option value="pie">Pie</option>
			<option value="line">Line</option>
		</select>
		<br>

		<label for="collection">Collection:</label>
		<select v-model="options.filter_collection" name="collection" @change="change_options">
			<option v-for="(item, index) in filter_collections" :key="index" :value="item">{{item}}</option>
		</select>
		<br>

		<label for="date_period">Default Period:</label>
		<select v-model="options.date_period" name="date_period" @change="change_options">
			<option value="daily">Daily</option>
			<option value="monthly">Monthly</option>
			<option value="quarterly">Quarterly</option>
			<option value="yearly">Yearly</option>
		</select>
		<br>

		<label for="date_range">Default Date Range:</label>
		<select v-model="options.date_range" name="date_range" @change="change_options">
			<option value="0">Last 28 days</option>
			<option value="1">Last month</option>
			<option value="2">Current month</option>
			<option value="3">Last 90 days</option>
			<option value="4">Last quarter</option>
			<option value="5">Current year</option>
			<option value="6">Last 365 days</option>
			<option value="7">Total</option>
		</select>
		<br>

		<label for="only_positive">Only Positive:</label>
		<input type="checkbox" v-model="options.only_positive" name="only_positive" @change="change_options" />
		<br>

		<label for="only_negative">Only Negative:</label>
		<input type="checkbox" v-model="options.only_negative" name="only_negative" @change="change_options" />
		<br>
		
		<button v-if="Number.isInteger(options.id)" class="red" @click="delete_this">Delete</button>
	</div>
</template>

<script lang="ts">
export default {
	data: () => ({
		options: {} as ChartOptions,
		filter_collections: [
			"get_per_recipient_over_time",
			"get_all_budget_utilization_overview",
			"get_per_account_over_time",
			"get_per_currency_over_time",
			"get_earning_spending_net_over_time",
			"get_per_tag_over_time",
		]
	}),

	emits: ["deleted", "update"],

	props: {
		chart_options: {
			type: Object as PropType<ChartOptions>,
		}
	},

	created() {
		if(this.chart_options) {
			this.options = {...this.chart_options};
		} else {
			this.options = {
				chart_type: "line",
				title: "Your new chart",
				date_period: "daily",
				date_range: 1,
				filter_collection: "earning_spending_net",
				max_items: 10,
				top_left_x: 0,
				top_left_y: 0,
				bottom_right_x: 5,
				bottom_right_y: 2,
				only_negative: false,
				only_positive: false,
			};
		}
	},

	methods: {
		async save() {
			if(Number.isInteger(this.options.id)) {
				await $fetch(`/api/v1/charts/${this.options.id}`, {
					method: "PUT", body: {
						...this.options,
						date_range: Number(this.options.date_range),
					}
				});
			} else {
				await $fetch("/api/v1/charts", {
					method: "POST", body: {
						...this.options,
						date_range: Number(this.options.date_range),
					}
				});
			}
		},

		async delete_this() {
			await $fetch(`/api/v1/charts/${this.options.id}`, {method: "DELETE"});
			this.$emit('deleted');
		},

		async change_options() {
			await this.save();
			this.$emit("update", this.options);
		},
	},
}
</script>

<style lang="sass" scoped>
div#wrapper
	height: 100%
	width: 100%
	position: relative

h5
	text-align: center

input
	width: 4em
input[type="text"]
	max-width: 40em
	width: 80%

</style>