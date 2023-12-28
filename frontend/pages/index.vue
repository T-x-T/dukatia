<template>
	<div>
		<Dashboard 
			:dashboard_data="dashboards[0]"
		/>
	</div>
</template>

<script lang="ts" setup>
const dashboards = (await useFetch("/api/v1/users/me/dashboards")).data.value;
</script>