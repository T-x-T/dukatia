<template>
	<div id="container">
		<h1>Demo</h1>
		<p>The live demo allows you try Dukatia without having to install anything!</p>
		<p>Every hour the demo site gets reset. Also please don't enter any sensitive information.</p>
		<button class="important" @click="redirect()">Let's go!</button>
	</div>
</template>

<style lang="sass" scoped>
div#container
	display: flex
	flex-direction: column
	align-items: center

h1
	margin: 25px
	font-size: 5em
	text-align: center

p
	text-align: center
	font-size: 1.5em

button
	margin: 25px 0 50vh 0
	width: fit-content
</style>

<script setup lang="ts">
const redirect= () => {
	window.location.href="https://demo_account_creator.dukatia.com";
};

useSeoMeta({
  title: "Dukatia - Demo",
  ogTitle: "Dukatia",
  description: "Dukatia is your finance planner. Track your spending, manage your assets and set smarter budgets. Run it in Docker and access it from any web browser!",
  ogDescription: "Dukatia is your finance planner. Track your spending, manage your assets and set smarter budgets. Run it in Docker and access it from any web browser!",
})
</script>