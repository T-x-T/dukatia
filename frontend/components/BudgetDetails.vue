<template>
	<div>
		<div id="grid">
			<div class="gridItem form">
				<DetailsPage
					v-if="Object.keys(config).length > 0"
					:config="config"
					v-on:back="$emit('back')"
					v-on:updateData="reload"
				/>
			</div>

			<div v-if="chart_utilization_current_period" class="gridItem pie_chart">
				<h3>Current Period</h3>
				<ChartPie
					:pie="chart_utilization_current_period"
				/>
			</div>
			<div v-if="chart_utilization_previous_period" class="gridItem pie_chart">
				<h3>Previous Period</h3>
				<ChartPie
					:pie="chart_utilization_previous_period"
				/>
			</div>
			<div v-if="chart_utilization_history" class="gridItem line_chart">
				<h3>Utilization History</h3>
				<ChartLine
					:line="chart_utilization_history"
				/>
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
		config: {} as DetailFormConfig,
		chart_utilization_current_period: null as any,
		chart_utilization_previous_period: null as any,
		chart_utilization_history: null as any,
		table_data: {} as TableData,
		budget: {} as Budget,
	}),

	props: {
		prop_budget: {
			type: Object as PropType<Budget>,
			required: true,
		}
	},

	async created() {
		this.budget = structuredClone(toRaw(this.prop_budget));
		await this.update();
	},

	methods: {
		async reload(res?: any) {
			if(Number.isInteger(res?.id)) await useRouter().push(`/budgets/${res.id}`);
			this.budget = await $fetch(`/api/v1/budgets/${this.budget.id}`);
			await this.update();
		},

		async update() {
			this.budget.filter_tag_ids = Array.isArray(this.budget.filter_tag_ids) ? [...this.budget.filter_tag_ids] : [];

			(this as any).config = {};
			this.$nextTick(() => {
				this.config = {
					...this.$detailPageConfig().budget,
					data: {
						...this.budget,
						active_from: new Date(new Date(this.budget.active_from).valueOf() - (new Date(this.budget.active_from).getTimezoneOffset() * 60000)).toISOString().slice(0, -8),
						active_to: this.budget.active_to ? new Date(new Date(this.budget.active_to).valueOf() - (new Date(this.budget.active_to).getTimezoneOffset() * 60000)).toISOString().slice(0, -8) : null,
					},
				};
			});


			if(this.budget?.id !== undefined) {
				this.chart_utilization_current_period = null;
				this.chart_utilization_previous_period = null;
				this.chart_utilization_history = null;

				this.$nextTick(async () => {
					this.chart_utilization_current_period = (await $fetch(`/api/v1/charts/pie/single_budget_current_period/data?budget_id=${this.budget.id}`)).pie;
					
					const chart_utilization_previous_period = (await $fetch(`/api/v1/charts/pie/single_budget_previous_period/data?budget_id=${this.budget.id}`)).pie;
					if (chart_utilization_previous_period[0][1][1] !== 0 || chart_utilization_previous_period[1][1][1] !== 0) {
						this.chart_utilization_previous_period = chart_utilization_previous_period;
					}
	
					this.chart_utilization_history = (await $fetch(`/api/v1/charts/line/single_budget_utilization_history/data?budget_id=${this.budget.id}`)).line;
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
							{name: "ID", type: "number", sortable: false, no_filter: true},
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
	height: 20em

div.line_chart
	width: 60em
	height: 20em
</style>