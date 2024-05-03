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
.subNavigation
	margin-left: 1em
</style>