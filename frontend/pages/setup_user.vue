<template>
	<div>
		<p>Lets go</p>
		<NuxtLink to="/">Im done here!</NuxtLink>
	</div>	
</template>

<script lang="ts">
export default {
	created() {
		definePageMeta({
			layout: "no-nav"
		})
	},
}
</script>