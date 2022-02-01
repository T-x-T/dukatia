<template>
	<div id="wrapper">
		<button @click="$emit('back')">Back</button>

		<div id="details">
			<label for="id">ID:</label>
			<input type="text" id="id" disabled v-model="tag.id">
			<br>
			<label for="name">Name:</label>
			<input type="text" id="name" v-model="tag.name">
			<br>
			<label for="parent">Parent:</label>
			<select id="parent" v-model="tag.parentId">
				<option value="">No Parent</option>
				<option v-for="(item, index) in $store.state.tags" :key="index" :value="item.id">{{item.name}}</option>
			</select>
			<br>
			<button class="green" @click="sendTag">Save</button>
			<button class="red" @click="$emit('back')">Cancel</button>
		</div>
	</div>
</template>

<script>
export default {
	data: () => ({

	}),

	props: {
		tag: Object
	},

	methods: {
		async sendTag() {
			const tagData = {
				name: this.tag.name,
				parentId: this.tag.parentId ? this.tag.parentId : undefined
			}

			if(this.tag.id) {
				await this.$axios.$put(`/api/v1/tags/${this.tag.id}`, tagData);
			} else {
				await this.$axios.$post("/api/v1/tags", tagData);
			}

			this.$emit("back");
		}
	}
}
</script>

<style lang="sass" scoped>
div#details
	margin: 20px 0px 20px
	button
		margin-top: 10px
</style>