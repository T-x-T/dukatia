<template>
	<div>
		<RecipientDetails 
			:recipient="recipientData"
			v-on:back="$router.push('/recipients')"
		/>		
	</div>
</template>

<script>
export default {
	data: () => ({
		recipientData: {}
	}),

	async fetch() {
		if(this.$route.path.split("/")[2] == "new") {
			this.recipientData = {
				id: "",
				name: ""
			};
		} else {
			const id = Number(this.$route.path.split("/")[2]);
			const recipientFromStore = this.$store.state.recipients.filter(x => x.id == id)[0];
			this.recipientData = {...recipientFromStore};
		}
	}
}
</script>