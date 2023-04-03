<template>
	<div id="wrapper">
		<DetailsPage
			:config="config"
			v-on:back="$emit('back')"
		/>
	</div>
</template>

<script>
export default {
	data: () => ({
		config: {}
	}),

	props: {
		currency: Object
	},

	created() {
		this.config = {
			...this.$detailPageConfig.currency,
			data: this.currency
		}
	}
}
</script>