<template>
	<div>
		<AccountForm
			:data="account"
			@back="$emit('back')"
		/>
	</div>	
</template>

<script lang="ts">
export default {
	emits: ["back"],

	props: {
		account: {
			type: Object as PropType<Account>,
			required: true,
		}
	},
}
</script>