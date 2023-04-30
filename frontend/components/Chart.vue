<template>
	<div>
		<p>{{ chart_options.title }}</p>
		<p>{{ chart_data?.text }}</p>
	</div>
</template>

<script lang="ts">
export default {
	data: () => ({
		chart_data: {} as any
	}),

	props: {
		chart_options: {
			type: Object as PropType<ChartOptions>,
			required: true,
		}
	},

	async mounted() {
		this.chart_data = await $fetch(`/api/v1/charts/${this.chart_options.id}/data`);
	}
}
</script>

<style lang="sass" scoped>
p
	white-space: pre-line
</style>