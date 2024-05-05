<template>
  <main>
    <div id="introduction">
      <div id="container">
        <h1>Manage Your Finances.</h1>
        <h2>Go Dukatia.</h2>
  
        <p>Dukatia is your finance planner.<br>Track your spending, manage your assets and set smarter budgets.<br>Run it in Docker and access it from any web browser!</p>
        <button class="important" @click="navigateTo('/docs')">Get started</button>
      </div>
    </div>

    <div id="main_image" class="mobile_hidden">
      <div id="container">
        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" d="M3.75 3v11.25A2.25 2.25 0 0 0 6 16.5h2.25M3.75 3h-1.5m1.5 0h16.5m0 0h1.5m-1.5 0v11.25A2.25 2.25 0 0 1 18 16.5h-2.25m-7.5 0h7.5m-7.5 0-1 3m8.5-3 1 3m0 0 .5 1.5m-.5-1.5h-9.5m0 0-.5 1.5m.75-9 3-3 2.148 2.148A12.061 12.061 0 0 1 16.5 7.605" /></svg>
      </div>
    </div>

    <div id="features_header">
      <h3>Features</h3>
      <p>Dukatia fulfills all your needs while not overwhelming you.<br>Your customizable dashboard shows you just what you need.</p>
    </div>

    <div class="feature">
      <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" d="M9 12h3.75M9 15h3.75M9 18h3.75m3 .75H18a2.25 2.25 0 0 0 2.25-2.25V6.108c0-1.135-.845-2.098-1.976-2.192a48.424 48.424 0 0 0-1.123-.08m-5.801 0c-.065.21-.1.433-.1.664 0 .414.336.75.75.75h4.5a.75.75 0 0 0 .75-.75 2.25 2.25 0 0 0-.1-.664m-5.8 0A2.251 2.251 0 0 1 13.5 2.25H15c1.012 0 1.867.668 2.15 1.586m-5.8 0c-.376.023-.75.05-1.124.08C9.095 4.01 8.25 4.973 8.25 6.108V8.25m0 0H4.875c-.621 0-1.125.504-1.125 1.125v11.25c0 .621.504 1.125 1.125 1.125h9.75c.621 0 1.125-.504 1.125-1.125V9.375c0-.621-.504-1.125-1.125-1.125H8.25ZM6.75 12h.008v.008H6.75V12Zm0 3h.008v.008H6.75V15Zm0 3h.008v.008H6.75V18Z" /></svg>
      <div class="feature_text">
        <h4>Transactions</h4>
        <p class="mobile_hidden">Log your transactions and add metadata like recipients, accounts, assets and tags to get better insights into your spending habits.</p>
      </div>
    </div>
    <div class="feature">
      <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" d="M10.5 6a7.5 7.5 0 1 0 7.5 7.5h-7.5V6Z" /><path stroke-linecap="round" stroke-linejoin="round" d="M13.5 10.5H21A7.5 7.5 0 0 0 13.5 3v7.5Z" /></svg>
      <div class="feature_text">
        <h4>Budgets</h4>
        <p class="mobile_hidden">Set yourself a budget to plan how you want to spend your money.</p>
      </div>
    </div>
    <div class="feature">
      <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" d="M12 3v17.25m0 0c-1.472 0-2.882.265-4.185.75M12 20.25c1.472 0 2.882.265 4.185.75M18.75 4.97A48.416 48.416 0 0 0 12 4.5c-2.291 0-4.545.16-6.75.47m13.5 0c1.01.143 2.01.317 3 .52m-3-.52 2.62 10.726c.122.499-.106 1.028-.589 1.202a5.988 5.988 0 0 1-2.031.352 5.988 5.988 0 0 1-2.031-.352c-.483-.174-.711-.703-.59-1.202L18.75 4.971Zm-16.5.52c.99-.203 1.99-.377 3-.52m0 0 2.62 10.726c.122.499-.106 1.028-.589 1.202a5.989 5.989 0 0 1-2.031.352 5.989 5.989 0 0 1-2.031-.352c-.483-.174-.711-.703-.59-1.202L5.25 4.971Z" /></svg>
      <div class="feature_text">
        <h4>Assets</h4>
        <p class="mobile_hidden">Track your financial assets to get an overview over your investments.</p>
      </div>
    </div>
    <div class="feature">
      <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" d="M2.25 8.25h19.5M2.25 9h19.5m-16.5 5.25h6m-6 2.25h3m-3.75 3h15a2.25 2.25 0 0 0 2.25-2.25V6.75A2.25 2.25 0 0 0 19.5 4.5h-15a2.25 2.25 0 0 0-2.25 2.25v10.5A2.25 2.25 0 0 0 4.5 19.5Z" /></svg>
      <div class="feature_text">
        <h4>Multiple Accounts</h4>
        <p class="mobile_hidden">Track your spending across all of your accounts.</p>
      </div>
    </div>
    <div class="feature">
      <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" d="M9.568 3H5.25A2.25 2.25 0 0 0 3 5.25v4.318c0 .597.237 1.17.659 1.591l9.581 9.581c.699.699 1.78.872 2.607.33a18.095 18.095 0 0 0 5.223-5.223c.542-.827.369-1.908-.33-2.607L11.16 3.66A2.25 2.25 0 0 0 9.568 3Z" /><path stroke-linecap="round" stroke-linejoin="round" d="M6 6h.008v.008H6V6Z" /></svg>
      <div class="feature_text">
        <h4>Powerful Tagging </h4>
        <p class="mobile_hidden">Tags don't just stand on their own - they can also have a parent. This lets you define a tree-like structure for all your tagging needs.</p>
      </div>
    </div>
    <div class="feature">
      <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" d="M12 6v12m-3-2.818.879.659c1.171.879 3.07.879 4.242 0 1.172-.879 1.172-2.303 0-3.182C13.536 12.219 12.768 12 12 12c-.725 0-1.45-.22-2.003-.659-1.106-.879-1.106-2.303 0-3.182s2.9-.879 4.006 0l.415.33M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z" /></svg>
      <div class="feature_text">
        <h4>Multiple Currencies</h4>
        <p class="mobile_hidden">You have a bank account in a foreign currency? No problem! You can also define your own custom currencies.</p>
      </div>
    </div>
    <div class="feature">
      <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" d="M3.75 3v11.25A2.25 2.25 0 0 0 6 16.5h2.25M3.75 3h-1.5m1.5 0h16.5m0 0h1.5m-1.5 0v11.25A2.25 2.25 0 0 1 18 16.5h-2.25m-7.5 0h7.5m-7.5 0-1 3m8.5-3 1 3m0 0 .5 1.5m-.5-1.5h-9.5m0 0-.5 1.5m.75-9 3-3 2.148 2.148A12.061 12.061 0 0 1 16.5 7.605" /></svg>
      <div class="feature_text">
        <h4>Customizable Dashboards </h4>
        <p class="mobile_hidden">With the Dashboard editor you can build the Dashboards you want. This allows you to get a quick overview over the data you need.</p>
      </div>
    </div>
    <div class="feature">
      <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" d="M16.5 10.5V6.75a4.5 4.5 0 1 0-9 0v3.75m-.75 11.25h10.5a2.25 2.25 0 0 0 2.25-2.25v-6.75a2.25 2.25 0 0 0-2.25-2.25H6.75a2.25 2.25 0 0 0-2.25 2.25v6.75a2.25 2.25 0 0 0 2.25 2.25Z" /></svg>
      <div class="feature_text">
        <h4>Self-hosted & Open-Source</h4>
        <p class="mobile_hidden">Your financial data tells a lot about you, so keep it safe! Dukatia is fully self-hosted and doesn't send your sensitive data to the cloud.</p>
      </div>
    </div>

    <div id="for_you">
      <h3>Dukatia Is Just for You …</h3>
      <p>
        … if you want to <span>easily manage</span> your finances.<br>
        … if you <span>outgrew</span> your <span>Excel</span> spreadsheet.<br>
        … if you search for a <span>self-hosted</span>, <span>private</span> finance planner.<br>
        … if you like to <span>customize</span> your user experience.<br>
        … if you are a bit <span>tech-savvy</span> and willing to self-host.<br>
      </p>
    </div>

    <div id="go_dukatia">
      <div id="container">
        <div id="text">
          <h3>Go Dukatia!</h3>
          <p>Installing Dukatia isn't hard. We'll show you how it's done. Alternatively, have a look at the live demo before deciding!</p>
        </div>
        <div id="buttons">
          <button class="important" @click="navigateTo('/docs')">Get started</button>
          <button class="less_important" @click="navigateTo('/demo')">Live Demo</button>
        </div>
      </div>
    </div>
  </main>
</template>

<style lang="sass" scoped>
@import "assets/_vars.sass"

main
  display: grid
  grid-template-columns: repeat(2, 50vw)
  grid-template-rows: 65vh
  grid-auto-rows: 400px
  min-height: 100vh
  @media screen and (max-width: 1079px)
    grid-template-rows: 90vh

div#go_dukatia
  grid-area: 8 / 1 / 9 / 3
  width: 100%
  padding: 25px
  height: fit-content
  background: $dark-bright
  display: flex
  justify-content: center
  align-content: center
  ::selection
    color: $dark-brightest
  div#container
    display: grid
    grid-template-columns: 1fr
    grid-template-rows: 1fr, 1fr
    column-gap: 50px
    div
      max-width: 400px
    div#text
      grid-area: 1 / 1 / 2 / 2
      color: $dark-darkest
      h3
        font-size: 60px
      p
        font-size: 26px
        line-height: 34px
    div#buttons
      grid-area: 1 / 2 / 2 / 3
      button
        width: 265px
        height: 76px
        margin: 10px

div#for_you
  grid-area: 7 / 1 / 8 / 3
  place-self: center
  h3
    font-size: 60px
  p
    font-size: 26px
    line-height: 45px
  span
    color: $dark-bright

div.feature
  display: grid
  height: 340px
  width: min(700px, 90%)
  border: 5px solid $dark-darker
  border-radius: 100px
  place-self: center
  @media screen and (max-width: 1079px)
    border: none
    display: flex
    flex-direction: column
    justify-content: center
    align-items: center
  svg
    grid-area: 1 / 1 / 2 / 2
    width: 188px
    max-width: 10vw
    color: $dark-bright
    margin: min(65px, 1vw)
    justify-self: center
    align-self: center
  div.feature_text
    grid-area: 1 / 2 / 2 / 3
    width: 380px
    max-width: 30vw
    align-self: center
    @media screen and (max-width: 1079px)
      grid-area: 2 / 1 / 3 / 2
    h4
      color: $dark-bright
      font-size: min(3vw, 50px)
    p
      font-size: 26px

html.light-mode
  div.feature
    border: 5px solid $light-dark
    div.feature_text > h4
      color: black

div#features_header
  grid-area: 2 / 1 / 3 / 3
  text-align: center
  align-self: center
  h3
    @extend .semibold
    font-size: 60px
  p
    font-size: 26px
    line-height: 34px

div#main_image
  grid-area: 1 / 2 / 2 / 3
  height: 100%
  div#container
    padding-top: 10%
    height: 100%
    svg
      height: 80%

html.light-mode
  div#main_image, div#introduction
    background: $light-dark
    div#container > h2
      color: white

div#introduction
  grid-area: 1 / 1 / 2 / 2
  height: 100%
  width: 100%
  div#container
    padding: 20% 0 10% 10%
    h1, h2
      font-size: 60px
      line-height: 70px
    h2
      color: $dark-bright
    p
      margin: 20px 0 20px 0
      width: min(75vw, 488px)
      font-size: 26px
  @media screen and (max-width: 1079px)
    grid-area: 1 / 1 / 2 / 3

</style>

