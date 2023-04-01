<template>
	<div id="main">
		<AccountDetails
			:account="accountData"
			v-on:back="$router.push('/accounts')"
		/>
	</div>
</template>

<script>
export default {
	data: () => ({
		accountData: {}
	}),

	async fetch() {
		if(this.$route.path.split("/")[2] == "new") {
			this.accountData = {
				id: "",
				name: "",
				default_currency: this.$store.state.currencies.filter(x => x.id == 0)[0]
			};
		} else {
			const id = Number(this.$route.path.split("/")[2]);
			const accountFromStore = this.$store.state.accounts.filter(x => x.id == id)[0];
			const default_currency = this.$store.state.currencies.filter(x => x.id == accountFromStore.default_currency)[0];
			this.accountData = {...accountFromStore, default_currency: {...default_currency}};
		}
	}
}
</script>