<template>
	<div id="wrapper">
		<div id="popup">
			<button @click="$emit('close')" class="red">
				<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" /></svg>
			</button>
			<div id="popupContent">
				<slot />
			</div>
		</div>
	</div>
</template>

<script lang="ts">
export default {
	emits: ["close"],
}
</script>

<style lang="sass" scoped>
#wrapper
	width: 100vw
	height: 100vh
	position: fixed
	top: 0
	left: 0
	background: rgba(1, 1, 1, 0.5)
	display: flex
	justify-content: center
	align-items: center
	z-index: 1000

#popup
	z-index: 1001
	padding: 10px

svg
	margin-bottom: -6px
	height: 24px
</style>