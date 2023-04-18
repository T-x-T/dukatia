<template>
	<div>
		<RecipientDetails 
			:recipient="recipientData"
			v-on:back="useRouter().push('/recipients')"
		/>		
	</div>
</template>

<script lang="ts">
export default {
	data: () => ({
		recipientData: {}
	}),

	async created() {
		if(useRoute().path.split("/")[2] == "new") {
			this.recipientData = {
				id: "",
				name: ""
			};
		} else {
			const recipients = await $fetch("/api/v1/recipients/all") as Recipient[];
			const id = Number(useRoute().path.split("/")[2]);
			const recipientFromStore = recipients.filter(x => x.id == id)[0];
			this.recipientData = {...recipientFromStore};
		}
	}
}
</script>