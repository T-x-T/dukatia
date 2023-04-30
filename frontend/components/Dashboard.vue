<template>
	<div>
		<h2>{{ dashboard_data.name }}</h2>
		<p v-if="dashboard_data.description">{{ dashboard_data.description }}</p>
	</div>
</template>

<script lang="ts">
export default {
	props: {
		dashboard_data: {
			type: Object as PropType<Dashboard>,
			required: true,
		}
	}
}
</script>