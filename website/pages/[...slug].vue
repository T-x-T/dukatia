<template>
	<main>
		<div id="left">
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
	grid-template-columns: 1fr min(1000px, 70vw) 1fr
	grid-template-rows: 3em 1fr
	min-height: 80vh
	@media screen and (max-width: 1079px)
		display: initial

article
	grid-area: 2 / 2 / 3 / 3
	text-align: justify
	padding: 10px
	:deep(li)
		list-style-type: initial

div#left
	margin-left: 5em
	padding: 10px
	width: 100%
	justify-self: center
	grid-area: 2 / 1 / 3 / 2

:deep(code)
	font-family: monospace
	font-size: 1.1em
	span
		font-family: monospace
		font-size: 1.1em

	margin-top: 2em
	margin-bottom: 0.25em

:deep(pre)
	padding: 8px
	background-color: #1e1e1e
	overflow: scroll

:deep(code)
	opacity: 0.8

:deep(table)
	margin: 20px 0 20px 0

:deep(th)
	padding: 5px 20px 5px 5px

:deep(td)
	padding: 5px

:deep(a)
	margin: 0
	text-decoration: underline

:deep(p)
	a
		text-decoration: underline

:deep(h1)
	a
		text-decoration: none

:deep(h2)
	a
		text-decoration: none

:deep(h3)
	a
		text-decoration: none

:deep(h4)
	a
		text-decoration: none

:deep(h5)
	a
		text-decoration: none

:deep(h6)
	a
		text-decoration: none

div#left
	:deep(li)
		a
			text-decoration: none

</style>

<script setup>
	const { data: navigation } = await useAsyncData('navigation', () => {
		return fetchContentNavigation();
	});
</script>
	