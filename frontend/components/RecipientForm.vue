<template>
	<div id="wrapper">
		<button class="orange" @click="$emit('back')">Back</button>
		
		<label>
			ID:
			<input type="number" v-model="recipient.id" disabled>
		</label>
		<label>
			Name:
			<input type="text" v-model="recipient.name" ref="first_input">
		</label>
		<InputMultiSelect
			v-if="tags_select_data && Object.keys(tags_select_data).length > 0"
			:selectData="tags_select_data"
			v-on:update="(selected: number[]) => recipient.tag_ids = selected"
			style="margin-right: 5px;"
		/>

		<br>
		<button class="green" @click="save(true)">Save</button>
		<button class="orange" @click="$emit('back')">Cancel</button>
		<button class="green" @click="save(false)">Save and New</button>
	</div>
</template>

<script lang="ts">
export default {
	data: () => ({
		recipient: {} as Recipient,
		tags_select_data: {} as SelectData | null,
		tags: [] as Tag[],
		default_recipient: {
			id: undefined,
			name: "",
			tag_ids: []
		} as Recipient,
	}),

	emits: ["back", "data_saved"],

	props: {
		data: {
			type: Object as PropType<Recipient>,
			required: false,
		}
	},

	async mounted() {
		this.recipient = this.data ? this.data : structuredClone(toRaw(this.default_recipient));

		this.tags = await $fetch("/api/v1/tags/all");
		this.update_tags_select_data();
		(this.$refs.first_input as any).focus();
	},

	methods: {
		async save(go_back: boolean) {
			let res = {} as Recipient;

			try {
				if(typeof this.recipient.id == "number") {
					res = await $fetch(`/api/v1/recipients/${this.recipient.id}`, {
						method: "PUT",
						body: this.get_body(),
					});
				} else {
					res = await $fetch("/api/v1/recipients", {
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
				this.recipient = structuredClone(toRaw(this.default_recipient));
				this.update_tags_select_data();
				(this.$refs.first_input as any).focus();
			}
		},

		update_tags_select_data() {
			this.tags_select_data = null;
			this.$nextTick(() => {
				this.tags_select_data = {
					options: [...this.tags.map(x => ({id: (Number.isInteger(x.id) ? x.id : -1) as number, name: x.name}))],
					selected: this.recipient.tag_ids,
					label: "Tags:"
				}
			});
		},
		
		get_body() {
			return {
				id: this.recipient.id,
				name: this.recipient.name,
				tag_ids: Array.isArray(this.recipient.tag_ids) ? this.recipient.tag_ids : undefined
			} as Recipient;
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