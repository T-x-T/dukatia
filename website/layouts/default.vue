<template>
	<DesktopNav v-if="!small_device" />
	<MobileNav v-if="small_device" />
	<div id="content">
		<slot />
	</div>
	<BottomFooter />
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
			this.small_device = window.innerWidth < 1080;
		}
  }
}
</script>

<style lang="sass" scoped>
div#content
	margin-top: 100px
</style>