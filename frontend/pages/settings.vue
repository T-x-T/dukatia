<template>
	<Head>
		<Title>Dukatia - Settings</Title>
	</Head>

	<div id="main">
		<div class="gridItem">
			<h3>Update Password</h3>
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
			<h3>Select Theme</h3>
			<select v-model="$colorMode.preference">
				<option value="dark">Dark</option>
				<option value="light">Light</option>
			</select>
		</div>

		<div class="gridItem">
			<BatchImport />
		</div>

		<div v-if="me.superuser" class="gridItem" id="userList">
			<UserList />
		</div>

		<div class="gridItem">
			<h3>Build info</h3>
			<p>Version: {{ $config.public.version }}</p>
			<p>Branch: {{ $config.public.branch }}</p>
			<p>Commit: {{ $config.public.commit }}</p>
		</div>
	</div>
</template>

<script lang="ts">
export default {
	data: () => ({
		oldPassword: null,
		newPassword: null,
		newPasswordConfirmation: null,
		passwordUpdateMessage: "",
		me: {} as User,
	}),

	async mounted() {
		this.me = await $fetch("/api/v1/users/me");
	},

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
				this.passwordUpdateMessage = "password updated successfully";
			} catch(e: any) {
				this.passwordUpdateMessage = e?.data?.error;
			}
		},
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

div#userList
	max-width: 750px
</style>