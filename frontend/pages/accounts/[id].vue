<template>
	<div>
		<AccountDetails
			v-if="Object.keys(accountData).length > 0"
			:account="accountData"
			v-on:back="useRouter().push('/accounts')"
		/>
	</div>
</template>

<script lang="ts">
export default {
	data: () => ({
		accountData: {} as Account,
	}),

	async created() {
		const isNew = useRoute().path.split("/")[2] == "new";
		if(isNew) {
			this.accountData = {
				name: "",
				default_currency_id: 0,
				tag_ids: []
			};
		} else {
			const id = Number(useRoute().path.split("/")[2]);
			this.accountData = await $fetch(`/api/v1/accounts/${id}`) as Account;
		}
	}
}
</script>