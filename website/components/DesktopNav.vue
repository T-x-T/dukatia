<template>
	<nav>
		<button class="important" @click="navigateTo('/docs')">Get started</button>
		<div id="theme_selector" @click="$colorMode.preference = $colorMode.preference == 'dark' ? 'light' : 'dark'">
			<svg v-if="$colorMode.preference == 'dark'" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2.5" stroke="currentColor" class="w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" d="M21.752 15.002A9.72 9.72 0 0 1 18 15.75c-5.385 0-9.75-4.365-9.75-9.75 0-1.33.266-2.597.748-3.752A9.753 9.753 0 0 0 3 11.25C3 16.635 7.365 21 12.75 21a9.753 9.753 0 0 0 9.002-5.998Z" /></svg>
      <svg v-if="$colorMode.preference == 'light'" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2.5" stroke="currentColor" class="w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" d="M12 3v2.25m6.364.386-1.591 1.591M21 12h-2.25m-.386 6.364-1.591-1.591M12 18.75V21m-4.773-4.227-1.591 1.591M5.25 12H3m4.227-4.773L5.636 5.636M15.75 12a3.75 3.75 0 1 1-7.5 0 3.75 3.75 0 0 1 7.5 0Z" /></svg>
		</div>
		<NuxtLink to="/docs">Docs</NuxtLink>
		<NuxtLink to="/demo">Demo</NuxtLink>
		<NuxtLink to="/about">About</NuxtLink>
		<NuxtLink to="/">Home</NuxtLink>
		<img v-if="$colorMode.preference == 'dark'" src="/dukatia-beta_logo_white.svg" @click="navigateTo('/')"/>
		<img v-if="$colorMode.preference == 'light'" src="/dukatia-beta_logo.svg" @click="navigateTo('/')"/>
	</nav>
</template>

<script lang="ts">
export default {
  data: () => ({
    small_device: false,
  }),

  mounted() {
		this.$nextTick(() => {
      window.addEventListener('resize', this.on_resize);
    });
		this.on_resize();
	},

  methods: {
    on_resize() {
			this.small_device = window.innerWidth <= 800;
		}
  }
}
</script>

<style lang="sass" scoped>
@import "assets/_vars.sass"

nav
  display: flex
  flex-direction: row-reverse
  height: 100px
  width: 100vw
  align-items: center
  position: fixed
  top: 0
  button.important
    margin: 2% 2% 2% 1%
  img
    height: 70%
    left: 2%
    top: 15%
    position: absolute
    &:hover
      scale: 1.05
      rotate: 5deg
      cursor: pointer
  a
    @extend .bold
    font-size: 200%
    margin: 1%
  div#theme_selector
    margin: 25px
    height: 2em
    color: white
    cursor: pointer
    svg
      height: 100%
      width: 100%

html.dark-mode
  nav
    background: $dark-darker
    box-shadow: black 0px 4px 4px 0px
    a, svg
      color: $dark-brightest
      &.active
        color: $dark-bright
      &:hover
        color: $dark-bright
html.light-mode
  nav
    background: $light-brightest
    box-shadow: #ddd 0px 4px 4px 0px
    a, svg
      color: $light-darkest
      &.active
        color: $light-darker
      &:hover
        color: $light-darker
</style>