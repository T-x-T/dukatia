<template>
	<NuxtLayout>
		<NuxtPage />
	</NuxtLayout>
</template>

<script lang="ts" setup>
try {
	await $fetch("/api/v1/users/me");
} catch(e) {
	useRouter().replace("/login");
}

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