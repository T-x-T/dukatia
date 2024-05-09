<template>
	<h1>About</h1>
		<div id="me">
			<img id="avatar" src="/avatar.svg" alt="Avatar of TheTxT" />
			<p id="speechbubble">Hi! On the Internet I call myself TxT and I like to develop software in my spare time. Dukatia is a project that I started in 2021 when I finally grew frustrated of my old way to manage my finances - an Excel sheet.</p>
		</div>
</template>

<style lang="sass" scoped>
@import "assets/_vars.sass"

h1
	margin: 125px 0 25px 0
	font-size: 5em
	text-align: center

div#me
	display: flex

img#avatar
	width: 200px
	filter: invert(100)

p#speechbubble
	width: 450px
	font-size: 1.2em
	padding: 25px
	margin-left: 50px
	border-radius: 50px
	border: 5px solid $dark-darker
	&::before
		content:"\A"
		border-style: solid
		border-width: 50px 50px 50px 0
		border-color: transparent $dark-darker transparent transparent
		position: absolute
		margin-left: -80px
		margin-top: 20px
</style>

<script setup lang="ts">
useSeoMeta({
  title: "Dukatia - About",
  ogTitle: "Dukatia",
  description: "Dukatia is your finance planner. Track your spending, manage your assets and set smarter budgets. Run it in Docker and access it from any web browser!",
  ogDescription: "Dukatia is your finance planner. Track your spending, manage your assets and set smarter budgets. Run it in Docker and access it from any web browser!",
})
</script>