<template>
	<main>
		<nav>
			<ThemeSelector />
		</nav>
		<div id="left">
			<NuxtLink to="/">
				<h1>Dukatia</h1>
			</NuxtLink>
			<ContentNavigation :navigation-tree="navigation"/>
		</div>
		<article>
			<ContentDoc />
		</article>
	</main>
</template>

<style lang="sass" scoped>
main
	display: grid
	grid-template-columns: 1fr 1000px 1fr
	grid-template-rows: 3em 1fr
	height: 100vh

nav
	grid-area: 1 / 1 / 1 / row-end

article
	grid-area: 2 / 2 / 3 / 3
	overflow-y: scroll
	text-align: justify
	padding: 0 20px 100px 20px

div#left
	justify-self: end
	padding-right: 20px
	grid-area: 2 / 1 / 3 / 2
	h1
		font-size: 3em
		cursor: pointer
		user-select: none
		text-shadow: 4px 4px 8px black
		text-align: center
		&:hover
			transform: scale(1.05) rotate(1deg)

:deep(pre)
	padding: 8px
	background-color: #1e1e1e

:deep(code)
	opacity: 0.8

:deep(table)
	padding: 20px 0 20px 0

</style>

<script setup>
	const { data: navigation } = await useAsyncData('navigation', () => {
		return fetchContentNavigation()
	});
</script>
	