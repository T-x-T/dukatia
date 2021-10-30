<template>
	<div id="wrapper">
		<button @click="$emit('back')">Back</button>

		<div id="details">
			<label for="id">ID:</label>
			<input type="text" id="id" disabled v-model="recipient.id">
			<br>
			<label for="name">Name:</label>
			<input type="text" id="name" v-model="recipient.name">
			<br>
			<button class="green" @click="sendRecipient">Save</button>
			<button class="red" @click="$emit('back')">Cancel</button>	
		</div>	
	</div>	
</template>

<script>
export default {
	props: {
		recipient: Object
	},

	methods: {
		async sendRecipient() {
			const recipientData = {
				name: this.recipient.name
			}

			if(this.recipient.id) {
				await this.$axios.$put(`/api/v1/recipients/${this.recipient.id}`, recipientData);
			} else {
				await this.$axios.$post("/api/v1/recipients", recipientData);
			}

			this.$emit("back");
		}
	}
}
</script>

<style lang="sass" scoped>
div#details
	margin: 20px 0px 20px
	button
		margin-top: 10px
</style>