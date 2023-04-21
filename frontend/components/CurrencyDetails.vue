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