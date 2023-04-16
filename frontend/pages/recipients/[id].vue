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
			const recipients: any = await $fetch("/api/v1/recipients/all");
			const id = Number(useRoute().path.split("/")[2]);
			const recipientFromStore = recipients.filter((x: any) => x.id == id)[0];
			this.recipientData = {...recipientFromStore};
		}
	}
}
</script>