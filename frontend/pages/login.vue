<template>
	<div id="wrapper">
		<h2>Login</h2>
		<form @submit.prevent="login">
			<label for="username">Username:</label>
			<input type="text" id="username" v-model="username">
			<br>
			<label for="password">Password:</label>
			<input type="password" id="password" v-model="password">
			<br>
			<button type="submit" class="green">Login</button>
			<p>{{error}}</p>
		</form>
	</div>
</template>

<script>
export default {
	data: () => ({
		username: "",
		password: "",
		error: ""
	}),

	methods: {
		async login() {
			try {
				const res = await this.$axios.$post("/api/v1/login", {
					name: this.username,
					secret: this.password
				});
				this.error = "";
				document.cookie =`accessToken=${res.accessToken};SameSite=Strict`;
				await this.$router.replace("/");
				location.reload();
			} catch(e) {
				this.error = e.response.data.error
			}
		}
	}
}
</script>