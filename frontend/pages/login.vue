<template>
	<Head>
		<Title>Dukatia - Login</Title>
	</Head>

	<div id="background">
		<div id="wrapper" v-if="show_login">
			<a href="https://dukatia.com" target="_blank">
				<img v-if="$colorMode.value == 'dark'" id="logo" src="/dukatia-beta_logo_white.svg" alt="logo">
				<img v-if="$colorMode.value == 'light'" id="logo" src="/dukatia-beta_logo.svg" alt="logo">
			</a>
			<form @submit.prevent="login">
				<input type="text" v-model="username" placeholder="username">
				<input type="password" v-model="password" placeholder="password">
				<button type="submit" class="green">Login</button>
				<p>{{error}}</p>
			</form>
		</div>
	</div>
</template>

<script lang="ts">
export default {
	data: () => ({
		username: "",
		password: "",
		error: "",
		show_login: false,
	}),

	created() {
		definePageMeta({
			layout: "no-nav"
		})
	},

	async mounted() {
		if(this.$route.fullPath.includes("/login?username=")) {
			this.username = this.$route.fullPath.replace("/login?username=", "");
			this.password = "";
			this.login();
		}

		try {
			await $fetch("/api/v1/users/me");
			useRouter().replace("/");
		} catch(e) {
			this.show_login = true;
		}
	},

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
				document.cookie = `access_token=${res.access_token};SameSite=Strict`;
				if (res.first_login) {
					await useRouter().replace("/setup_user");
				} else {
					await useRouter().replace("/");
				}
				location.reload();
			} catch(e: any) {
				this.error = e?.data?.error
			}
		}
	}
}
</script>

<style lang="sass" scoped>
div#background
	width: 100vw
	height: 100vh
	display: flex
	align-items: center
	justify-content: center
	background-image: url("/login_background.webp")
	background-size: cover
	background-repeat: no-repeat
	background-position: center

div#wrapper
	border: 3px solid black
	padding: 3em
	backdrop-filter: blur(5px)

img#logo
	&:hover
		scale: 1.05
		rotate: 5deg
		cursor: pointer

form
	width: 200px
	height: 150px
	margin-top: 1em
	display: flex
	flex-direction: column
	align-items: center
	input, button
		width: 100%
		margin: 0.33em
		padding: 0.5em
	p
		text-align: center
		max-width: 80%
</style>