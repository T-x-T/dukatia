<template>
	<ul>
		<li v-for="(item, index) in navigationTree" :key="index">
			<NuxtLink v-if="item.title !== parentTitle" :to="item._path">
				{{ item.title }}
			</NuxtLink>
			<ContentNavigation v-if="item.children" :navigationTree="item.children" :parentTitle="item.title" class="subNavigation"/>
		</li>
	</ul>
</template>

<script setup>
	defineProps({
		navigationTree: {
			type: Array,
			default: () => []
		},
		parentTitle: {
			type: String,
			default: ""
		}
	})
</script>

<style lang="sass" scoped>
@import "assets/_vars.sass"
.subNavigation
	margin-left: 1em

html.dark-mode	
	a
		color: $dark-brightest
		&:hover
			color:$dark-heavydark
		&.router-link-active
			color:$dark-heavydark
html.light-mode	
	a
		color: black
		&:hover
			color:$light-heavy
		&.router-link-active
			color:$light-heavy
html.monochrome-mode	
	a
		color: $monochrome-brightest
		&:hover
			color:$monochrome-heavydark
		&.router-link-active
			color:$monochrome-heavydark

html.black-mode	
	a
		color: $black-brightest
		&:hover
			color:$black-brighter
		&.router-link-active
			color:$black-brighter
</style>