<template>
	<div id="wrapper" v-if="tableData && Object.keys(tableData).length > 0">
		<table :class="tableData.auto_sizing ? 'auto_size' : 'fixed_size'">
			<colgroup>
				<col v-if="tableData.multiSelect" class="multiselect">
				<col v-for="(header, index) in tableData.columns.filter(x => !x.hidden)" :key="index" :class="header.type">
			</colgroup>
			<thead>
				<tr>
					<th v-if="tableData.multiSelect"><input type="checkbox" v-model="allRowsSelected" @click="selectAllRows"></th>
					<th v-for="(header, index) in tableData.columns.filter(x => !x.hidden)" :key="index">
						<p v-if="tableData.columns.filter(x => !x.hidden)[index].sortable" @click="updateSort(index)">
							{{header.name}}
							<svg v-if="currentSort.column === index && currentSort.sort == 'desc'" xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 4h13M3 8h9m-9 4h6m4 0l4-4m0 0l4 4m-4-4v12" /></svg>
							<svg v-else-if="currentSort.column === index && currentSort.sort == 'asc'" xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 4h13M3 8h9m-9 4h9m5-4v12m0 0l-4-4m4 4l4-4" /></svg>
							<svg v-else xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16V4m0 0L3 8m4-4l4 4m6 0v12m0 0l4-4m-4 4l-4-4" /></svg>
						</p>
						<p v-else>
							{{header.name}}
						</p>

						<div v-if="header.type == 'choice' && !tableData.columns.filter(x => !x.hidden)[index].no_filter" class="columnHeaderWrapper">
							<div>
								<select v-model="filters[index].value" @change="filter()">
									<option value=""></option>
									<option v-for="(item, index) in header.options" :key="index" :value="item.id">{{item.name}}</option>
								</select>
								<svg @click="openFilter = index" xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 4a1 1 0 011-1h16a1 1 0 011 1v2.586a1 1 0 01-.293.707l-6.414 6.414a1 1 0 00-.293.707V17l-4 4v-6.586a1 1 0 00-.293-.707L3.293 7.293A1 1 0 013 6.586V4z" /></svg>
							</div>

							<div v-if="openFilter === index" class="filterPopout">
								<div>
									<input type="radio" :id="`exact${index}`" :name="`type${index}`" value="exact" v-model="filters[index].option" @change="filter()">
									<label :for="`exact${index}`">Exact</label>
								</div>
								<div>
									<input type="radio" :id="`not${index}`" :name="`type${index}`" value="not" v-model="filters[index].option" @change="filter()">
									<label :for="`not${index}`">Not</label>
								</div>
								<div>
									<input type="radio" :id="`less${index}`" :name="`type${index}`" value="less" v-model="filters[index].option" @change="filter()">
									<label :for="`less${index}`">Less</label>
								</div>
								<div>
									<input type="radio" :id="`more${index}`" :name="`type${index}`" value="more" v-model="filters[index].option" @change="filter()">
									<label :for="`more${index}`">More</label>
								</div>
							</div>
							<div v-if="openFilter === index" class="clickTarget" @click="openFilter = null"></div>
						</div>

						<div v-if="header.type == 'date' && !tableData.columns.filter(x => !x.hidden)[index].no_filter" class="columnHeaderWrapper">
							<div>
								<input type="date" v-model="filters[index].start" @input="filter()">
								-
								<input type="date" v-model="filters[index].end" @input="filter()">
								<svg @click="openFilter = index" xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 4a1 1 0 011-1h16a1 1 0 011 1v2.586a1 1 0 01-.293.707l-6.414 6.414a1 1 0 00-.293.707V17l-4 4v-6.586a1 1 0 00-.293-.707L3.293 7.293A1 1 0 013 6.586V4z" /></svg>
							</div>

							<div v-if="openFilter === index" class="filterPopout">
								<div>
									<input type="radio" :id="`between${index}`" :name="`type${index}`" value="between" v-model="filters[index].option" @change="filter()">
									<label :for="`between${index}`">Between</label>
								</div>
								<div>
									<input type="radio" :id="`outside${index}`" :name="`type${index}`" value="outside" v-model="filters[index].option" @change="filter()">
									<label :for="`outside${index}`">Outside</label>
								</div>
							</div>
							<div v-if="openFilter === index" class="clickTarget" @click="openFilter = null"></div>
						</div>

						<div v-if="header.type == 'number' && !tableData.columns.filter(x => !x.hidden)[index].no_filter" class="columnHeaderWrapper">
							<div>
								<input type="number" v-model="filters[index].value" @input="filter()">
								<svg @click="openFilter = index" xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 4a1 1 0 011-1h16a1 1 0 011 1v2.586a1 1 0 01-.293.707l-6.414 6.414a1 1 0 00-.293.707V17l-4 4v-6.586a1 1 0 00-.293-.707L3.293 7.293A1 1 0 013 6.586V4z" /></svg>
							</div>

							<div v-if="openFilter === index" class="filterPopout">
								<div>
									<input type="radio" :id="`exact${index}`" :name="`type${index}`" value="exact" v-model="filters[index].option" @change="filter()">
									<label :for="`exact${index}`">Exact</label>
								</div>
								<div>
									<input type="radio" :id="`not${index}`" :name="`type${index}`" value="not" v-model="filters[index].option" @change="filter()">
									<label :for="`not${index}`">Not</label>
								</div>
								<div>
									<input type="radio" :id="`less${index}`" :name="`type${index}`" value="less" v-model="filters[index].option" @change="filter()">
									<label :for="`less${index}`">Less</label>
								</div>
								<div>
									<input type="radio" :id="`more${index}`" :name="`type${index}`" value="more" v-model="filters[index].option" @change="filter()">
									<label :for="`more${index}`">More</label>
								</div>
							</div>
							<div v-if="openFilter === index" class="clickTarget" @click="openFilter = null"></div>
						</div>

						<div v-if="header.type == 'string' && !tableData.columns.filter(x => !x.hidden)[index].no_filter" class="columnHeaderWrapper">
							<div>
								<input v-model="filters[index].value" @input="filter()">
								<svg @click="openFilter = index" xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 4a1 1 0 011-1h16a1 1 0 011 1v2.586a1 1 0 01-.293.707l-6.414 6.414a1 1 0 00-.293.707V17l-4 4v-6.586a1 1 0 00-.293-.707L3.293 7.293A1 1 0 013 6.586V4z" /></svg>
							</div>

							<div v-if="openFilter === index" class="filterPopout">
								<div>
									<input type="radio" :id="`contains${index}`" :name="`type${index}`" value="contains" v-model="filters[index].option" @change="filter()">
									<label :for="`contains${index}`">Contains</label>
								</div>
								<div>
									<input type="radio" :id="`exact${index}`" :name="`type${index}`" value="exact" v-model="filters[index].option" @change="filter()">
									<label :for="`exact${index}`">Exact</label>
								</div>
								<div>
									<input type="radio" :id="`begins${index}`" :name="`type${index}`" value="begins_with" v-model="filters[index].option" @change="filter()">
									<label :for="`begins${index}`">Begins with</label>
								</div>
								<div>
									<input type="radio" :id="`ends${index}`" :name="`type${index}`" value="ends_with" v-model="filters[index].option" @change="filter()">
									<label :for="`ends${index}`">Ends with</label>
								</div>
								<div>
									<input type="radio" :id="`doesntcontain${index}`" :name="`type${index}`" value="doesnt_contain" v-model="filters[index].option" @change="filter()">
									<label :for="`doesntcontain${index}`">Doesn't contain</label>
								</div>
							</div>
							<div v-if="openFilter === index" class="clickTarget" @click="openFilter = null"></div>
						</div>
					</th>
				</tr>
			</thead>
			<tbody>
				<tr v-for="(row, row_index) in rowsForDisplay" :key="row_index" :ref="x => x = row[0]">
					<td v-if="tableData.multiSelect"><input type="checkbox" v-model="selectedRows[row_index]" @change="updateSelectedRows"></td>
					<td v-for="(cell, column_index) in row" :key="column_index" @click="$emit('rowClick', tableData.rows[row_index])" :class="columnsForDisplay[column_index]?.type != 'number' ? '' : parseFloat(cell) < 0 ? 'negative' : parseFloat(cell) > 0 ? 'positive' : 'zero'">{{cell}}</td>
				</tr>
			</tbody>
		</table>
		<div id="bottom_bar" class="background_color_darkest">
			<div v-if="!tableData.disable_pagination">
				<label for="page_size">Rows per Page: </label>
				<input type="number" name="page_size" v-model="pageSize" @change="updatePage()">
				<button @click="() => {currentPage=0; updatePage()}">First</button>
				<button @click="() => {currentPage--; updatePage()}">Previous</button>
				<label for="current_page">Page</label>
				<input type="number" name="current_page" v-model="currentPage" @change="updatePage()">
				<button @click="() => {currentPage++; updatePage()}">Next</button>
				<button @click="() => {currentPage=Math.ceil((Number.isInteger(tableData.row_count) ? Number(tableData.row_count) : 0) / pageSize) - 1; updatePage()}">Last</button>
			</div>
			<div v-if="(typeof tableData.row_count == 'number')">
				<p>Count: {{tableData.row_count}}</p>
				<p v-if="tableData.total_amount">Sum: {{tableData.total_amount}}</p>
			</div>
		</div>
	</div>
</template>

<script lang="ts">
export default {
	data: () => ({
		rows: [] as Row[],
		currentSort: {} as TableSort,
		filters: [] as TableFilter[],
		rowsForDisplay: [] as Row[],
		columnsForDisplay: [] as Column[],
		selectedRows: [] as boolean[],
		openFilter: null as number | null,
		allRowsSelected: false,
		pageSize: 50,
		currentPage: 0,
		tableData: {} as TableData,
	}),

	emits: ["rowClick", "updateSort", "resetSort", "updateFilter", "resetFilter", "applyFilter", "updatePage", "rowSelect"],

	props: {
		tableDataProp: {
			type: Object as PropType<TableData>,
			required: true,
		},
	},
	
	async mounted() {
		this.update();

		this.tableData.columns.filter(x => !x.hidden).forEach(c => {
			this.filters.push({
				type: c.type,
				option: c.type == "choice" ? "exact" : c.type == "date" ? "between" : c.type == "number" ? "exact" : "contains",
				empty: c.type == "string" ? "anything" : "",
			});
		});

		this.currentSort = this.tableData.defaultSort;
		this.resetSelectedRows();
	},
	
	methods: {
		update() {
			this.tableData = structuredClone(toRaw(this.tableDataProp));
			this.rows = this.tableData.rows;
			
			this.columnsForDisplay = this.tableData.columns.filter(y => !y.hidden);

			const hidden_columns = this.tableData.columns.filter(y => y.hidden);
			if(hidden_columns.length === 0) {
				this.rowsForDisplay = this.rows;
			} else {
				this.rowsForDisplay = this.rows.map(x => {
					let new_row = [];
					for(let i = 0; i < this.tableData.columns.length; i++) {
						if (!this.tableData.columns[i].hidden) {
							new_row.push(x[i]);
						}
					}
					return new_row;
				})
			}

			this.updateRowsCurrentPage();
		},

		updateSelectedRows() {
			let selectedRowContents: Row = [];
				this.selectedRows.forEach((selected, i) => {
					if(selected) selectedRowContents.push({...this.rowsForDisplay[i]});
				});
				this.$emit("rowSelect", selectedRowContents);
		},

		resetSelectedRows() {
			this.selectedRows = [];
			this.updateSelectedRows();
			this.rowsForDisplay.forEach(() => this.selectedRows.push(false));
		},

		selectAllRows() {
			this.allRowsSelected = !this.allRowsSelected;
			this.selectedRows = this.selectedRows.map(() => this.allRowsSelected);
			this.updateSelectedRows();
		},

		updateSort(i: number) {
			if(this.currentSort.column === i) {
				if(this.currentSort.sort == "desc") {
					this.currentSort.sort = "asc";
				} else {
					if(this.currentSort.column === this.tableData.defaultSort.column && this.currentSort.sort === this.tableData.defaultSort.sort) {
						this.currentSort.sort = "desc";
					} else {
						this.currentSort = this.tableData.defaultSort;
					}
				}
			} else {
				this.currentSort = {
					column: i,
					sort: "desc",
				};
			}
			const property_name = this.tableData.columns.filter(x => !x.hidden)[this.currentSort.column].name;
			const direction = this.currentSort.sort;
			this.$emit("updateSort", property_name, direction);
		},
		
		filter() {
			this.resetSelectedRows();
			for(let i = 0; i < this.filters.length; i++) {
				if(this.filters[i].type == "date") {
					if(this.filters[i].start && this.filters[i].end) {
						this.$emit("updateFilter", this.tableData.columns.filter(x => !x.hidden)[i].name, {lower: this.filters[i].start, upper: this.filters[i].end}, this.filters[i].option);
					} else {
						this.$emit("resetFilter", this.tableData.columns.filter(x => !x.hidden)[i].name);
					}
				}

				if(this.filters[i].type == "number") {
					if(typeof this.filters[i].value == "number") {
						this.$emit("updateFilter", this.tableData.columns.filter(x => !x.hidden)[i].name, this.filters[i].value, this.filters[i].option);
					} else {
						this.$emit("resetFilter", this.tableData.columns.filter(x => !x.hidden)[i].name);
					}
				}

				if(this.filters[i].type == "choice") {
					if(this.filters[i].value?.toString().length == 36 || typeof this.filters[i].value == "number") {
						this.$emit("updateFilter", this.tableData.columns.filter(x => !x.hidden)[i].name, this.filters[i].value, this.filters[i].option);
					} else {
						this.$emit("resetFilter", this.tableData.columns.filter(x => !x.hidden)[i].name);
					}
				}

				if(this.filters[i].type == "string") {
					if(this.filters[i].value || this.filters[i].empty != "anything") {
						this.$emit("updateFilter", this.tableData.columns.filter(x => !x.hidden)[i].name, this.filters[i].value, this.filters[i].option);
					} else {
						this.$emit("resetFilter", this.tableData.columns.filter(x => !x.hidden)[i].name);
					}
				}
			}

			this.$emit("applyFilter");
		},

		updateRowsCurrentPage() {
			this.resetSelectedRows();
			if(this.currentPage > Math.ceil((Number.isInteger(this.tableData.row_count) ? Number(this.tableData.row_count) : 10000) / this.pageSize) - 1) {
				this.currentPage = Math.ceil((Number.isInteger(this.tableData.row_count) ? Number(this.tableData.row_count) : 10000) / this.pageSize) - 1;
			}
			if(this.currentPage < 0) this.currentPage = 0;
		},
		
		updatePage() {
			this.$emit("updatePage", this.currentPage, this.pageSize);
			this.updateRowsCurrentPage();
		}
	},
	watch: {
		tableDataProp: {
			handler() {
				this.update();
			},
			deep: true,
		},
	}
}
</script>

<style lang="sass" scoped>

div#wrapper
	@media screen and (max-width: 800px)
		width: min-content

table
	width: 100% !important
	border-collapse: separate
	border-spacing: 0px
	white-space: nowrap
	text-align: center
	select
		height: 100%
		right: 0
		margin: 0
		width: 100%
	@media screen and (max-width: 800px)
		table-layout: auto
		width: max-content

table.auto_size
	table-layout: auto
	td
		padding: 0 10px 0 10px

table.fixed_size
	table-layout: fixed

td
	white-space: nowrap
	overflow: hidden
	text-overflow: ellipsis

tr
	cursor: pointer

thead
	position: sticky
	top: 0
	padding: 4px 0 4px
	input, select
		max-width: 100px

div.columnHeaderWrapper
	display: flex
	flex-direction: column
	align-items: center

svg
	margin-bottom: -6px
	height: 24px

div.filterPopout
	position: absolute
	margin-top: 38px
	padding: 5px 20px 5px 20px
	display: flex
	flex-direction: column
	align-items: flex-start
	font-size: 16px
	background: rgba(0, 0, 0, 0.5)
	backdrop-filter: blur(5px) saturate(20%)
	box-shadow: 0px 0px 15px black
	z-index: 5
	hr
		color: white
		width: 100%
		margin: 10px 0 10px

div.clickTarget
	position: fixed
	top: 0px
	left: 0px
	z-index: 4
	width: 100vw
	height: 100vh

col.multiselect
	width: 1.5em
col.number
	width: 10em
col.choice
	width: 10em
col.date
	width: 20em
col.string
	width: 30em

div#bottom_bar
	display: flex
	justify-content: space-between
	align-items: center
	width: 100%
	div
		padding: 0 0.5em 0 0.5em
	input[type="number"]
		width: 4em
	@media screen and (max-width: 800px)
		justify-content: start
		flex-direction: column
		align-items: flex-start
</style>