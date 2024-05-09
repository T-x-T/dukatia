<template>
	<h1>About</h1>
		<section id="me">
			<img id="avatar" src="/avatar.svg" alt="Avatar of TheTxT" />
			<p id="speechbubble">Hi! On the Internet I call myself TxT and I like to develop software in my spare time. Dukatia is a project that I started in 2021 when I finally grew frustrated of my old way to manage my finances - an Excel sheet.</p>
		</section>

		<section id="goals">
			<div id="goals">
				<h3>Goals</h3>
				<p>Dukatia aims to...</p>
				<p>… be <span>private</span> and <span>transparent</span></p>
				<p>… be <span>accessible</span> on every device (with a web browser)</p>
				<p>… provide an <span>easy to digest</span> overview over your financial habits</p>
				<p>… be <span>resource efficient</span> and <span>performant</span></p>
			</div>
			<div id="non_goals">
				<h3>Non-Goals</h3>
				<p>Dukatia doesn't try to …</p>
				<p>… completely automate data ingest</p>
				<p>… employ AI/ML systems to analyze your data</p>
				<p>… encrypt all your data (except passwords of course)</p>
				<p>… require deep financial knowledge to fully utilize it</p>
			</div>
		</section>

		<section id="history">
			<h3>History</h3>
			<div id="content">
				<p>
					In 2021, I started Dukatia under the name “TxT's Treasury”. Since then, a lot has happened both in the backend and the frontend.<br>
					If you want to dive into the making of Dukatia, I created a video series to document the development process.
				</p>
				<picture>
					<source srcset="/about_yt_playlist_thumbnail.webp" type="image/webp">
					<source srcset="/about_yt_playlist_thumbnail.jpg" type="image/jpeg"> 
					<img src="/about_yt_playlist_thumbnail.jpg" alt="Youtube Playlist Thumbnail">
				</picture>
			</div>
		</section>
</template>

<style lang="sass" scoped>
@import "assets/_vars.sass"

h1
	margin: 125px 0 25px 0
	font-size: 5em
	text-align: center

h3
	font-size: 4em

p
	font-size: 1.2em

section
	margin-top: 50px

section#me
	display: flex
	justify-content: center

section#goals
	display: flex
	justify-content: space-evenly
	div#goals > h3
		color: $success
	div#non_goals > h3
		color: $error
	span
		color: $success

section#history
	margin: 50px
	h3
		text-align: center
		margin-bottom: 25px
	div#content
		display: grid
		gap: 50px
		grid-template-columns: 1fr 1fr
		img
			width: 100%


img#avatar
	width: 200px
	filter: invert(100)

p#speechbubble
	width: 450px
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