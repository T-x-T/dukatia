<template>
	<div id="wrapper">
		<label>
			ID:
			<input type="number" v-model="asset.id" disabled>
		</label>
		<label>
			Name:
			<input type="text" v-model="asset.name" ref="first_input">
		</label>
		<label>
			Description:
			<input type="text" v-model="asset.description">
		</label>
		<label>
			Amount:
			<input type="number" v-model="asset.amount" disabled>
		</label>
		<label>
			Value per Unit:
			<InputMoney
			v-if="asset.value_per_unit && Object.keys(asset.value_per_unit).length > 0"
				:initial_value="asset.value_per_unit"
				:disabled="true"
			/>
		</label>
		<label>
			Currency:
			<select v-model="asset.currency_id">
				<option v-for="(currency, index) in currencies" :key="index" :value="currency.id">{{currency.name}}</option>
			</select>
		</label>
		<InputMultiSelect
			v-if="tags_select_data && Object.keys(tags_select_data).length > 0"
			:selectData="tags_select_data"
			v-on:update="(selected: number[]) => asset.tag_ids = selected"
			style="margin-right: 5px;"
		/>

		<br>
		<button class="green" @click="save">Save</button>
		<button class="orange" @click="$emit('back')">Cancel</button>
		<button class="red" @click="delete_this">Delete</button>
	</div>
</template>

<script lang="ts">
export default {
	data: () => ({
		asset: {} as Asset,
		tags_select_data: {} as SelectData | null,
		tags: [] as Tag[],
		currencies: [] as Currency[],
		default_asset: {
			id: undefined,
			name: "",
			description: "",
			amount: 0,
			value_per_unit: {major: 0, minor: 0, minor_in_major: 100, symbol: "€"},
			currency_id: 0,
			tag_ids: [],
			user_id: 0,
		} as Asset,
	}),

	emits: ["back", "data_saved"],

	props: {
		data: {
			type: Object as PropType<Asset>,
			required: false,
		}
	},

	async mounted() {
		this.asset = this.data && Object.keys(this.data).length > 0 ? structuredClone(toRaw(this.data)) : structuredClone(toRaw(this.default_asset));

		this.tags = await $fetch("/api/v1/tags/all");
		this.currencies = await $fetch("/api/v1/currencies/all");
		this.update_tags_select_data();
		(this.$refs.first_input as any).focus();
	},

	methods: {
		async save() {
			let res = {} as Asset;

			try {
				if(typeof this.asset.id == "number") {
					res = await $fetch(`/api/v1/assets/${this.asset.id}`, {
						method: "PUT",
						body: this.get_body(),
					});
				} else {
					res = await $fetch("/api/v1/assets", {
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
		},

		async delete_this() {
			try {
				await $fetch(`/api/v1/assets/${this.asset.id}`, { method: "DELETE" });
				this.$emit("back");
			} catch(e: any) {
				console.error(e);
				window.alert(e?.data);
			}
		},

		update_tags_select_data() {
			this.tags_select_data = null;
			this.$nextTick(() => {
				this.tags_select_data = {
					options: [...this.tags.map(x => ({id: (Number.isInteger(x.id) ? x.id : -1) as number, name: x.name}))],
					selected: this.asset.tag_ids,
					label: "Tags:"
				}
			});
		},

		get_body() {
			return {
				name: this.asset.name,
				description: this.asset.description,
				currency_id: this.asset.currency_id, 
				tag_ids: Array.isArray(this.asset.tag_ids) ? this.asset.tag_ids : undefined,
			} as Asset;
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