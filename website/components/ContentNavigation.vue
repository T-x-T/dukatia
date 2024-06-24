<template>
	<ul>
		<li v-for="(item, index) in navigationTree" :key="index">
			<span v-if="item.children" @click="item.show_children = !item.show_children">
				<svg v-if="item.show_children || $route.path.startsWith(item._path)" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="m19.5 8.25-7.5 7.5-7.5-7.5" /></svg>
				<svg v-else xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="m8.25 4.5 7.5 7.5-7.5 7.5" /></svg>
			</span>
			<NuxtLink v-if="item.title !== parentTitle" :to="item._path">
				{{ item.title }}
			</NuxtLink>
			<ContentNavigation v-if="item.children && (item.show_children || $route.path.startsWith(item._path))" :navigationTree="item.children" :parentTitle="item.title" class="subNavigation"/>
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
.subNavigation
	margin-left: 1em

svg
	position: relative
	margin-left: -1em
	height: 1em
</style>