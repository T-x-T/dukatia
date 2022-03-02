<template>
	<div id="main">
		<div id="form" v-for="(field, index) in config.fields" :key="index">
		
			<div v-if="field.type == 'number'">
				<label>{{`${field.label}: `}}</label>
				<input type="number" v-model="config.data[field.property]" :step="field.step" :disabled="field.disabled">
				<span v-if="field.suffix == 'currencyOfAccountSymbol'">{{$store.state.currencies.filter(y => y.id == $store.state.accounts.filter(x => x.id == config.data.accountId)[0].defaultCurrency)[0].symbol}}</span>
			</div>

			<div v-else-if="field.type == 'string'">
				<label>{{`${field.label}: `}}</label>
				<input type="text" v-model="config.data[field.property]" :disabled="field.disabled">
			</div>

			<div v-else-if="field.type == 'timestamp'">
				<label>{{`${field.label}: `}}</label>
				<input type="datetime-local" v-model="config.data[field.property]" :disabled="field.disabled">
			</div>

			<div v-else-if="field.type == 'currency'">
				<label>{{`${field.label}: `}}</label>
				<select v-model="config.data[field.property]">
					<option v-for="(currency, cindex) in $store.state.currencies" :key="cindex" :value="currency.id">{{currency.name}}</option>
				</select>
			</div>

			<div v-else-if="field.type == 'account'">
				<label>{{`${field.label}: `}}</label>
				<select v-model="config.data[field.property]">
					<option v-for="(account, aindex) in $store.state.accounts" :key="aindex" :value="account.id">{{account.name}}</option>
				</select>	
			</div>

			<div v-else-if="field.type == 'recipient'">
				<label>{{`${field.label}: `}}</label>
				<select v-model="config.data[field.property]">
					<option v-for="(recipient, rindex) in $store.state.recipients" :key="rindex" :value="recipient.id">{{recipient.name}}</option>
				</select>	
			</div>

			<div v-else-if="field.type == 'tags'">
				<CustomSelect
					:selectData="selectData"
					v-on:update="tagUpdate"
				/>
			</div>

			<div v-else-if="field.type == 'singleTag'">
				<label>{{`${field.label}: `}}</label>
				<select v-model="config.data[field.property]">
					<option value=""></option>
					<option v-for="(item, tindex) in $store.state.tags" :key="tindex" :value="item.id">{{item.name}}</option>
				</select>
			</div>
		</div>
		<button class="green" @click="send(true)">Save</button>
		<button class="red" @click="$emit('back')">Cancel</button>
		<button class="green" @click="send(false)">Save and New</button>
		<button class="red" @click="deleteThis">Delete</button>
	</div>
</template>

<script>
export default {
	props: {
		config: Object
	},

	created() {
		this.config.data.tagIds = Array.isArray(this.config.data.tagIds) ? [...this.config.data.tagIds] : [null];
		this.updateSelectData();
	},

	methods: {
		tagUpdate(selected) {
			this.config.data.tagIds = selected;
		},

		async send(goBack) {
			if(typeof this.config.data.id == "number") {
				await this.$axios.$put(`${this.config.apiEndpoint}/${this.config.data.id}`, this.config.prepareForApi(this.config.data));
			} else {
				await this.$axios.$post(this.config.apiEndpoint, this.config.prepareForApi(this.config.data));
			}

			if(goBack) {
				this.$emit("back");
			} else {
				this.config.data = this.config.defaultData;
				if(this.config.resetDefaultCurrencyId) this.config.data.defaultCurrencyId = this.config.data.defaultCurrency.id;
				this.updateSelectData();
			}
		},

		updateSelectData() {
			this.selectData = null;
			this.selectData = {
				options: [...this.$store.state.tags.map(x => ({id: x.id, name: x.name}))],
				selected: this.config.data.tagIds ? [...this.config.data.tagIds] : undefined,
				label: "Tags:"
			}
		},

		async deleteThis() {
			await this.$axios.$delete(`${this.config.apiEndpoint}/${this.config.data.id}`);
			this.$emit("back");
		}
	}
}
</script>