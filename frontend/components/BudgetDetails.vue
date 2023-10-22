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
		config: {} as DetailFormConfig,
	}),

	props: {
		budget: {
			type: Object as PropType<Budget>,
			required: true,
		}
	},

	async created() {
		this.budget.filter_tag_ids = Array.isArray(this.budget.filter_tag_ids) ? [...this.budget.filter_tag_ids] : []

		this.config = {
			...this.$detailPageConfig().budget,
			data: {
				...this.budget,
				active_from: new Date(new Date(this.budget.active_from).valueOf() - (new Date(this.budget.active_from).getTimezoneOffset() * 60000)).toISOString().slice(0, -8),
				active_to: this.budget.active_to ? new Date(new Date(this.budget.active_to).valueOf() - (new Date(this.budget.active_to).getTimezoneOffset() * 60000)).toISOString().slice(0, -8) : null,
			},
		}
	}
}
</script>