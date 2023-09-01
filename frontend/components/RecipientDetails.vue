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
		recipient: {
			type: Object as PropType<Recipient>,
			required: true,
		}
	},

	async created() {
		this.recipient.tag_ids = Array.isArray(this.recipient.tag_ids) ? [...this.recipient.tag_ids] : []

		this.config = {
			...this.$detailPageConfig().recipient,
			data: this.recipient,
		}
	}
}
</script>