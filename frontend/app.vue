<template>
	<main>
		<nav>
			<div id="navcontainer">
				<h1>TxTs Treasury</h1>
				<ul v-if="loggedIn">
					<li ref="dashboard" @click="currentRoute = 'dashboard'"><NuxtLink to="/" :class="currentRoute == 'dashboard' ? 'active' : ''">Dashboard</NuxtLink></li>
					<li ref="transactions" @click="currentRoute = 'transactions'"><NuxtLink to="/transactions" :class="currentRoute == 'transactions' ? 'active' : ''">Transactions</NuxtLink></li>
					<li ref="assets" @click="currentRoute = 'assets'"><NuxtLink to="/assets" :class="currentRoute == 'assets' ? 'active' : ''">Assets</NuxtLink></li>
					<li ref="accounts" @click="currentRoute = 'accounts'"><NuxtLink to="/accounts" :class="currentRoute == 'accounts' ? 'active' : ''">Accounts</NuxtLink></li>
					<li ref="recipients" @click="currentRoute = 'recipients'"><NuxtLink to="/recipients" :class="currentRoute == 'recipients' ? 'active' : ''">Recipients</NuxtLink></li>
					<li ref="tags" @click="currentRoute = 'tags'"><NuxtLink to="/tags" :class="currentRoute == 'tags' ? 'active' : ''">Tags</NuxtLink></li>
					<li ref="currencies" @click="currentRoute = 'currencies'"><NuxtLink to="/currencies" :class="currentRoute == 'currencies' ? 'active' : ''">Currencies</NuxtLink></li>
					<br><br>
					<li ref="settings" @click="currentRoute = 'settings'"><NuxtLink to="/settings" :class="currentRoute == 'settings' ? 'active' : ''">Settings</NuxtLink></li>
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
const loggedIn = document.cookie.includes("accessToken");

if(!loggedIn) useRouter().replace("/login");

function logout() {
	document.cookie = "accessToken=;expires=Thu, 01 Jan 1970 00:00:00 GMT";
	location.reload();
}
</script>

<script lang="ts">
export default {
	data: () => ({
		currentRoute: useRoute().path.split("/")[1] ? useRoute().path.split("/")[1] : "dashboard",
	})
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