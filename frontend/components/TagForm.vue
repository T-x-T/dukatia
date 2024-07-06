<template>
	<div id="wrapper">
		<button class="orange" @click="$emit('back')">Back</button>
		
		<label>
			ID:
			<input type="number" v-model="tag.id" disabled>
		</label>
		<label>
			Name:
			<input type="text" v-model="tag.name" ref="first_input">
		</label>
		<label>
			Parent:
			<select v-model="tag.parent_id">
				<option></option>
				<option v-for="(tag, index) in tags" :key="index" :value="tag.id">{{tag.name}}</option>
			</select>
		</label>

		<br>
		<button class="green" @click="save(true)">Save</button>
		<button class="orange" @click="$emit('back')">Cancel</button>
		<button class="green" @click="save(false)">Save and New</button>
	</div>
</template>

<script lang="ts">
export default {
	data: () => ({
		tag: {} as Tag,
		tags: [] as Tag[],
		default_tag: {
			id: undefined,
			name: "",
			parent_id: undefined,
		} as Tag,
	}),

	emits: ["back", "data_saved"],

	props: {
		data: {
			type: Object as PropType<Tag>,
			required: false,
		}
	},

	async mounted() {
		this.tag = this.data && Object.keys(this.data).length > 0 ? this.data : structuredClone(toRaw(this.default_tag));

		this.tags = await $fetch("/api/v1/tags/all");
		(this.$refs.first_input as any).focus();
	},

	methods: {
		async save(go_back: boolean) {
			let res = {} as Tag;

			try {
				if(typeof this.tag.id == "string" && this.tag.id.length == 36) {
					res = await $fetch(`/api/v1/tags/${this.tag.id}`, {
						method: "PUT",
						body: this.get_body(),
					});
				} else {
					res = await $fetch("/api/v1/tags", {
						method: "POST",
						body: this.get_body(),
					});
				}
			} catch(e: any) {
				console.error(e);
				window.alert(e?.data);
				return;
			}

			this.$emit("data_saved", res);

			if(go_back) {
				this.$emit("back");
			} else {
				this.tag = structuredClone(toRaw(this.default_tag));
				this.tags = await $fetch("/api/v1/tags/all");
				(this.$refs.first_input as any).focus();
			}
		},
		
		get_body() {
			return {
				id: this.tag.id,
				name: this.tag.name,
				parent_id: typeof this.tag.parent_id == "string" && this.tag.parent_id.length == 36 ? this.tag.parent_id : undefined,
			} as Tag;
 		},
	},
}
</script>

<style lang="sass" scoped>
div#wrapper
	width: 350px

label
	display: flex
	input, select
		flex-grow: 1
</style>