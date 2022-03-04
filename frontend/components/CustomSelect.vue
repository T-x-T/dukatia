<template>
	<div id="wrapper">
		<div id="input" @click="toggleDropdown()">
			<label for="thething">{{this.selectData.label}}</label>
			<input id="thething" type="text" v-model="displayText" readonly>
			<svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" /></svg>
		</div>
		<div id="clickTarget" v-if="dropdown" @click="toggleDropdown()"></div>
		<div id="dropdown" v-if="dropdown">
			<ul>
				<li v-for="(item, index) in selectData.options" :key="index" @click="toggleOption(item.id)">
					<input type="checkbox" v-model="optionStates[index]">
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
		optionStates: []
	}),

	props: {
		selectData: Object
	},

	mounted() {
		this.updateSelectData();
	},

	methods: {
		updateSelectData() {
			if(this.selectData.selected?.length > 0) {
				this.selectData.selected.forEach(x => this.optionStates[x] = true);
			} else {
				this.optionStates = [];
			}
			this.updateDisplayText();
		},

		toggleDropdown() {
			this.dropdown = !this.dropdown;
			this.updateDisplayText();
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
				if(this.optionStates[i]) {
					this.displayText += x.name;
					this.displayText += ", ";	
				}
			});
			if(this.displayText) this.displayText = this.displayText.slice(0, this.displayText.length - 2);
		}
	},

	watch: {
		selectData() {
			this.updateSelectData();
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
		color: white
		height: 32px
		position: absolute
		margin-left: -35px
		&:hover
			color: $heavy
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
	input
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