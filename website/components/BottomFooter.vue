<template>
	<footer>
    <img src="/dukatia-beta_logo_white.svg" />
    <p>Coded by TxT. Made in Rust.</p>
    <div id="notices">
      <NuxtLink to="/privacy">Privacy Policy</NuxtLink>
      <NuxtLink to="/legal">Legal Notice</NuxtLink>
    </div>
  </footer>
</template>

<style lang="sass" scoped>
footer
  width: 100%
  height: 150px
  display: flex
  align-items: center
  justify-content: space-between
  p
    font-size: 150%
    width: 20vw
    text-align: center
  img
    margin: 30px
    height: 85px
  a
    font-size: 150%
    margin-right: 30px
</style>