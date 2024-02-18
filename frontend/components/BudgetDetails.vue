<template>
	<div>
		<div id="grid">
			<div class="gridItem form">
				<BudgetForm
					:data="budget"
					@back="$emit('back')"
					@data_saved="reload"
				/>
			</div>

			<div v-if="chart_utilization_current_period" class="gridItem pie_chart">
				<h3>Current Period</h3>
				<div class="actual_chart">
					<ChartPie
						:pie="chart_utilization_current_period"
					/>
				</div>
			</div>
			<div v-if="chart_utilization_previous_period" class="gridItem pie_chart">
				<h3>Previous Period</h3>
				<div class="actual_chart">
					<ChartPie
						:pie="chart_utilization_previous_period"
					/>
				</div>
			</div>
			<div v-if="chart_utilization_history" class="gridItem line_chart">
				<h3>Utilization History</h3>
				<div class="actual_chart">
					<ChartLine
						:line="chart_utilization_history"
					/>
				</div>
			</div>
			<div v-if="Object.keys(table_data).length > 0" class="gridItem">
				<h3>Transactions in current Period</h3>
				<CustomTable
					:tableDataProp="table_data"
				/>
			</div>
		</div>
	</div>	
</template>

<script lang="ts">
export default {
	data: () => ({
		chart_utilization_current_period: null as any,
		chart_utilization_previous_period: null as any,
		chart_utilization_history: null as any,
		table_data: {} as TableData,
		budget: {} as Budget | undefined,
	}),

	props: {
		prop_budget: {
			type: Object as PropType<Budget>,
			required: false,
		}
	},

	emits: ["back"],

	async created() {
		this.budget = structuredClone(toRaw(this.prop_budget));
		await this.update();
	},

	methods: {
		async reload(res?: any) {
			if(res?.id) (this.budget as Budget).id = res.id;

			if(!this.budget || Object.keys(this.budget).length === 0) {
				console.error("this.budget isnt defined in BudgetDetails.vue reload method");
				return;
			}

			if(Number.isInteger(res?.id)) {
				console.log("res.id is good")
				await useRouter().push(`/budgets/${res.id}`);
				(this.budget as Budget).id = res.id;
			}
			this.budget = await $fetch(`/api/v1/budgets/${(this.budget as Budget).id}`);
			await this.update();
		},

		async update() {
			if(this.budget?.id !== undefined) {
				this.chart_utilization_current_period = null;
				this.chart_utilization_previous_period = null;
				this.chart_utilization_history = null;

				this.$nextTick(async () => {
					this.chart_utilization_current_period = (await $fetch(`/api/v1/charts/by_collection/get_single_budget_current_period_utilization?budget_id=${(this.budget as Budget).id}`));
					
					const chart_utilization_previous_period = (await $fetch(`/api/v1/charts/by_collection/get_single_budget_previous_period_utilization?budget_id=${(this.budget as Budget).id}`));
					if (chart_utilization_previous_period.datasets[0].data[0].value !== 0 || chart_utilization_previous_period.datasets[1].data[0].value) {
						this.chart_utilization_previous_period = chart_utilization_previous_period;
					}
	
					this.chart_utilization_history = (await $fetch(`/api/v1/charts/by_collection/get_single_budget_utilization_history?budget_id=${(this.budget as Budget).id}`));
				});


				const transactions = await $fetch(`/api/v1/budgets/${this.budget.id}/transactions`) as Transaction[];
				const accounts = await $fetch("/api/v1/accounts/all") as Account[];
				const currencies = await $fetch("/api/v1/currencies/all") as Currency[];
				const recipients = await $fetch("/api/v1/recipients/all") as Recipient[];
				const tags = await $fetch("/api/v1/tags/all") as Tag[];
 
				const transactionsForDisplay = transactions.map(x => {
					x.account = accounts.filter(a => a.id === x.account_id)[0];
					x.currency = currencies.filter(c => c.id === x.currency_id)[0];
					x.recipient = recipients.filter(r => r.id === x.recipient_id)[0];
					return x;
				});
				this.table_data = {} as TableData;
				this.$nextTick(() => {
					this.table_data = {
						multiSelect: false,
						disable_pagination: true,
						defaultSort: {
							column: 4,
							sort: "desc"
						},
						columns: [
							{name: "ID", type: "number", sortable: false, no_filter: true, hidden: true},
							{name: "Account", type: "string", sortable: false, no_filter: true},
							{name: "Recipient", type: "string", sortable: false, no_filter: true},
							{name: "Asset", type: "string", sortable: false, no_filter: true},
							{name: "Timestamp", type: "date", sortable: false, no_filter: true},
							{name: "Amount", type: "number", sortable: false, no_filter: true},
							{name: "Comment", type: "string", sortable: false, no_filter: true},
							{name: "Tags", type: "string", sortable: false, no_filter: true},
						],
						rows: transactionsForDisplay.map(x => ([
							x.id,
							x.account?.name,
							x.recipient?.name,
							x.asset ? x.asset.name : "",
							new Date(new Date(x.timestamp).valueOf() - (new Date(x.timestamp).getTimezoneOffset() * 60000)).toISOString().slice(0, 10),
							`${x.total_amount.major >= 0 && x.total_amount.is_negative ? "-" : ""}${x.total_amount.major}.${x.total_amount.minor.toString().padStart(x.total_amount.minor_in_major.toString().length - 1, "0")}${x.total_amount.symbol}`,
							x.comment,
							tags.filter(y => x.tag_ids?.includes((Number.isInteger(y.id) ? y.id : -1) as number)).map(y => y.name).join(", ")
						]))
					};
				});
			}
		}
	}
}
</script>

<style lang="sass" scoped>

h3
	text-align: center
	font-size: 1.5em

div#grid
	display: flex
	width: 100%
	justify-content: flex-start
	align-items: flex-start
	align-content: flex-start
	gap: 10px
	flex-wrap: wrap

div.form
	display: flex
	align-items: center
	justify-content: center
	height: max-content

div.gridItem
	padding: 10px

div.pie_chart
	width: 20em

div.line_chart
	width: 60em

div.actual_chart
	height: 20em
</style>