<template>
	<nav v-if="loggedIn" :class="hidden ? 'mobile' : ''">
		<div id="header" v-if="!hidden || !small_device">
			<h1 v-if="!collapsed">TxTs Treasury</h1>
		</div>
		<ul v-if="!hidden || !small_device">
			<li ref="dashboard" @click="change_route('dashboard')">
				<NuxtLink to="/" :class="currentRoute == 'dashboard' ? 'active' : ''">
					<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" d="M3.75 3v11.25A2.25 2.25 0 006 16.5h2.25M3.75 3h-1.5m1.5 0h16.5m0 0h1.5m-1.5 0v11.25A2.25 2.25 0 0118 16.5h-2.25m-7.5 0h7.5m-7.5 0l-1 3m8.5-3l1 3m0 0l.5 1.5m-.5-1.5h-9.5m0 0l-.5 1.5m.75-9l3-3 2.148 2.148A12.061 12.061 0 0116.5 7.605" /></svg>
					<span v-if="!collapsed">Dashboard</span>
				</NuxtLink>
			</li>
			<li ref="transactions" @click="change_route('transactions')">
				<NuxtLink to="/transactions" :class="currentRoute == 'transactions' ? 'active' : ''">
					<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" d="M9 12h3.75M9 15h3.75M9 18h3.75m3 .75H18a2.25 2.25 0 002.25-2.25V6.108c0-1.135-.845-2.098-1.976-2.192a48.424 48.424 0 00-1.123-.08m-5.801 0c-.065.21-.1.433-.1.664 0 .414.336.75.75.75h4.5a.75.75 0 00.75-.75 2.25 2.25 0 00-.1-.664m-5.8 0A2.251 2.251 0 0113.5 2.25H15c1.012 0 1.867.668 2.15 1.586m-5.8 0c-.376.023-.75.05-1.124.08C9.095 4.01 8.25 4.973 8.25 6.108V8.25m0 0H4.875c-.621 0-1.125.504-1.125 1.125v11.25c0 .621.504 1.125 1.125 1.125h9.75c.621 0 1.125-.504 1.125-1.125V9.375c0-.621-.504-1.125-1.125-1.125H8.25zM6.75 12h.008v.008H6.75V12zm0 3h.008v.008H6.75V15zm0 3h.008v.008H6.75V18z" /></svg>
					<span v-if="!collapsed">Transactions</span>
				</NuxtLink>
			</li>
			<li ref="assets" @click="change_route('assets')">
				<NuxtLink to="/assets" :class="currentRoute == 'assets' ? 'active' : ''">
					<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" d="M12 3v17.25m0 0c-1.472 0-2.882.265-4.185.75M12 20.25c1.472 0 2.882.265 4.185.75M18.75 4.97A48.416 48.416 0 0012 4.5c-2.291 0-4.545.16-6.75.47m13.5 0c1.01.143 2.01.317 3 .52m-3-.52l2.62 10.726c.122.499-.106 1.028-.589 1.202a5.988 5.988 0 01-2.031.352 5.988 5.988 0 01-2.031-.352c-.483-.174-.711-.703-.59-1.202L18.75 4.971zm-16.5.52c.99-.203 1.99-.377 3-.52m0 0l2.62 10.726c.122.499-.106 1.028-.589 1.202a5.989 5.989 0 01-2.031.352 5.989 5.989 0 01-2.031-.352c-.483-.174-.711-.703-.59-1.202L5.25 4.971z" /></svg>
					<span v-if="!collapsed">Assets</span>
				</NuxtLink>
			</li>
			<li ref="accounts" @click="change_route('accounts')">
				<NuxtLink to="/accounts" :class="currentRoute == 'accounts' ? 'active' : ''">
					<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" d="M2.25 8.25h19.5M2.25 9h19.5m-16.5 5.25h6m-6 2.25h3m-3.75 3h15a2.25 2.25 0 002.25-2.25V6.75A2.25 2.25 0 0019.5 4.5h-15a2.25 2.25 0 00-2.25 2.25v10.5A2.25 2.25 0 004.5 19.5z" /></svg>
					<span v-if="!collapsed">Accounts</span>
				</NuxtLink>
			</li>
			<li ref="recipients" @click="change_route('recipients')">
				<NuxtLink to="/recipients" :class="currentRoute == 'recipients' ? 'active' : ''">
					<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" d="M13.5 21v-7.5a.75.75 0 01.75-.75h3a.75.75 0 01.75.75V21m-4.5 0H2.36m11.14 0H18m0 0h3.64m-1.39 0V9.349m-16.5 11.65V9.35m0 0a3.001 3.001 0 003.75-.615A2.993 2.993 0 009.75 9.75c.896 0 1.7-.393 2.25-1.016a2.993 2.993 0 002.25 1.016c.896 0 1.7-.393 2.25-1.016a3.001 3.001 0 003.75.614m-16.5 0a3.004 3.004 0 01-.621-4.72L4.318 3.44A1.5 1.5 0 015.378 3h13.243a1.5 1.5 0 011.06.44l1.19 1.189a3 3 0 01-.621 4.72m-13.5 8.65h3.75a.75.75 0 00.75-.75V13.5a.75.75 0 00-.75-.75H6.75a.75.75 0 00-.75.75v3.75c0 .415.336.75.75.75z" /></svg>
					<span v-if="!collapsed">Recipients</span>
				</NuxtLink>
			</li>
			<li ref="tags" @click="change_route('tags')">
				<NuxtLink to="/tags" :class="currentRoute == 'tags' ? 'active' : ''">
					<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" d="M9.568 3H5.25A2.25 2.25 0 003 5.25v4.318c0 .597.237 1.17.659 1.591l9.581 9.581c.699.699 1.78.872 2.607.33a18.095 18.095 0 005.223-5.223c.542-.827.369-1.908-.33-2.607L11.16 3.66A2.25 2.25 0 009.568 3z" /><path stroke-linecap="round" stroke-linejoin="round" d="M6 6h.008v.008H6V6z" /></svg>
					<span v-if="!collapsed">Tags</span>
				</NuxtLink>
			</li>
			<li ref="currencies" @click="change_route('currencies')">
				<NuxtLink to="/currencies" :class="currentRoute == 'currencies' ? 'active' : ''">
					<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" d="M12 6v12m-3-2.818l.879.659c1.171.879 3.07.879 4.242 0 1.172-.879 1.172-2.303 0-3.182C13.536 12.219 12.768 12 12 12c-.725 0-1.45-.22-2.003-.659-1.106-.879-1.106-2.303 0-3.182s2.9-.879 4.006 0l.415.33M21 12a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
					<span v-if="!collapsed">Currencies</span>
				</NuxtLink>
			</li>
			<br><br>
			<li ref="settings" @click="change_route('settings')">
				<NuxtLink to="/settings" :class="currentRoute == 'settings' ? 'active' : ''">
					<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" d="M4.5 12a7.5 7.5 0 0015 0m-15 0a7.5 7.5 0 1115 0m-15 0H3m16.5 0H21m-1.5 0H12m-8.457 3.077l1.41-.513m14.095-5.13l1.41-.513M5.106 17.785l1.15-.964m11.49-9.642l1.149-.964M7.501 19.795l.75-1.3m7.5-12.99l.75-1.3m-6.063 16.658l.26-1.477m2.605-14.772l.26-1.477m0 17.726l-.26-1.477M10.698 4.614l-.26-1.477M16.5 19.794l-.75-1.299M7.5 4.205L12 12m6.894 5.785l-1.149-.964M6.256 7.178l-1.15-.964m15.352 8.864l-1.41-.513M4.954 9.435l-1.41-.514M12.002 12l-3.75 6.495" /></svg>
					<span v-if="!collapsed">Settings</span>
				</NuxtLink>
			</li>
			<li @click="logout">
				<a>
					<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" d="M5.636 5.636a9 9 0 1012.728 0M12 3v9" /></svg>
					<span v-if="!collapsed">Logout</span>
				</a>
			</li>
		</ul>
		<div id="controller">
			<div id="collapse_controller">
				<div @click="collapsed = !collapsed">
					<svg :style="collapsed ? 'transform: rotate(0deg);' : 'transform: rotate(180deg);'" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" d="M11.25 4.5l7.5 7.5-7.5 7.5m-6-15l7.5 7.5-7.5 7.5" /></svg>
				</div>
			</div>
			<div id="hide_controller">
				<div @click="hidden = !hidden">
					<svg v-if="!hidden" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" d="M9.75 9.75l4.5 4.5m0-4.5l-4.5 4.5M21 12a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
					<svg v-if="hidden" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25H12" /></svg>
				</div>
			</div>
		</div>
	</nav>
</template>

<script lang="ts">
export default {
	data: () => ({
		currentRoute: useRoute().path.split("/")[1] ? useRoute().path.split("/")[1] : "dashboard",
		collapsed: false,
		hidden: false,
		small_device: false,
	}),

	props: {
		loggedIn: {
			type: Boolean,
			required: true,
		},
		show_trigger: {
			type: Number,
		}
	},

	mounted() {
		this.$nextTick(() => {
      window.addEventListener('resize', this.on_resize);
    });
		this.on_resize();
	},

	methods: {
		logout() {
			document.cookie = "accessToken=;expires=Thu, 01 Jan 1970 00:00:00 GMT";
			location.reload();
		},

		change_route(new_route: string) {
			this.currentRoute = new_route;
			this.hidden = true;
		},

		on_resize() {
			this.small_device = window.innerWidth <= 800;
		}
	},
}
</script>

<style lang="sass">
@import "assets/_vars.sass"

nav
	width: fit-content
	min-height: 100dvh
	display: grid
	grid-template-columns: 1fr
	grid-template-rows: 5em 1fr 5em
	transition-duration: 0.2s
	h1
		font-size: 2em
		margin: 0.5em
		cursor: pointer
		user-select: none
		text-shadow: 4px 4px 8px black
		&:hover
			transform: scale(1.1) rotate(5deg)
	a, span
		@extend .semibold
		font-size: 24px
		margin: 10px
		transition-duration: 0.2s
		cursor: pointer
	@media screen and (max-width: 800px)
		position: fixed
		top: 0
		left: 0
		height: 100dvh
		width: 100dvw
		z-index: 100
		border-right: none !important

.mobile
	@media screen and (max-width: 800px)
		position: inherit
		display: block
		min-height: 4em
		height: 4em
		width: 100dvw

@media screen and (max-width: 800px)
	html.dark-mode 
		nav
			border-bottom: 2px solid $dark-heavy
	html.light-mode
		nav
			border-bottom: 2px solid $light-heavy
	html.monochrome-mode
		nav
			border-bottom: 2px solid $monochrome-heavy
	html.black-mode
		nav
			border-bottom: 2px solid $black-heavy

li
	width: fit-content
	&:hover
		transform: scale(1.1)
	svg
		height: 1.5em
		margin-bottom: -0.4em
		margin-left: 0.1em

#controller
	height: 4em
	width: 4em
	svg
		height: 100%
		width: 100%
	&:hover
		transform: rotate(-30deg)

#collapse_controller
	@media screen and (max-width: 800px)
		display: none

#hide_controller
	@media screen and (min-width: 801px)
		display: none
</style>