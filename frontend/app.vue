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
			<div id="innerContent">
				<NuxtPage />
			</div>
		</div>
	</main>
</template>

<script lang="ts" setup>

let currentRoute = "dashboard";
let loggedIn = true;

try {
	//await useMainStore().fetchAll();
} catch(e: any) {
	if(e.response?.data?.error == "cookie accessToken not set") {
		console.info("not logged in, redirecting to login page");
		useRouter().replace("/login");
	} else {
		console.error(e.response);
	}
}

function logout() {
	document.cookie = "accessToken=;expires=Thu, 01 Jan 1970 00:00:00 GMT";
	location.reload();
}
</script>

<script lang="ts">
export default {
	async mounted() {
		if(!document.cookie.includes("accessToken")) {
			loggedIn = false;
			await useRouter().replace("/login");
		}

		//currentRoute = this.$route.path.replace("/", "");
		(this as any).$refs[(this as any).currentRoute]?.firstChild.classList.add("active");
	},

	watch: {
		currentRoute(newRoute, oldRoute) {
			(this.$refs[newRoute] as any)?.firstChild.classList.add("active");
			(this.$refs[oldRoute] as any)?.firstChild.classList.remove("active");
		}
	}
}
</script>

<style lang="sass" scoped>
@import "assets/_vars.sass"

main
	display: flex
	height: 100vh
	width: 100vw
	overflow: hidden

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

li
	width: fit-content
	&:hover
		transform: scale(1.1)

#content
	flex-grow: 1
	width: 1rem
	height: 100vh
	overflow: auto

#innerContent
	margin: 10px 10px 10px 10px
</style>