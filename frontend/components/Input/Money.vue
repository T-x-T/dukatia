<template>
	<input type="text" v-model="value_for_display" :disabled="disabled" @change="update_value">
</template>

<script lang="ts">
export default {
	data: () => ({
		value: {} as Money,
		value_for_display: "",
	}),
	
	props: {
		initial_value: {
			type: Object as PropType<Money>,
			required: true,
		},
		disabled: {
			type: Boolean,
			required: false,
		}
	},

	mounted() {
		this.value = structuredClone(toRaw(this.initial_value));
		this.value_for_display = this.update_value_for_display();
	},

	watch: {
		initial_value: {
			handler() {
				this.value = structuredClone(toRaw(this.initial_value));
				this.value_for_display = this.update_value_for_display();
			},
			deep: true,
		},
	},

	methods: {
		update_value_for_display() {
			return `${this.value.major >= 0 && this.value.is_negative ? "-" : ""}${this.value.major}.${this.value.minor.toString().padStart(this.value.minor_in_major.toString().length - 1, "0")}${this.value.symbol}`;
		},

		update_value() {
			let major = Number(this.value_for_display.split(/[.,]/)[0]);
			let minor = this.value_for_display.split(/[.,]/).length > 1 ? Number(this.value_for_display.split(/[.,]/)[1].replace(this.value.symbol, "")) : 0;
			while (minor >= this.value.minor_in_major) {
				major += 1;
				minor -= this.value.minor_in_major;
			}

			this.value.major = major;
			this.value.minor = minor;

			this.value.is_negative = this.value_for_display.startsWith("-");

			this.value_for_display = this.update_value_for_display();
			this.$emit("update", structuredClone(toRaw(this.value)));
		}
	}
}
</script>