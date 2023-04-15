<template>
	<div id="main">
		<div class="gridItem">
			<p>Update Password:</p>
			<input type="password" v-model="oldPassword" placeholder="Old Password">
			<br>
			<input type="password" v-model="newPassword" placeholder="New Password">
			<br>
			<input type="password" v-model="newPasswordConfirmation" placeholder="Confirm new Password">
			<br>
			<button class="green" @click="updatePassword()">Update</button>
			<p v-if="passwordUpdateMessage">{{passwordUpdateMessage}}</p>
		</div>

		<div class="gridItem">
			<p>Select Theme</p>
			<select v-model="$colorMode.preference">
				<option value="dark">Dark</option>
				<option value="light">Light</option>
				<option value="monochrome">Monochrome</option>
				<option value="black">Black</option>
				<option>None</option>
			</select>
		</div>
	</div>
</template>

<script lang="ts">
export default {
	data: () => ({
		oldPassword: null,
		newPassword: null,
		newPasswordConfirmation: null,
		passwordUpdateMessage: ""
	}),

	methods: {
		async updatePassword() {
			this.passwordUpdateMessage = "";

			if(this.newPassword !== this.newPasswordConfirmation) {
				this.passwordUpdateMessage = "The entered new passwords don't match";
				return;
			}

			try {
				await $fetch("/api/v1/users/me/secret", {
					method: "PUT",
					body: {
						old_secret: this.oldPassword,
						new_secret: this.newPassword 
					}
				});
			} catch(e: any) {
				this.passwordUpdateMessage = e?.data?.data?.error;
			}
		}
	}
}
</script>

<style lang="sass" scoped>
div#main
	height: 100vh
	display: flex

div.gridItem
	width: max-content
	height: max-content
	padding: 5px
	margin: 10px
	box-shadow: 4px 4px 0px black
</style>