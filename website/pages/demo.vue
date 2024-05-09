<template>
	<div id="container">
		<h1>Demo</h1>
		<p>The demo is coming soon!</p>
		<p>Until then you could also just try dukatia by running it yourself, for free.</p>
		<button class="important" @click="navigateTo('/docs')">Get started</button>
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
useSeoMeta({
  title: "Dukatia - Demo",
  ogTitle: "Dukatia",
  description: "Dukatia is your finance planner. Track your spending, manage your assets and set smarter budgets. Run it in Docker and access it from any web browser!",
  ogDescription: "Dukatia is your finance planner. Track your spending, manage your assets and set smarter budgets. Run it in Docker and access it from any web browser!",
})
</script>