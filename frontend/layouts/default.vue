<template>
	<main>
		<SideNav />
		<div id="content">
			<slot />
		</div>
	</main>
</template>

<style lang="sass" scoped>
main
	display: flex
	height: 100svh
	width: 100svw
	overflow: hidden
	@media screen and (max-width: 800px)
		flex-direction: column

#content
	flex-grow: 1
	width: 1rem
	height: 100vh
	overflow: auto
	@media screen and (max-width: 800px)
		height: 1rem
		width: 100vw
</style>