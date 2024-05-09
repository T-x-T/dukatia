<template>
	<h1>Privacy Policy</h1>
	<div id="policy">
		<p>Last updated: May 07, 2024</p>
		<br>
		<p>I do not collect any information about you myself. You are free to audit the source code on <a href="https://github.com/T-x-T/dukatia" target="_blank" rel="noopener noreferrer">Github</a> to verify this fact yourself.</p>
		<p>This website however is hosted on cloudflare pages, so their <a href="https://www.cloudflare.com/privacypolicy/" target="_blank" rel="noopener noreferrer">privacy policy</a> also applies.</p>
		<p>The source code and docker images are hosted on Github, so their <a href="https://docs.github.com/en/site-policy/privacy-policies/github-general-privacy-statement" target="_blank" rel="noopener noreferrer">privacy policy</a> applies too.</p>
		<p>If you have any questions, concerns or other inquiries please contact me via E-Mail at <a href="mailto:contact@thetxt.io">contact@thetxt.io</a>.</p>
	</div>
</template>

<style lang="sass" scoped>
h1
	margin: 125px 0 25px 0
	font-size: 5em
	text-align: center

div#policy
	margin: 25px

p, a
	text-align: center

a
	margin: 0
	text-decoration: underline

div#policy
	margin-bottom: 50vh
</style>

<script setup lang="ts">
useSeoMeta({
  title: "Dukatia - Privacy",
  ogTitle: "Dukatia",
  description: "Dukatia is your finance planner. Track your spending, manage your assets and set smarter budgets. Run it in Docker and access it from any web browser!",
  ogDescription: "Dukatia is your finance planner. Track your spending, manage your assets and set smarter budgets. Run it in Docker and access it from any web browser!",
})
</script>