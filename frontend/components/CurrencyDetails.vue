<template>
	<div>
		<CurrencyForm
			:data="currency"
			@back="$emit('back')"
		/>
	</div>	
</template>

<script lang="ts">
export default {
	emits: ["back"],

	props: {
		currency: {
			type: Object as PropType<Currency>,
			required: false,
		}
	},
}
</script>