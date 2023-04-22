<template>
	<div id="wrapper">
		<DetailsPage
			v-if="Object.keys(config).length > 0"
			:config="config"
			v-on:back="$emit('back')"
		/>
	</div>
</template>

<script lang="ts">
export default {
	data: () => ({
		config: {} as DetailFormConfig
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