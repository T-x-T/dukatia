<template>
	<div>
		<RecipientDetails 
			v-if="Object.keys(recipientData).length > 0"
			:recipient="recipientData"
			v-on:back="useRouter().push('/recipients')"
		/>		
	</div>
</template>

<script lang="ts">
export default {
	data: () => ({
		recipientData: {} as Recipient
	}),

	async created() {
		if(useRoute().path.split("/")[2] == "new") {
			this.recipientData = {
				name: ""
			};
		} else {
			const id = Number(useRoute().path.split("/")[2]);
			this.recipientData = await $fetch(`/api/v1/recipients/${id}`);
		}
	}
}
</script>