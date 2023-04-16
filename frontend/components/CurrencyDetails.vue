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
		currency: Object
	},

	created() {
		console.log(this.currency)
		this.config = {
			...this.$detailPageConfig().currency,
			data: this.currency
		}
	}
}
</script>