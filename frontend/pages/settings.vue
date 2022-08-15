<template>
	<main>
		<div class="settingGroup">
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

		<div class="settingGroup">
			<p>Select Theme</p>
			<select>
				<option value="dark">Dark</option>
				<option value="light">Light</option>
				<option value="monochrome">Monochrome</option>
				<option value="black">Black</option>
			</select>
		</div>
	</main>
</template>

<script>
export default {
	data: () => ({
		oldPassword: null,
		newPassword: null,
		newPasswordConfirmation: null,
		passwordUpdateMessage: null
	}),

	methods: {
		async updatePassword() {
			this.passwordUpdateMessage = null;

			if(this.newPassword !== this.newPasswordConfirmation) {
				this.passwordUpdateMessage = "The entered new passwords don't match";
				return;
			}

			try {
				await this.$axios.$put("/api/v1/users/me/secret", {
					old_secret: this.oldPassword,
					new_secret: this.newPassword 
				});
			} catch(e) {
				this.passwordUpdateMessage = e.response.data;
			}
		}
	}
}
</script>

<style lang="sass" scoped>
@import "assets/_vars.sass"

div.settingGroup
	background: $darkest
	padding: 5px
	margin: 10px
	box-shadow: 4px 4px 0px black
</style>