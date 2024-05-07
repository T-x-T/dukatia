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