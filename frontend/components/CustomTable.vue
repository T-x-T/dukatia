<template>
	<table>
		<thead>
			<tr>
				<th><input type="checkbox" v-model="allRowsSelected" @click="selectAllRows"></th>
				<th v-for="(header, index) in tableData.columns" :key="index">
					<p @click="sort(index)">{{header.name}}</p>
					<div v-if="Number.isInteger(openFilter)" class="clickTarget" @click="openFilter = null"></div>
					
					<div v-if="header.type == 'choice'" class="columnHeaderWrapper">
						<div>
							<select v-model="filters[index].value" @change="filter()">
								<option value=""></option>
								<option v-for="(item, index) in header.options" :key="index" :value="item">{{item}}</option>
							</select>
							<svg @click="openFilter = index" xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6V4m0 2a2 2 0 100 4m0-4a2 2 0 110 4m-6 8a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4m6 6v10m6-2a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4" /></svg>
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
						</div>
					</div>

					<div v-if="header.type == 'date'" class="columnHeaderWrapper">
						<div>
							<input type="date" v-model="filters[index].start" @input="filter()">
							-
							<input type="date" v-model="filters[index].end" @input="filter()">
							<svg @click="openFilter = index" xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6V4m0 2a2 2 0 100 4m0-4a2 2 0 110 4m-6 8a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4m6 6v10m6-2a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4" /></svg>
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
							<svg @click="openFilter = index" xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6V4m0 2a2 2 0 100 4m0-4a2 2 0 110 4m-6 8a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4m6 6v10m6-2a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4" /></svg>
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
							<svg @click="openFilter = index" xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6V4m0 2a2 2 0 100 4m0-4a2 2 0 110 4m-6 8a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4m6 6v10m6-2a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4" /></svg>
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
			<tr v-for="(row, index) in rowsForDisplay" :key="index">
				<td><input type="checkbox" v-model="selectedRows[index]"></td>
				<td v-for="(cell, index) in row" :key="index" @click="$emit('rowClick', row)">{{cell}}</td>
			</tr>
		</tbody>
	</table>
</template>

<script>
export default {
	data: () => ({
		rows: [],
		sorted: [],
		filters: [],
		rowsForDisplay: [],
		selectedRows: [],
		openFilter: null,
		allRowsSelected: false
	}),

	props: {
		tableData: Object,
	},

	fetch() {
		this.rows = this.tableData.rows;
		this.rowsForDisplay = this.rows;

		this.tableData.columns.forEach(c => {
			this.filters.push({
				type: c.type,
				option: c.type == "choice" ? "is" : c.type == "date" ? "between" : c.type == "number" ? "exact" : c.type == "string" ? "contains" : null,
				empty: c.type == "string" ? "anything" : undefined
			});
		});
		
		this.fillSelectedRows();
	},

	watch: {
		tableData() {
			this.rows = this.tableData.rows;
			this.rowsForDisplay = this.rows;
		}
	},

	methods: {
		fillSelectedRows() {
			this.selectedRows = [];
			this.rowsForDisplay.forEach(() => this.selectedRows.push(false));
		},

		selectAllRows() {
			this.allRowsSelected = !this.allRowsSelected;
			this.selectedRows = this.selectedRows.map(() => this.allRowsSelected);
		},

		sort(i) {
			this.fillSelectedRows();
			if(this.sorted[i]) {
				if(this.sorted[i] == "asc") {
					this.sorted[i] = "desc";
				} else {
					this.sorted[i] = "asc";
				}
			} else {
				this.sorted[i] = "asc";
			}

			const isNumber = this.rows.filter(x => !Number.isNaN(parseInt(x[i]))).length > 0 ? true : false;			
			const reverseSort = this.sorted[i] == "asc" ? true : false;

			if(isNumber) {
				this.sortNumberColumn(i, reverseSort);
			} else {
				this.sortStringColumn(i, reverseSort);
			}
		},

		sortNumberColumn(i, asc) {
			this.rows.sort((a, b) => {
				return asc ? parseInt(b[i]) - parseInt(a[i]) : parseInt(a[i]) - parseInt(b[i]);
			});
		},

		sortStringColumn(i, asc) {
			this.rows.sort((a, b) => {
				if(a[i] > b[i]) {
					return asc ? 1 : -1;
				} else if(a[i] < b[i]) {
					return asc ? -1 : 1;
				} else {
					return 0;
				}
			});
		},

		filter() {
			this.fillSelectedRows();
			this.rowsForDisplay = this.rows;
			for(let i = 0; i < this.filters.length; i++) {
				if(this.filters[i].type == "choice" && this.filters[i].value) {
					if(this.filters[i].option == "is") {
						this.rowsForDisplay = this.rowsForDisplay.filter(x => x[i].toLowerCase() === this.filters[i].value.toLowerCase());
					}
					if(this.filters[i].option == "isnt") {
						this.rowsForDisplay = this.rowsForDisplay.filter(x => x[i].toLowerCase() !== this.filters[i].value.toLowerCase());
					}
				}

				if(this.filters[i].type == "date" && this.filters[i].start && this.filters[i].end) {
					if(this.filters[i].option == "between") {
						this.rowsForDisplay = this.rowsForDisplay.filter(x => Date.parse(x[i]) > Date.parse(this.filters[i].start) && Date.parse(x[i]) < Date.parse(this.filters[i].end));
					}
					if(this.filters[i].option == "outside") {
						this.rowsForDisplay = this.rowsForDisplay.filter(x => Date.parse(x[i]) < Date.parse(this.filters[i].start) || Date.parse(x[i]) > Date.parse(this.filters[i].end));
					}
				}

				if(this.filters[i].type == "number" && this.filters[i].value) {
					if(this.filters[i].option == "exact") {
						this.rowsForDisplay = this.rowsForDisplay.filter(x => parseFloat(x[i]) === parseFloat(this.filters[i].value));
					}
					if(this.filters[i].option == "less") {
						this.rowsForDisplay = this.rowsForDisplay.filter(x => parseFloat(x[i]) < parseFloat(this.filters[i].value));
					}
					if(this.filters[i].option == "more") {
						this.rowsForDisplay = this.rowsForDisplay.filter(x => parseFloat(x[i]) > parseFloat(this.filters[i].value));
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
							this.rowsForDisplay = this.rowsForDisplay.filter(x => x[i].toLowerCase().includes(this.filters[i].value.toLowerCase()));
						}
						if(this.filters[i].option == "exact") {
							this.rowsForDisplay = this.rowsForDisplay.filter(x => x[i].toLowerCase() === this.filters[i].value.toLowerCase());
						}
						if(this.filters[i].option == "begins") {
							this.rowsForDisplay = this.rowsForDisplay.filter(x => x[i].toLowerCase().startsWith(this.filters[i].value.toLowerCase()));
						}
						if(this.filters[i].option == "ends") {
							this.rowsForDisplay = this.rowsForDisplay.filter(x => x[i].toLowerCase().endsWith(this.filters[i].value.toLowerCase()));
						}
						if(this.filters[i].option == "doesntcontain") {
							this.rowsForDisplay = this.rowsForDisplay.filter(x => !x[i].toLowerCase().includes(this.filters[i].value.toLowerCase()));
						}
					}
				}
			}
		}
	},
	watch: {
		selectedRows() {
			let selectedRowContents = [];
			this.selectedRows.forEach((selected, i) => {
				if(selected) selectedRowContents.push(this.rowsForDisplay[i]);
			});
			this.$emit("rowSelect", selectedRowContents);
		}
	}
}
</script>

<style lang="sass" scoped>
@import "assets/_vars.sass"

td
	white-space: break-spaces

tr
	cursor: pointer
	&:hover td
		transition-duration: 0s
		background: $dark

th
	padding-bottom: 4px
	input, select
		max-width: 100px

div.columnHeaderWrapper
	display: flex
	flex-direction: column
	align-items: center

svg
	margin-bottom: -6px
	height: 24px
	&:hover
		color: $heavy

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
</style>