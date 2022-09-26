<template>
	<div id="wrapper">
		<div id="input" @click="toggleDropdown()" @keypress="(e) => {if(e.keyCode == 32) toggleDropdown()}">
			<label for="thething">{{this.selectData.label}}</label>
			<input id="thething" type="text" v-model="displayText" readonly>
			<svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" /></svg>
		</div>
		<div id="clickTarget" v-if="dropdown" @click="toggleDropdown()"></div>
		<div id="dropdown" v-if="dropdown">
			<input type="text" id="dropdownSearch" placeholder="filter" ref="dropdownSearch" v-model="searchTerm" @keydown="keypressDropdownInput">
			<ul>
				<li v-for="(item, index) in sortedSelectData.options" :key="index" class="listItem" @click="toggleOption(item.id)">
					<input class="checkbox" type="checkbox" v-model="optionStates[item.id]" tabindex="-1" :ref="'dropdown' + index" :id="index" @focusout="focusOutDropdown" @keydown="keypressDropdownInput">
					<span>{{item.name}}</span>
				</li>
			</ul>
		</div>
	</div>
</template>

<script>
export default {
	data: () => ({
		displayText: "",
		dropdown: false,
		sortedSelectData: {},
		filteredSelectData: null,
		optionStates: [],
		searchTerm: ""
	}),

	props: {
		selectData: Object
	},

	mounted() {
		this.updateSelectData();
	},

	methods: {
		updateSelectData() {
			this.sortedSelectData = this.filteredSelectData ? this.filteredSelectData : this.selectData;
			this.sortedSelectData.options.sort((a, b) => this.sortStrings(a.name, b.name));

			this.optionStates = [];
			if(this.selectData.selected) {
				this.selectData.selected.forEach(x => this.optionStates[x] = true);
			}
			this.updateDisplayText();
		},

		toggleDropdown() {
			this.dropdown = !this.dropdown;
			this.$nextTick(() => this.$refs["dropdownSearch"]?.focus());
			this.updateDisplayText();
		},

		closeDropdown() {
			this.dropdown = false;
		},

		openDropdown() {
			this.dropdown = true;
		},

		focusOutDropdown(e) {
			if(e.relatedTarget?.parentNode?.className != "listItem" && e.relatedTarget !== null) return this.closeDropdown();
			if(e.relatedTarget === null) this.$nextTick(() => focus(e.target));
		},

		keypressDropdownInput(e) {
			if(e.keyCode == 40) { //Down
				e.preventDefault();
				if(Number(e.target.id) + 1 > Object.keys(this.$refs).filter(x => x.startsWith("dropdown")).length - 1 || e.target.id == "dropdownSearch") {
					this.$refs["dropdown0"]?.[0]?.focus();
				} else {
					this.$refs["dropdown" + (Number(e.target.id) + 1)]?.[0]?.focus();
				}
			} else if(e.keyCode == 38) { //Up
				e.preventDefault();
				if(Number(e.target.id) - 1 < 0) {
					this.$refs["dropdown" + (Object.keys(this.$refs).filter(x => x.startsWith("dropdown")).length - 1)]?.[0]?.focus();
				} else {
					this.$refs["dropdown" + (Number(e.target.id) - 1)]?.[0]?.focus();
				}
			}
		},

		toggleOption(id) {
			let optionStates = this.optionStates;
			optionStates[id] = !optionStates[id];
			this.optionStates = null;
			this.optionStates = optionStates;
			this.$emit("update", this.optionStates.map((x, i) => this.selectData.options.filter(y => x && y.id === i)[0]?.id).filter(x => typeof x == "number"));
			this.updateDisplayText();
		},

		updateDisplayText() {
			this.displayText = "";
			this.selectData.options.forEach((x, i) => {
				if(this.optionStates[x.id]) {
					this.displayText += x.name;
					this.displayText += ", ";	
				}
			});
			if(this.displayText) this.displayText = this.displayText.slice(0, this.displayText.length - 2);
		},

		sortStrings(a, b) {
			if(a.toLowerCase() > b.toLowerCase()) return 1;
			if(a.toLowerCase() < b.toLowerCase()) return -1;
			return 0;
		},

		applyFilter() {
			this.filteredSelectData = {...this.selectData};
			this.filteredSelectData.options = this.selectData.options.filter(x => 
				x.name.toLowerCase().includes(this.searchTerm.toLowerCase())
			);
			this.updateSelectData();
		}
	},

	watch: {
		selectData() {
			this.updateSelectData();
		},
		searchTerm() {
			this.applyFilter();
		}
	}
}
</script>

<style lang="sass" scoped>
@import "assets/_vars.sass"

#wrapper
	width: fit-content
	display: inline-block	

#input
	svg
		display: inline
		height: 32px
		position: absolute
		margin-left: -35px
		&:hover
			cursor: pointer

#dropdown
	background: rgba(0, 0, 0, 0.5)
	backdrop-filter: blur(5px) saturate(20%)
	box-shadow: 0px 0px 15px black
	min-width: 200px
	position: absolute
	z-index: 10
	padding: 4px 40px 4px 4px
	margin: 0px 0px 0px 4px
	li
		cursor: pointer
		user-select: none
	input.checkbox
		cursor: pointer
	p
		padding: 1px 2px 1px 2px
	span
		@extend .semibold

#clickTarget
	position: fixed
	top: 0px
	left: 0px
	width: 100vw
	height: 100vh
	z-index: 5
</style>