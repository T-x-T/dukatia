<template>
	<main>
		<SideNav 
			:loggedIn="loggedIn"		
		/>
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

const registerServiceWorker = async () => {
  if ("serviceWorker" in navigator) {
    try {
      const registration = await navigator.serviceWorker.register("/worker.js", {
        scope: "/",
      });
      if (registration.installing) {
        console.log("Service worker installing");
      } else if (registration.waiting) {
        console.log("Service worker installed");
      } else if (registration.active) {
        console.log("Service worker active");
      }
    } catch (error) {
      console.error(`Registration failed with ${error}`);
    }
  }
};

registerServiceWorker();
</script>

<style lang="sass" scoped>
main
	display: flex
	height: 100svh
	width: 100svw
	overflow: hidden
	@media screen and (max-width: 800px)
		flex-direction: column

#content
	flex-grow: 1
	width: 1rem
	height: 100vh
	overflow: auto
	@media screen and (max-width: 800px)
		height: 1rem
		width: 100vw
</style>