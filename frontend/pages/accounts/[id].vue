<template>
	<div>
		<AccountDetails
			v-if="loaded"
			:account="accountData"
			@back="useRouter().push('/accounts')"
		/>
	</div>
</template>

<script lang="ts">
export default {
	data: () => ({
		accountData: {} as Account,
		loaded: false,
	}),

	async created() {
		if(useRoute().path.split("/")[2] == "new") {
			this.loaded = true;
		} else {
			const id = Number(useRoute().path.split("/")[2]);
			this.accountData = await $fetch(`/api/v1/accounts/${id}`) as Account;
			this.loaded = true;
		}
	}
}
</script>