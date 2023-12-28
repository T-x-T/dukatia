<template>
	<div id="wrapper">
		<button class="orange" @click="$emit('back')">Back</button>
		
		<label>
			ID:
			<input type="number" v-model="currency.id" disabled>
		</label>
		<label>
			Name:
			<input type="text" v-model="currency.name" ref="first_input">
		</label>
		<label>
			Minor in Major:
			<input type="number" v-model="currency.minor_in_major">
		</label>
		<label>
			Symbol:
			<input type="text" v-model="currency.symbol">
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
		currency: {} as Currency,
		default_currency: {
			id: undefined,
			name: "",
			minor_in_major: 100,
			symbol: "",
		} as Currency,
	}),

	emits: ["back", "data_saved"],

	props: {
		data: {
			type: Object as PropType<Currency>,
			required: false,
		}
	},

	async mounted() {
		this.currency = this.data && Object.keys(this.data).length > 0 ? this.data : structuredClone(toRaw(this.default_currency));
		(this.$refs.first_input as any).focus();
	},

	methods: {
		async save(go_back: boolean) {
			let res = {} as Currency;

			try {
				if(typeof this.currency.id == "number") {
					res = await $fetch(`/api/v1/currencies/${this.currency.id}`, {
						method: "PUT",
						body: this.get_body(),
					});
				} else {
					res = await $fetch("/api/v1/currencies", {
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
				this.currency = structuredClone(toRaw(this.default_currency));
				(this.$refs.first_input as any).focus();
			}
		},
		
		get_body() {
			return {
				id: this.currency.id,
				name: this.currency.name,
				minor_in_major: Number(this.currency.minor_in_major),
				symbol: this.currency.symbol
			} as Currency;
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