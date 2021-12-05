<template>
	<table>
		<thead>
			<tr>
				<th><input type="checkbox" v-model="allRowsSelected" @click="selectAllRows"></th>
				<th v-for="(header, index) in tableData.headers" :key="index">
					<p @click="sort(index)">{{header}}</p>
					<input v-model="filters[index]" @input="filter()">
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
		allRowsSelected: false
	}),

	props: {
		tableData: Object,
	},

	mounted() {
		this.rows = this.tableData.rows;
		this.rowsForDisplay = this.rows;

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
				if(!this.filters[i]) continue;
				this.rowsForDisplay = this.rowsForDisplay.filter(x => x[i].toString().toLowerCase().includes(this.filters[i].toLowerCase()));
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
	input
		max-width: 100px
</style>