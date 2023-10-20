<template>
	<div>
		<BudgetDetails 
			v-if="Object.keys(budgetData).length > 0"
			:budget="budgetData"
			v-on:back="useRouter().push('/budgets')"
		/>		
	</div>
</template>

<script lang="ts">
export default {
	data: () => ({
		budgetData: {} as Budget
	}),

	async created() {
		if(useRoute().path.split("/")[2] == "new") {
			this.budgetData = {
				name: "",
				amount: {major: 0, minor: 0, minor_in_major: 100, symbol: "€"},
				rollover: false,
				period: 2,
				filter_tag_ids: [],
				currency_id: 0,
				active_from: new Date(),
			};
		} else {
			const id = Number(useRoute().path.split("/")[2]);
			this.budgetData = await $fetch(`/api/v1/budgets/${id}`);
		}
	}
}
</script>