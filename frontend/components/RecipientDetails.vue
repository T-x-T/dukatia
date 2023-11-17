<template>
	<div>
		<RecipientForm
			:data="recipient"
			@back="$emit('back')"
		/>
	</div>	
</template>

<script lang="ts">
export default {
	emits: ["back"],

	props: {
		recipient: {
			type: Object as PropType<Recipient>,
			required: false,
		}
	},
}
</script>