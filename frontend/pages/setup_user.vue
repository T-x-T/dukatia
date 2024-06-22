<template>
	<div id="wrapper">
		<div id="setup_user_header">
			<div id="no_flex">
				<h2>Welcome to</h2>
				<img v-if="$colorMode.preference == 'dark'" src="/dukatia_logo.svg">
				<img v-else src="/dukatia_logo_white.svg">
			</div>
			<h4>Lets get you set up.</h4>
		</div>

		<div>
			<h4>Select Theme:</h4>
			<select v-model="$colorMode.preference">
				<option value="dark">Dark</option>
				<option value="light">Light</option>
			</select>
		</div>

		<div>
			<h4>Do you want demo data?</h4>
			<button @click="insert_demo_data()">Yes, please!</button>
		</div>

		<button id="done" @click="navigateTo('/')">
			Im done, let's go!
		</button>
	</div>	
</template>

<script lang="ts">
export default {
	created() {
		definePageMeta({
			layout: "no-nav"
		})
	},

	methods: {
		async insert_demo_data() {
			try {
				await $fetch("/api/v1/insert_demo_data", {method: "POST"});
				window.alert("Inserted demo data successfully!");
			} catch(e: any) {
				window.alert(e?.data?.error);
				console.error(e?.data);
			}
		}
	}
}
</script>

<style lang="sass" scoped>
div#setup_user_header
	margin-top: 0 !important
	padding-top: 25px
	padding-bottom: 25px
	width: 100vw
	border-bottom: 2px solid black
	div#no_flex
		display: initial
		h2
			display: inline
		img
			margin-left: 8px
			height: 36px

div#wrapper
	display: flex
	flex-direction: column
	align-items: center
	div
		display: flex
		flex-direction: column
		align-items: center
		*
			margin-top: 0
	*
		margin-top: 10px
	button#done
		margin-top: 30px
</style>