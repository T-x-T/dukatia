<template>
	<Head>
		<Title>Dukatia - Budgets - {{ budgetData.name }}</Title>
	</Head>

	<div>
		<BudgetDetails 
			v-if="loaded"
			:prop_budget="budgetData"
			@back="useRouter().push('/budgets')"
		/>		
	</div>
</template>

<script lang="ts">
export default {
	data: () => ({
		budgetData: {} as Budget,
		loaded: false,
	}),

	async created() {
		if(useRoute().path.split("/")[2] == "new") {
			this.loaded = true;
		} else {
			const id = useRoute().path.split("/")[2];
			this.budgetData = await $fetch(`/api/v1/budgets/${id}`);
			this.loaded = true;
		}
	}
}
</script>