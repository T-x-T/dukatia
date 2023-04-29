<template>
	<div id="wrapper">
		<DetailsPage
			v-if="Object.keys(config).length > 0"
			:config="(config as DetailFormConfig)"
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
		currency: {
			type: Object as PropType<Currency>,
			required: true,
		}
	},

	created() {
		this.config = {
			...this.$detailPageConfig().currency,
			data: this.currency
		}
	}
}
</script>