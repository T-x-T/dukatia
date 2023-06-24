<template>
	<div id="main">

		<button @click="$emit('back')">Back</button>
		
		<div id="details">
			<DetailsForm
				:config="config"
				v-on:back="$emit('back')"
				v-on:updateData="$emit('updateData')"
			/>
		</div>
	</div>
</template>

<script lang="ts">
export default {
	data: () => ({
		selectData: {}
	}),

	props: {
		config: {
			type: Object as PropType<DetailFormConfig>,
			required: true,
		}
	}
}
</script>