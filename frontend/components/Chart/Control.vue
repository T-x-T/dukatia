<template>
	<div id="wrapper">
		<label for="from">From:</label>
		<input type="date" id="from" v-model="from_date" @change="updateDate">

		<label for="to">To:</label>
		<input type="date" id="to" v-model="to_date" @change="updateDate">

		<label for="range">Range: </label>
		<select id="range" v-model="date_range" @change="update">
			<option value="0">Last 28 days</option>
			<option value="1">Last month</option>
			<option value="2">Current month</option>
			<option value="3">Last 90 days</option>
			<option value="4">Last quarter</option>
			<option value="5">Current year</option>
			<option value="6">Last 365 days</option>
			<option value="7">Total</option>
		</select>

		<label for="period">Period: </label>
		<select id="period" v-model="date_period" @change="update">
			<option value="daily">Daily</option>
			<option value="monthly">Monthly</option>
			<option value="quarterly">Quarterly</option>
			<option value="yearly">Yearly</option>
		</select>
	</div>
</template>

<script lang="ts">
export default {
	data: () => ({
		from_date: new Date(Date.now() - 1000 * 60 * 60 * 24 * 28).toISOString().slice(0, 10),
		to_date: new Date().toISOString().slice(0, 10),
		date_range: "0",
		date_period: "",
	}),

	emits: ["update"],

	props: {
		prop_date_range: String,
		prop_date_period: String,
	},

	mounted() {
		this.update_from_props();
	},

	methods: {
		update_from_props() {
			if(typeof this.prop_date_range == "string") this.date_range = this.prop_date_range;
			if(typeof this.prop_date_period == "string") this.date_period = this.prop_date_period;
			this.update();
		},

		updateDate() {
			this.$emit("update", {
				from_date: this.from_date,
				to_date: this.to_date,
				date_period: this.date_period
			});
		},

		update() {
			switch(this.date_range) {
				case "0": {
					this.from_date = new Date(Date.now() - 1000 * 60 * 60 * 24 * 28).toISOString().slice(0, 10);
					this.to_date = new Date().toISOString().slice(0, 10);
					break;
				}
				case "1": {
					let from_date = new Date();
					from_date.setMonth(from_date.getMonth() - 1)
					from_date.setDate(1);
					this.from_date = from_date.toISOString().slice(0, 10);

					let to_date = new Date();
					to_date.setDate(0);
					this.to_date = to_date.toISOString().slice(0, 10);
					break;
				}
				case "2": {
					this.from_date = new Date(new Date().setDate(1)).toISOString().slice(0, 10);
					this.to_date = new Date(new Date(new Date(this.from_date).setMonth(new Date (this.from_date).getMonth() + 1)).setDate(0)).toISOString().slice(0, 10);
					break;
				}
				case "3": {
					this.from_date = new Date(Date.now() - 1000 * 60 * 60 * 24 * 90).toISOString().slice(0, 10);
					this.to_date = new Date().toISOString().slice(0, 10);
					break;
				}
				case "4": {
					let quarter;
					let year = new Date().getFullYear();

					if(new Date().getMonth() <= 2) {
						quarter = 1;
					} else if(new Date().getMonth() <= 5) {
						quarter = 2;
					} else if(new Date().getMonth() <= 8) {
						quarter = 3;
					} else {
						quarter = 4;
					}

					if(quarter == 1) {
						quarter = 4;
						year--;
					} else {
						quarter--;
					}

					switch(quarter) {
						case 1: {
							this.from_date = year + "-01-01";
							this.to_date = year + "-03-31";
							break;
						}
						case 2: {
							this.from_date = year + "-04-01";
							this.to_date = year + "-06-30";
							break;
						}
						case 3: {
							this.from_date = year + "-07-01";
							this.to_date = year + "-09-30";
							break;
						}
						case 4: {
							this.from_date = year + "-10-01";
							this.to_date = year + "-12-31";
							break;
						}
					}
					break;
				}
				case "5": {
					this.from_date = new Date().getFullYear() + "-01-01";
					this.to_date = new Date().getFullYear() + "-12-31";
					break;
				}
				case "6": {
					this.from_date = new Date(Date.now() - 1000 * 60 * 60 * 24 * 365).toISOString().slice(0, 10);
					this.to_date = new Date().toISOString().slice(0, 10);
					break;
				}
				case "7": {
					this.from_date = "0001-01-01";
					this.to_date = "9999-12-31";
					break;
				}
			}

			this.$emit("update", {
				from_date: this.from_date,
				to_date: this.to_date,
				date_period: this.date_period
			});
		}
	},

	watch: {
		prop_date_period() {
			this.update_from_props();
		},
		prop_date_range() {
			this.update_from_props();
		},
	}
}
</script>