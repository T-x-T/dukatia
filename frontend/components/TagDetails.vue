<template>
	<div id="wrapper">
		<DetailsPage
			v-if="config"
			:config="config"
			v-on:back="$emit('back')"
		/>
	</div>
</template>

<script lang="ts">
export default {
	data: () => ({
		config: null
	}),

	props: {
		tag: Object
	},

	created() {
		(this as any).config = {
			...this.$detailPageConfig().tags,
			data: this.tag
		}
	}
}
</script>