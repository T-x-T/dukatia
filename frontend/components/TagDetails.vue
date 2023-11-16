<template>
	<div>
		<TagForm
			:data="tag"
			@back="$emit('back')"
		/>
	</div>	
</template>

<script lang="ts">
export default {
	props: {
		tag: {
			type: Object as PropType<Tag>,
			required: true,
		}
	},
}
</script>