<template>
	<div>
		<AccountDetails
			:account="accountData"
			v-on:back="useRouter().push('/accounts')"
		/>
	</div>
</template>

<script lang="ts" setup>
const currencies: any = (await useFetch("/api/v1/currencies/all")).data.value;
const accounts: any = (await useFetch("/api/v1/accounts/all")).data.value;

const isNew = useRoute().path.split("/")[2] == "new";

let id: any, accountFromStore: any, default_currency: any;
if(!isNew) {
	id = Number(useRoute().path.split("/")[2]);
	accountFromStore = accounts.filter((x: any) => x.id == id)[0];
	default_currency = currencies.filter((x: any) => x.id == accountFromStore.default_currency)[0];
}

const accountData = isNew ? 
	{
		id: "",
		name: "",
		default_currency: currencies.filter((x: any) => x.id == 0)[0]
	} : 
	{...accountFromStore, default_currency: {...default_currency}};
</script>