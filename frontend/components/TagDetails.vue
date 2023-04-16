<template>
	<div id="wrapper">
		<DetailsPage
			:config="config"
			v-on:back="$emit('back')"
		/>
	</div>
</template>

<script lang="ts">
export default {
	data: () => ({
		config: {}
	}),

	props: {
		tag: Object
	},

	created() {
		this.config = {
			...this.$detailPageConfig().tags,
			data: this.tag
		}
	}
}
</script>