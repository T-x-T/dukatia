<template>
	<nav>
    <div v-if="!nav_open" id="top_nav" @click="nav_open = true">
      <img v-if="$colorMode.preference == 'dark'" src="/dukatia-beta_logo_white.svg" @click="navigateTo('/')"/>
      <img v-if="$colorMode.preference == 'light'" src="/dukatia-beta_logo.svg" @click="navigateTo('/')"/>
      <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" d="M3.75 6.75h16.5M3.75 12h16.5M12 17.25h8.25" /></svg>
    </div>
    
    <div v-if="nav_open" id="fullscreen_nav">
      <img v-if="$colorMode.preference == 'dark'" src="/dukatia-beta_logo_white.svg" @click="() => {navigateTo('/'); nav_open = false}"/>
      <img v-if="$colorMode.preference == 'light'" src="/dukatia-beta_logo.svg" @click="() => {navigateTo('/'); nav_open = false}"/>
      <button class="important" @click="() => {navigateTo('/docs'); nav_open = false}">Get started</button>
      <NuxtLink @click.native="nav_open = false" to="/docs">Docs</NuxtLink>
      <NuxtLink @click.native="nav_open = false" to="/demo">Demo</NuxtLink>
      <NuxtLink @click.native="nav_open = false" to="/about">About</NuxtLink>
      <NuxtLink @click.native="nav_open = false" to="/">Home</NuxtLink>
      <div id="theme_selector" @click="$colorMode.preference = $colorMode.preference == 'dark' ? 'light' : 'dark'">
        <svg v-if="$colorMode.preference == 'dark'" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2.5" stroke="currentColor" class="w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" d="M21.752 15.002A9.72 9.72 0 0 1 18 15.75c-5.385 0-9.75-4.365-9.75-9.75 0-1.33.266-2.597.748-3.752A9.753 9.753 0 0 0 3 11.25C3 16.635 7.365 21 12.75 21a9.753 9.753 0 0 0 9.002-5.998Z" /></svg>
        <svg v-if="$colorMode.preference == 'light'" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2.5" stroke="currentColor" class="w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" d="M12 3v2.25m6.364.386-1.591 1.591M21 12h-2.25m-.386 6.364-1.591-1.591M12 18.75V21m-4.773-4.227-1.591 1.591M5.25 12H3m4.227-4.773L5.636 5.636M15.75 12a3.75 3.75 0 1 1-7.5 0 3.75 3.75 0 0 1 7.5 0Z" /></svg>
      </div>
      <svg id="close" @click="nav_open = false" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" class="w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" d="m9.75 9.75 4.5 4.5m0-4.5-4.5 4.5M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z" /></svg>
    </div>
	</nav>
</template>

<script lang="ts">
export default {
  data: () => ({
    nav_open: false
  }),
}
</script>

<style lang="sass" scoped>
@import "assets/_vars.sass"

#top_nav
  height: 100px
  width: 100vw
  align-items: center
  position: fixed
  top: 0 
  z-index: 1000
  display: flex
  align-items: center
  justify-content: space-between
  img
    height: 70%
    width: auto
    margin-left: 20px
    &:hover
      scale: 1.05
      rotate: 5deg
      cursor: pointer
  svg
    height: 60px
    padding: 3px
    margin-right: 20px
    color: $dukatia-yellow !important
    border: 4px solid $dukatia-yellow
    border-radius: 100px

#fullscreen_nav
  position: fixed
  min-height: 100vh
  width: 100vw
  top: 0
  left: 0
  display: flex
  flex-direction: column
  align-items: center
  z-index: 1000
  button
    font-size: 3em
    width: max-content
  img
    margin: 25px
    height: 8em
    width: auto
    max-width: 80vw
  a
    @extend .bold
    font-size: min(5vh, 4em)
  div#theme_selector
    height: min(10vh, 7em)
    width: auto
    color: white
    cursor: pointer
    svg
      height: 100%
      width: 100%
  svg#close
    height: min(10vh, 7em)

html.dark-mode
  #top_nav
    background: $dark-darker
    box-shadow: black 0px 4px 4px 0px
  #fullscreen_nav
    background: $dark-darker
    a, svg
      color: $dark-brightest
      &.active
        color: $dark-bright
      &:hover
        color: $dark-bright
html.light-mode
  #top_nav
    background: $light-brightest
    box-shadow: #ddd 0px 4px 4px 0px
  #fullscreen_nav
    background: $light-brightest
    a, svg
      color: $light-darkest
      &.active
        color: $light-darker
      &:hover
        color: $light-darker
</style>