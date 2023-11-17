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
	emits: ["back"],

	props: {
		tag: {
			type: Object as PropType<Tag>,
			required: false,
		}
	},
}
</script>