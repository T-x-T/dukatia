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
		asset: Object
	},

	created() {
		this.config = {
			...this.$detailPageConfig.asset,
			data: {
				...this.asset,
				value_per_unit: this.asset.value_per_unit / 100, //TODO: use minor_in_mayor
			},
		};
	}
}
</script>