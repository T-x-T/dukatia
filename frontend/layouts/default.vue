<template>
	<main>
		<nav>
			<div id="navcontainer">
				<h1>TxTs Treasury</h1>
				<ul v-if="loggedIn">
					<li ref="dashboard" @click="currentRoute = 'dashboard'"><NuxtLink to="/">Dashboard</NuxtLink></li>
					<li ref="transactions" @click="currentRoute = 'transactions'"><NuxtLink to="/transactions">Transactions</NuxtLink></li>
					<li ref="assets" @click="currentRoute = 'assets'"><NuxtLink to="/assets">Assets</NuxtLink></li>
					<li ref="accounts" @click="currentRoute = 'accounts'"><NuxtLink to="/accounts">Accounts</NuxtLink></li>
					<li ref="recipients" @click="currentRoute = 'recipients'"><NuxtLink to="/recipients">Recipients</NuxtLink></li>
					<li ref="tags" @click="currentRoute = 'tags'"><NuxtLink to="/tags">Tags</NuxtLink></li>
					<li ref="currencies" @click="currentRoute = 'currencies'"><NuxtLink to="/currencies">Currencies</NuxtLink></li>
					<br><br>
					<li ref="settings" @click="currentRoute = 'settings'"><NuxtLink to="/settings">Settings</NuxtLink></li>
					<li @click="logout"><a>Logout</a></li>
				</ul>
			</div>
		</nav>
		<div id="content">
			<Nuxt />
		</div>
	</main>
</template>

<script>
export default {
	data: () => ({
		currentRoute: "dashboard",
		loggedIn: true
	}),

	async fetch() {
		try {
			await this.$store.dispatch("fetchAll");
		} catch(e) {
			if(e.response.data.error == "cookie accessToken not set") {
				console.info("not logged in, redirecting to login page");
				redirect(302, '/login');
			} else {
				console.error(e.response);
			}
		}
	},

	async mounted() {
		if(!document.cookie.includes("accessToken")) {
			this.loggedIn = false;
			this.$router.replace("/login");
		}
		this.currentRoute = this.$route.path.replace("/", "");
		this.$refs[this.currentRoute]?.firstChild.classList.add("active");
	},

	watch: {
		currentRoute(newRoute, oldRoute) {
			this.$refs[newRoute]?.firstChild.classList.add("active");
			this.$refs[oldRoute]?.firstChild.classList.remove("active");
		}
	},

	methods: {
		logout() {
			document.cookie = "accessToken=;expires=Thu, 01 Jan 1970 00:00:00 GMT";
			location.reload();
		}
	}
}
</script>

<style lang="sass" scoped>
@import "assets/_vars.sass"

main
	display: flex
	height: fit-content
	width: 100vw

nav
	width: 21em
	min-height: 100vh
	h1
		margin: 25px
		cursor: pointer
		user-select: none
		text-shadow: 4px 4px 8px black
		&:hover
			transform: scale(1.1) rotate(5deg)
	a
		@extend .semibold
		font-size: 24px
		margin: 10px
		margin-left: 20px
		transition-duration: 0.2s
		cursor: pointer

#navcontainer
	position: fixed
	top: 0

li
	width: fit-content
	&:hover
		transform: scale(1.1)

#content
	flex-grow: 1
	padding: 1%
	width: 1rem
</style>