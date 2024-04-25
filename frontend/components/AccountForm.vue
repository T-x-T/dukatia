<template>
	<div id="wrapper">
		<button class="orange" @click="$emit('back')">Back</button>
		
		<label>
			ID:
			<input type="string" v-model="account.id" disabled>
		</label>
		<label>
			Name:
			<input type="text" v-model="account.name" ref="first_input">
		</label>
		<label>
			Currency:
			<select v-model="account.default_currency_id">
				<option v-for="(currency, index) in currencies" :key="index" :value="currency.id">{{currency.name}}</option>
			</select>
		</label>
		<InputMultiSelect
			v-if="tags_select_data && Object.keys(tags_select_data).length > 0"
			:selectData="tags_select_data"
			@update="(selected: string[]) => account.tag_ids = selected"
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
		account: {} as Account,
		tags_select_data: {} as SelectData | null,
		tags: [] as Tag[],
		currencies: [] as Currency[],
		default_account: {
			id: undefined,
			default_currency_id: 0,
			name: "",
			tag_ids: []
		} as Account,
	}),

	emits: ["back", "data_saved"],

	props: {
		data: {
			type: Object as PropType<Account>,
			required: false,
		}
	},

	async mounted() {
		this.account = this.data && Object.keys(this.data).length > 0 ? this.data : structuredClone(toRaw(this.default_account));

		this.tags = await $fetch("/api/v1/tags/all");
		this.currencies = await $fetch("/api/v1/currencies/all");
		this.update_tags_select_data();
		(this.$refs.first_input as any).focus();
	},

	methods: {
		async save(go_back: boolean) {
			let res = {} as Account;

			try {
				if(typeof this.account.id == "string" && this.account.id.length == 36) {
					res = await $fetch(`/api/v1/accounts/${this.account.id}`, {
						method: "PUT",
						body: this.get_body(),
					});
				} else {
					res = await $fetch("/api/v1/accounts", {
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
				this.account = structuredClone(toRaw(this.default_account));
				this.update_tags_select_data();
				(this.$refs.first_input as any).focus();
			}
		},

		update_tags_select_data() {
			this.tags_select_data = null;
			this.$nextTick(() => {
				this.tags_select_data = {
					options: [...this.tags.map(x => ({id: (typeof x.id == "string" && x.id.length == 36 ? x.id : ""), name: x.name}))],
					selected: this.account.tag_ids,
					label: "Tags:"
				}
			});
		},

		get_body() {
			return {
				id: this.account.id,
				name: this.account.name,
				default_currency_id: this.account.default_currency_id,
				tag_ids: Array.isArray(this.account.tag_ids) ? this.account.tag_ids : undefined
			} as Account;
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