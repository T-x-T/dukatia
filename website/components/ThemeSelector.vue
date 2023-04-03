<template>
	<div>
		<label for="theme">Select Theme</label>
		<select id="theme" v-model="$colorMode.preference">
			<option value="dark">Dark</option>
			<option value="light">Light</option>
			<option value="monochrome">Monochrome</option>
			<option value="black">Black</option>
		</select>
	</div>
</template>