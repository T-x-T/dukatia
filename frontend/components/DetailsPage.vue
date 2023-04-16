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

		<div v-if="(config as any).tableData" id="table">
			<CustomTable
				:tableData="(config as any).tableData"
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
		config: Object
	}
}
</script>

<style lang="sass" scoped>
div#table
	margin-top: 20px
</style>