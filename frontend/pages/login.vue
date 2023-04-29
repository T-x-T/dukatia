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

<script lang="ts">
export default {
	data: () => ({
		username: "",
		password: "",
		error: ""
	}),

	methods: {
		async login() {
			try {
				const res: any = await $fetch("/api/v1/login", {
					method: "POST",
					body: {
						name: this.username,
						secret: this.password
					}
				});
				this.error = "";
				document.cookie =`accessToken=${res.accessToken};SameSite=Strict`;
				await useRouter().replace("/");
				location.reload();
			} catch(e: any) {
				this.error = e?.data?.data?.error
			}
		}
	}
}
</script>