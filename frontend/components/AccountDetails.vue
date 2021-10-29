<template>
	<div id="wrapper">
		<button @click="$emit('back')">Back</button>

		<div id="details">
			<label for="id">ID:</label>
			<input type="text" id="id" disabled v-model="account.id">
			<br>
			<label for="name">Name:</label>
			<input type="text" id="name" v-model="account.name">
			<br>
			<label for="currency">Currency:</label>
			<select id="currency" v-model="account.defaultCurrency.id">
				<option v-for="(currency, index) in $store.state.currencies" :key="index" :value="currency.id">{{currency.name}}</option>
			</select>
			<br>
			<button class="green" @click="sendAccount">Save</button>
			<button class="red" @click="$emit('back')">Cancel</button>
		</div>	
	</div>	
</template>

<script>
export default {
	props: {
		account: Object
	},

	methods: {
		async sendAccount() {
			const accountData = {
				name: this.account.name,
				defaultCurrency: this.account.defaultCurrency.id
			}
			if(this.account.id) {
				await this.$axios.$put(`/api/v1/accounts/${this.account.id}`, accountData);
			} else {
				await this.$axios.$post("/api/v1/accounts", accountData);
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