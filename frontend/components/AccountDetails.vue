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
		account: {
			type: Object as PropType<Account>,
			required: true,
		}
	},

	async created() {
		this.account.tag_ids = Array.isArray(this.account.tag_ids) ? [...this.account.tag_ids] : [];

		this.config = {
			...this.$detailPageConfig().account,
			data: this.account,
			reset_default_currency_id: true,
		}
	}
}
</script>