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
				<option value=""></option>
				<option v-for="(item, index) in $store.state.tags" :key="index" :value="item.id">{{item.name}}</option>
			</select>
			<br>
			<button class="green" @click="sendTag(true)">Save</button>
			<button class="red" @click="$emit('back')">Cancel</button>
			<button class="green" @click="sendTag(false)">Save and New</button>
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
		async sendTag(goBack) {
			const tagData = {
				name: this.tag.name,
				parentId: this.tag.parentId ? this.tag.parentId : undefined
			}

			if(typeof this.tag.id == "number") {
				await this.$axios.$put(`/api/v1/tags/${this.tag.id}`, tagData);
			} else {
				await this.$axios.$post("/api/v1/tags", tagData);
			}

			if(goBack) {
				this.$emit("back");
			} else {
				this.tag = {
					id: "",
					name: ""
				}
			}
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