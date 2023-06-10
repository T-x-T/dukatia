<template>
	<div>
		<table>
			<colgroup>
					<col v-if="tableData.multiSelect" class="multiselect">
					<col v-for="(header, index) in tableData.columns" :key="index" :class="header.type">
			</colgroup>
			<thead>
				<tr>
					<th v-if="tableData.multiSelect"><input type="checkbox" v-model="allRowsSelected" @click="selectAllRows"></th>
					<th v-for="(header, index) in tableData.columns" :key="index">
						<div v-if="Number.isInteger(openFilter)" class="clickTarget" @click="openFilter = null"></div>
						
						<p @click="updateSort(index)">{{header.name}}
							<svg v-if="currentSort.filter(x => x.index === index)[0] && currentSort.filter(x => x.index === index)[0].direction == 'desc'" xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 4h13M3 8h9m-9 4h6m4 0l4-4m0 0l4 4m-4-4v12" /></svg>
							<svg v-else-if="currentSort.filter(x => x.index === index)[0] && currentSort.filter(x => x.index === index)[0].direction == 'asc'" xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 4h13M3 8h9m-9 4h9m5-4v12m0 0l-4-4m4 4l4-4" /></svg>
							<svg v-else xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16V4m0 0L3 8m4-4l4 4m6 0v12m0 0l4-4m-4 4l-4-4" /></svg>
						</p>

						<div v-if="header.type == 'choice'" class="columnHeaderWrapper">
							<div>
								<select v-model="filters[index].value" @change="filter()">
									<option value=""></option>
									<option v-for="(item, index) in header.options" :key="index" :value="item">{{item}}</option>
								</select>
								<svg @click="openFilter = index" xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 4a1 1 0 011-1h16a1 1 0 011 1v2.586a1 1 0 01-.293.707l-6.414 6.414a1 1 0 00-.293.707V17l-4 4v-6.586a1 1 0 00-.293-.707L3.293 7.293A1 1 0 013 6.586V4z" /></svg>
							</div>

							<div v-if="openFilter === index" class="filterPopout">
								<div>
									<input type="radio" :id="`is${index}`" :name="`type${index}`" value="is" v-model="filters[index].option" @change="filter()">
									<label :for="`is${index}`">Is</label>
								</div>
								<div>
									<input type="radio" :id="`isnt${index}`" :name="`type${index}`" value="isnt" v-model="filters[index].option" @change="filter()">
									<label :for="`isnt${index}`">Is not</label>
								</div>
								<div>
									<input type="radio" :id="`contains${index}`" :name="`type${index}`" value="contains" v-model="filters[index].option" @change="filter()">
									<label :for="`contains${index}`">Contains</label>
								</div>
							</div>
						</div>

						<div v-if="header.type == 'date'" class="columnHeaderWrapper">
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
						</div>

						<div v-if="header.type == 'number'" class="columnHeaderWrapper">
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
									<input type="radio" :id="`less${index}`" :name="`type${index}`" value="less" v-model="filters[index].option" @change="filter()">
									<label :for="`less${index}`">Less</label>
								</div>
								<div>
									<input type="radio" :id="`more${index}`" :name="`type${index}`" value="more" v-model="filters[index].option" @change="filter()">
									<label :for="`more${index}`">More</label>
								</div>
							</div>
						</div>

						<div v-if="header.type == 'string'" class="columnHeaderWrapper">
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
									<input type="radio" :id="`begins${index}`" :name="`type${index}`" value="begins" v-model="filters[index].option" @change="filter()">
									<label :for="`begins${index}`">Begins with</label>
								</div>
								<div>
									<input type="radio" :id="`ends${index}`" :name="`type${index}`" value="ends" v-model="filters[index].option" @change="filter()">
									<label :for="`ends${index}`">Ends with</label>
								</div>
								<div>
									<input type="radio" :id="`doesntcontain${index}`" :name="`type${index}`" value="doesntcontain" v-model="filters[index].option" @change="filter()">
									<label :for="`doesntcontain${index}`">Doesn't contain</label>
								</div>
								<hr>
								<div>
									<input type="radio" :id="`notempty${index}`" :name="`empty${index}`" value="notempty" v-model="filters[index].empty" @change="filter()">
									<label :for="`notempty${index}`">Not empty</label>
								</div>
								<div>
									<input type="radio" :id="`empty${index}`" :name="`empty${index}`" value="empty" v-model="filters[index].empty" @change="filter()">
									<label :for="`empty${index}`">Empty</label>
								</div>
								<div>
									<input type="radio" :id="`anything${index}`" :name="`empty${index}`" value="anything" v-model="filters[index].empty" @change="filter()">
									<label :for="`anything${index}`">Doesn't matter</label>
								</div>
							</div>
						</div>
					</th>
				</tr>
			</thead>
			<tbody>
				<tr v-for="(row, index) in rowsCurrentPage" :key="index" :ref="x => x = row[0]">
					<td v-if="tableData.multiSelect"><input type="checkbox" v-model="selectedRows[index]"></td>
					<td v-for="(cell, index) in row" :key="index" @click="$emit('rowClick', row)">{{cell}}</td>
				</tr>
			</tbody>
		</table>
		<div id="bottom_bar" class="background_color_darkest">
			<div>
				<p>Count: {{rowsForDisplay?.length}}</p>
				<p v-if="sum">{{sum}}</p>
			</div>
			<div>
				<label for="page_size">Rows per Page: </label>
				<input type="number" name="page_size" v-model="pageSize">
				<button @click="currentPage=0">First</button>
				<button @click="currentPage--">Previous</button>
				<label for="current_page">Page</label>
				<input type="number" name="current_page" v-model="currentPage">
				<button @click="currentPage++">Next</button>
				<button @click="currentPage=Math.ceil(rowsForDisplay.length / pageSize) - 1">Last</button>
			</div>
		</div>
	</div>
</template>

<script lang="ts">
export default {
	data: () => ({
		currencies: [] as Currency[],
		rows: [] as Row[],
		currentSort: [] as {index: number, direction: "asc" | "desc"}[],
		filters: [] as TableFilter[],
		rowsForDisplay: [] as Row[],
		rowsCurrentPage: [] as Row[],
		selectedRows: [] as boolean[],
		openFilter: null as number | null,
		allRowsSelected: false,
		sum: "",
		pageSize: 50,
		currentPage: 0,
		tableData: {} as TableData,
	}),

	props: {
		tableDataProp: {
			type: Object as PropType<TableData>,
			required: true,
		},
	},
	
	async beforeMount() {
		await this.update(true);
	},
	
	methods: {
		async update(initial: boolean) {
			this.tableData = structuredClone(toRaw(this.tableDataProp));
			this.rows = this.tableData.rows;
			
			if(initial) {
				this.rowsForDisplay = this.rows;
				
				this.tableData.columns.forEach(c => {
					this.filters.push({
						type: c.type,
						option: c.type == "choice" ? "is" : c.type == "date" ? "between" : c.type == "number" ? "exact" : "contains",
						empty: c.type == "string" ? "anything" : ""
					});
				});
				
				this.currencies = await $fetch("/api/v1/currencies/all");
				this.applyDefaultSort();
				this.fillSelectedRows();
			} else {
				this.sort();
			}

			this.filter();
		},

		fillSelectedRows() {
			this.selectedRows = [];
			this.rowsForDisplay.forEach(() => this.selectedRows.push(false));
		},

		selectAllRows() {
			this.allRowsSelected = !this.allRowsSelected;
			this.selectedRows = this.selectedRows.map(() => this.allRowsSelected);
		},

		applyDefaultSort() {
			this.currentSort[0] = {
				index: this.tableData.defaultSort.column,
				direction: this.tableData.defaultSort.sort
			}
			this.sort();
		},

		updateSort(i: number) {
			if(this.currentSort.filter(x => x.index === i).length !== 0) { //If column i is sorted
				const currentSortPrio = this.currentSort.findIndex(x => x.index === i);
				if(this.currentSort[currentSortPrio].direction === "asc") {
					this.currentSort.splice(currentSortPrio, 1);
					this.currentSort.unshift({
						index: i,
						direction: "desc"
					});
					this.sort();
				} else {
					this.currentSort.splice(currentSortPrio, 1);
					if(this.currentSort.length === 0) {
						this.applyDefaultSort();
					} else {
						this.sort();
					}
					return;
				}
			} else {
				if(
					this.currentSort.length === 1 &&
					this.currentSort[0].index === this.tableData.defaultSort.column &&
					this.currentSort[0].direction === this.tableData.defaultSort.sort
				) { //If default sort is applied
					this.currentSort.shift();
				}
				this.currentSort.unshift({
					index: i,
					direction: "asc"
				});
				this.sort();
			}
		},

		sort() {
			this.fillSelectedRows();

			const columnType = this.tableData.columns[this.currentSort[0].index].type;	
			const reverseSort = this.currentSort[0].direction == "desc" ? true : false;

			switch(columnType) {
				case "string": {
					this.sortStringColumn(this.currentSort[0].index, reverseSort);
					break;
				}
				case "number": {
					this.sortNumberColumn(this.currentSort[0].index, reverseSort);
					break;
				}
				case "date": {
					this.sortDateColumn(this.currentSort[0].index, reverseSort);
					break;
				}
				case "choice": {
					this.sortStringColumn(this.currentSort[0].index, reverseSort);
					break;
				}
			}
		},

		sortNumberColumn(i: number, asc: boolean) {
			this.rows.sort((a, b) => {
				return asc ? parseInt(b[i]) - parseInt(a[i]) : parseInt(a[i]) - parseInt(b[i]);
			});
		},

		sortStringColumn(i: number, asc: boolean) {
			this.rows.sort((a, b) => {
				if(a[i].toLowerCase() > b[i].toLowerCase()) {
					return asc ? 1 : -1;
				} else if(a[i].toLowerCase() < b[i].toLowerCase()) {
					return asc ? -1 : 1;
				} else {
					return 0;
				}
			});
		},

		sortDateColumn(i: number, asc: boolean) {
			this.rows.sort((a, b) => {
				return asc ? Date.parse(b[i]) - Date.parse(a[i]) : Date.parse(a[i]) - Date.parse(b[i]);
			});
		},

		filter() {
			this.fillSelectedRows();
			this.rowsForDisplay = this.rows;
			for(let i = 0; i < this.filters.length; i++) {
				if(this.filters[i].type == "choice" && this.filters[i].value) {
					if(this.filters[i].option == "is") {
						this.rowsForDisplay = this.rowsForDisplay.filter(x => x[i].toLowerCase() === (this.filters[i].value as string)?.toLowerCase());
					}
					if(this.filters[i].option == "isnt") {
						this.rowsForDisplay = this.rowsForDisplay.filter(x => x[i].toLowerCase() !== (this.filters[i].value as string)?.toLowerCase());
					}
					if(this.filters[i].option == "contains") {
						this.rowsForDisplay = this.rowsForDisplay.filter(x => x[i].toLowerCase().includes((this.filters[i].value as string)?.toLowerCase()));
					}
				}

				if(this.filters[i].type == "date" && this.filters[i].start && this.filters[i].end) {
					if(this.filters[i].option == "between") {
						this.rowsForDisplay = this.rowsForDisplay.filter(x => Date.parse(x[i]) > Date.parse(this.filters[i].start as string) && Date.parse(x[i]) < Date.parse(this.filters[i].end as string));
					}
					if(this.filters[i].option == "outside") {
						this.rowsForDisplay = this.rowsForDisplay.filter(x => Date.parse(x[i]) < Date.parse(this.filters[i].start as string) || Date.parse(x[i]) > Date.parse(this.filters[i].end as string));
					}
				}

				if(this.filters[i].type == "number" && typeof this.filters[i].value == "number") {
					if(this.filters[i].option == "exact") {
						this.rowsForDisplay = this.rowsForDisplay.filter(x => parseFloat(x[i]) === this.filters[i].value);
					}
					if(this.filters[i].option == "less") {
						this.rowsForDisplay = this.rowsForDisplay.filter(x => parseFloat(x[i]) < (this.filters[i].value as number));
					}
					if(this.filters[i].option == "more") {
						this.rowsForDisplay = this.rowsForDisplay.filter(x => parseFloat(x[i]) > (this.filters[i].value as number));
					}
				}

				if(this.filters[i].type == "string" && (this.filters[i].value || this.filters[i].empty != "anything")) {
					if(this.filters[i].empty == "empty") {
						this.rowsForDisplay = this.rowsForDisplay.filter(x => !x[i]);
					}
					if(this.filters[i].empty == "notempty") {
						this.rowsForDisplay = this.rowsForDisplay.filter(x => x[i]);
					}
					if(this.filters[i].value) {
						if(this.filters[i].option == "contains") {
							this.rowsForDisplay = this.rowsForDisplay.filter(x => x[i].toLowerCase().includes((this.filters[i].value as string)?.toLowerCase()));
						}
						if(this.filters[i].option == "exact") {
							this.rowsForDisplay = this.rowsForDisplay.filter(x => x[i].toLowerCase() === (this.filters[i].value as string)?.toLowerCase());
						}
						if(this.filters[i].option == "begins") {
							this.rowsForDisplay = this.rowsForDisplay.filter(x => x[i].toLowerCase().startsWith((this.filters[i].value as string)?.toLowerCase()));
						}
						if(this.filters[i].option == "ends") {
							this.rowsForDisplay = this.rowsForDisplay.filter(x => x[i].toLowerCase().endsWith((this.filters[i].value as string)?.toLowerCase()));
						}
						if(this.filters[i].option == "doesntcontain") {
							this.rowsForDisplay = this.rowsForDisplay.filter(x => !x[i].toLowerCase().includes((this.filters[i].value as string)?.toLowerCase()));
						}
					}
				}
			}

			if(this.tableData.displaySum) this.getSum();
		},

		async getSum() {
			if(typeof this.tableData.sumColumn != "number") {
				console.error("CustomTable#getSum got called with missing this.tableData.sumColumn: ", this.tableData);
				return;
			}
			
			let output = "Sum:";
			this.currencies.forEach(currency => {
				output += " ";
				let total = 0;
				this.rowsForDisplay.forEach(x => x[this.tableData.sumColumn as number].endsWith(currency.symbol) ? total += Number(x[this.tableData.sumColumn as number].replace(currency.symbol, "")) : null);
				if(total !== 0) {
					output += total.toFixed(2);
					output += currency.symbol;
				}
			});
			this.sum = output;
		},

		updateRowsCurrentPage() {
			this.fillSelectedRows();
			if(this.currentPage < 0) this.currentPage = 0;
			const startingIndex = this.currentPage * this.pageSize;
			this.rowsCurrentPage = this.rowsForDisplay.slice(startingIndex, startingIndex + this.pageSize);
			if(this.currentPage > Math.ceil(this.rowsForDisplay.length / this.pageSize) - 1) {
				this.currentPage = Math.ceil(this.rowsForDisplay.length / this.pageSize) - 1;
			}
		},
	},
	watch: {
		selectedRows: {
			handler() {
				let selectedRowContents: Row = [];
				this.selectedRows.forEach((selected, i) => {
					if(selected) selectedRowContents.push({...this.rowsForDisplay[i]});
				});
				this.$emit("rowSelect", selectedRowContents);
			},
			deep: true,
		},
		tableDataProp: {
			handler() {
				this.update(false);
			},
			deep: true,
		},
		rowsForDisplay: {
			handler() {
				this.updateRowsCurrentPage();
			},
			deep: true
		},
		currentPage() {
			this.updateRowsCurrentPage();
		},
		pageSize() {
			this.updateRowsCurrentPage();
		}
	}
}
</script>

<style lang="sass" scoped>
table
	table-layout: fixed
	width: 100%
	border-collapse: separate
	border-spacing: 0px
	white-space: nowrap
	text-align: center
	select
		height: 100%
		right: 0
		margin: 0
		width: 100%

td
	white-space: break-spaces
	overflow: hidden
	text-overflow: ellipsis

tr
	cursor: pointer

th
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

div#bottom_bar
	position: sticky
	bottom: 0
	display: flex
	justify-content: space-between
	align-items: center
	div
		padding: 0 0.5em 0 0.5em
	input[type="number"]
		width: 4em
</style>