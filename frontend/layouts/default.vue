<template>
	<main>
		<nav>
			<h1>TxTs Treasury</h1>
			<ul v-if="loggedIn">
				<li ref="dashboard" @click="currentRoute = 'dashboard'"><NuxtLink to="/">Dashboard</NuxtLink></li>
				<li ref="transactions" @click="currentRoute = 'transactions'"><NuxtLink to="/transactions">Transactions</NuxtLink></li>
				<li ref="accounts" @click="currentRoute = 'accounts'"><NuxtLink to="/accounts">Accounts</NuxtLink></li>
				<li ref="recipients" @click="currentRoute = 'recipients'"><NuxtLink to="/recipients">Recipients</NuxtLink></li>
				<li ref="currencies" @click="currentRoute = 'currencies'"><NuxtLink to="/currencies">Currencies</NuxtLink></li>
				<br><br>
				<li @click="logout"><a>Logout</a></li>
			</ul>
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
		await this.$store.dispatch("fetchAccounts");
		await this.$store.dispatch("fetchCurrencies");
		await this.$store.dispatch("fetchRecipients");
		await this.$store.dispatch("fetchTransactions");
	},

	mounted() {
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
	width: 100vw

nav
	background: $darkest
	width: fit-content
	min-height: 100vh
	align-self: stretch
	flex-shrink: 0
	border-right: 2px solid $heavy
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
	color: $bright
	margin: 10px
	margin-left: 20px
	transition-duration: 0.2s
	cursor: pointer
	&:hover
		color: $heavy

.active
	color: $heavy

li
	width: fit-content
	&:hover
		transform: scale(1.1)

#content
	flex-grow: 1
	padding: 1%
</style>