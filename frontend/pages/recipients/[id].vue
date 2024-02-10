<template>
	<div>
		<RecipientDetails 
			v-if="loaded"
			:recipient="recipientData"
			@back="useRouter().push('/recipients')"
		/>		
	</div>
</template>

<script lang="ts">
export default {
	data: () => ({
		recipientData: {} as Recipient,
		loaded: false,
	}),

	async created() {
		if(useRoute().path.split("/")[2] == "new") {
			this.loaded = true;
		} else {
			const id = useRoute().path.split("/")[2];
			this.recipientData = await $fetch(`/api/v1/recipients/${id}`);
			this.loaded = true;
		}
	}
}
</script>