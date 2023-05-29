<template>
	<div id="wrapper">
		<button id="back_button" @click="save_and_go_back">Back</button>
		<h5>Options</h5>

		<label for="title">Title:</label>
		<input type="text" v-model="options.title" name="title" />
		<br>

		<label for="max_items">Maximum Number of Items:</label>
		<input type="number" v-model="options.max_items" name="max_items" />
		<br>

		<label for="chart_type">Chart Type:</label>
		<select v-model="options.chart_type" name="chart_type">
			<option value="text">Text</option>
			<option value="pie">Pie</option>
			<option value="line">Line</option>
		</select>
		<br>

		<label v-if="options.chart_type != 'text'" for="collection">Collection:</label>
		<select v-if="options.chart_type != 'text'" v-model="options.filter_collection" name="collection">
			<option v-for="(item, index) in filter_collections[options.chart_type]" :value="item">{{item}}</option>
		</select>
		<label v-if="options.chart_type == 'text'" for="text_template">Template:</label>
		<input type="text" v-if="options.chart_type == 'text'" v-model="options.text_template" name="text_template" />
		<br>

		<label v-if="options.chart_type == 'line'" for="date_period">Default Period:</label>
		<select v-if="options.chart_type == 'line'" v-model="options.date_period" name="date_period">
			<option value="daily">Daily</option>
			<option value="monthly">Monthly</option>
			<option value="quarterly">Quarterly</option>
			<option value="yearly">Yearly</option>
		</select>
		<br>

		<label for="date_range">Default Date Range:</label>
		<select v-model="options.date_range" name="date_range">
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

		<label for="top_left_x">Top left X:</label>
		<input type="number" v-model="options.top_left_x" @change="change_size" name="top_left_x" />
		<label for="top_left_y">Top left Y:</label>
		<input type="number" v-model="options.top_left_y" @change="change_size" name="top_left_y" />
		<br>
		<label for="bottom_right_x">Bottom right X:</label>
		<input type="number" v-model="options.bottom_right_x" @change="change_size" name="bottom_right_x" />
		<label for="bottom_right_y">Bottom right Y:</label>
		<input type="number" v-model="options.bottom_right_y" @change="change_size" name="bottom_right_y" />
	</div>
</template>

<script lang="ts">
export default {
	data: () => ({
		options: {} as ChartOptions,
		filter_collections: {
			"pie": [
				"recipients",
				"tags",
			],
			"line": [
				"recipients",
				"accounts",
				"currencies",
				"earning_spending_net",
			],
		}
	}),

	props: {
		chart_options: {
			type: Object as PropType<ChartOptions>,
			required: true,
		}
	},

	created() {
		this.options = {...this.chart_options};
	},

	methods: {
		async save_and_go_back() {
			await this.save();
			this.$emit('back');
		},

		async change_size() {
			await this.save();
			this.$emit('change_size');
		},

		async save() {
			await $fetch(`/api/v1/charts/${this.options.id}`, {
				method: "PUT", body: {
					...this.options,
					date_range: Number(this.options.date_range),
				}
			});
		}
	},
}
</script>

<style lang="sass">
div#wrapper
	height: 100%
	width: 100%

h5
	text-align: center

#back_button
	position: absolute
	width: fit-content
</style>