<template>
	<div>
		<AccountDetails
			v-if="accountData"
			:account="accountData"
			v-on:back="useRouter().push('/accounts')"
		/>
	</div>
</template>

<script lang="ts">
export default {
	data: () => ({
		currencies: [] as Currency[],
		accounts: [] as Account[],
		accountData: null as Account | null,
	}),

	async created() {
		this.currencies = await $fetch("/api/v1/currencies/all");
		this.accounts = await $fetch("/api/v1/accounts/all");

		const isNew = useRoute().path.split("/")[2] == "new";
		if(isNew) {
			this.accountData = {
				name: "",
				default_currency: this.currencies.filter(x => x.id == 0)[0],
				tag_ids: []
			};
		} else {
			const id = Number(useRoute().path.split("/")[2]);
			const accountFromStore = this.accounts.filter(x => x.id == id)[0];
			const default_currency = this.currencies.filter(x => x.id == accountFromStore.default_currency)[0];
			this.accountData = {...accountFromStore, default_currency: {...default_currency}};
		}
	}
}
</script>