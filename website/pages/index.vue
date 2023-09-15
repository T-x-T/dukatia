<template>
  <main>

    <nav>
      <p>Navigation lol</p>
      <ThemeSelector />
    </nav>
    
    <div id="title">
      <h1 class="hovercolor" @mouseover="hover_feature" @mouseleave="hover_feature_dismantle">Dukatia</h1>
      <p>A way to manage your finances!</p>
    </div>

    <div class="feature gridItem hovercolor" @mouseover="hover_feature" @mouseleave="hover_feature_dismantle">
      <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" d="M9 12h3.75M9 15h3.75M9 18h3.75m3 .75H18a2.25 2.25 0 002.25-2.25V6.108c0-1.135-.845-2.098-1.976-2.192a48.424 48.424 0 00-1.123-.08m-5.801 0c-.065.21-.1.433-.1.664 0 .414.336.75.75.75h4.5a.75.75 0 00.75-.75 2.25 2.25 0 00-.1-.664m-5.8 0A2.251 2.251 0 0113.5 2.25H15c1.012 0 1.867.668 2.15 1.586m-5.8 0c-.376.023-.75.05-1.124.08C9.095 4.01 8.25 4.973 8.25 6.108V8.25m0 0H4.875c-.621 0-1.125.504-1.125 1.125v11.25c0 .621.504 1.125 1.125 1.125h9.75c.621 0 1.125-.504 1.125-1.125V9.375c0-.621-.504-1.125-1.125-1.125H8.25zM6.75 12h.008v.008H6.75V12zm0 3h.008v.008H6.75V15zm0 3h.008v.008H6.75V18z" /></svg>
      <p>Record your transactions</p>
    </div>
    <div class="feature gridItem hovercolor" @mouseover="hover_feature" @mouseleave="hover_feature_dismantle">
      <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" d="M2.25 12l8.954-8.955c.44-.439 1.152-.439 1.591 0L21.75 12M4.5 9.75v10.125c0 .621.504 1.125 1.125 1.125H9.75v-4.875c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125V21h4.125c.621 0 1.125-.504 1.125-1.125V9.75M8.25 21h8.25" /></svg>
      <p>Track your assets</p>
    </div>
    <div class="feature gridItem hovercolor" @mouseover="hover_feature" @mouseleave="hover_feature_dismantle">
      <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" d="M3.75 3v11.25A2.25 2.25 0 006 16.5h2.25M3.75 3h-1.5m1.5 0h16.5m0 0h1.5m-1.5 0v11.25A2.25 2.25 0 0118 16.5h-2.25m-7.5 0h7.5m-7.5 0l-1 3m8.5-3l1 3m0 0l.5 1.5m-.5-1.5h-9.5m0 0l-.5 1.5m.75-9l3-3 2.148 2.148A12.061 12.061 0 0116.5 7.605" /></svg>
      <p>Collect insights</p>
    </div>

    <div id="install" class="gridItem">
      <p>
        Installation is very simple, thanks to the power of Docker!
        <br>Read the <NuxtLink to="/docs">getting started guide</NuxtLink> or watch <a href="#">the video guide</a>
      </p>
    </div>

  </main>
</template>

<script>
export default {
  data: () => ({
    feature_hovering: false,
    colors: ["#EC8A83", "#FFAD85", "#F9F176", "#8BE59D", "#6AB4F1", "#A983D8"],
  }),

  methods: {
    hover_feature(e) {
      if(this.feature_hovering) return;
      this.feature_hovering = true;

      if(e.target.classList.contains("hovercolor")) {
        const random_color = this.colors[Math.floor(Math.random() * this.colors.length)];
        if(e.target.tagName == "DIV") {
          e.target.style = `background: ${random_color}60`;
        } else {
          e.target.style = `color: ${random_color}`;
        }
      }
    },

    hover_feature_dismantle(e) {
      if(e.target.classList.contains("hovercolor")) {
        e.target.style = "";
        this.feature_hovering = false;
      }
    }
  }
}
</script>

<style lang="sass" scoped>
@import "assets/_vars.sass"

main
  display: grid
  grid-template-columns: repeat(24, 1fr)
  grid-template-rows: 3em
  grid-auto-rows: 10vh
nav
  grid-area: 1 / 1 / 1 / row-end
  display: flex
  flex-direction: row-reverse
  *
    padding: 0.5em

.gridItem
  display: flex
  flex-direction: column
  align-items: center
  justify-content: center 
  text-align: center

div#title
  grid-area: 3 / 7 / 5 / 19
  display: flex
  flex-direction: column
  align-items: center
  justify-content: center
  background-color: rgba(0, 0, 0, 0.3)
  backdrop-filter: grayscale(30%) blur(5px)
  h1
    font-size: 5vw
    cursor: pointer
    user-select: none
    text-shadow: 4px 4px 8px black
    &:hover
      transform: scale(1.05) rotate(1deg)
  p
    font-size: 1.5em

.feature
  font-size: 2em
  svg
    height: 50%
    &:hover
      transition-duration: 0.2s
      transition-timing-function: cubic-bezier(0.1, -1, 0.1, 1.0)
      transform: scale(1.1)
.feature:nth-of-type(2)
  grid-area: 6 / 3 / 8 / 7
.feature:nth-of-type(3)
  grid-area: 6 / 11 / 8 / 15
.feature:nth-of-type(4)
  grid-area: 6 / 19 / 8 / 23

#install
  font-size: 1.5em
  grid-area: 9 / 4 / 10 / 21
  a
    @extend .semibold

</style>

