<template>
	<div id="main">
		<div id="formWrapper">
			<div class="formInput" v-for="(field, index) in config.fields" :key="index">
			
				<div v-if="field.type == 'number'">
					<label>{{`${field.label}: `}}</label>
					<input type="number" v-model="config.data[field.property]" :step="field.step" :disabled="field.disabled" :ref="'forminput' + index">
					<span v-if="field.suffix == 'currencyOfAccountSymbol'">{{$store.state.currencies.filter(y => y.id == $store.state.accounts.filter(x => x.id == config.data.accountId)[0].defaultCurrency)[0].symbol}}</span>
				</div>

				<div v-else-if="field.type == 'string'">
					<label>{{`${field.label}: `}}</label>
					<input type="text" v-model="config.data[field.property]" :disabled="field.disabled" :ref="'forminput' + index">
				</div>

				<div v-else-if="field.type == 'timestamp'">
					<label>{{`${field.label}: `}}</label>
					<input type="datetime-local" v-model="config.data[field.property]" :disabled="field.disabled" :ref="'forminput' + index">
				</div>

				<div v-else-if="field.type == 'currency'">
					<label>{{`${field.label}: `}}</label>
					<select v-model="config.data[field.property]" :ref="'forminput' + index">
						<option v-for="(currency, cindex) in $store.state.currencies" :key="cindex" :value="currency.id">{{currency.name}}</option>
					</select>
				</div>

				<div v-else-if="field.type == 'account'">
					<label>{{`${field.label}: `}}</label>
					<select v-model="config.data[field.property]" :ref="'forminput' + index">
						<option v-for="(account, aindex) in $store.state.accounts" :key="aindex" :value="account.id">{{account.name}}</option>
					</select>
					<button v-if="field.addNew" class="secondary" @click="subForm = 'account'" tabindex="-1">New</button>	
				</div>

				<div v-else-if="field.type == 'recipient'">
					<label>{{`${field.label}: `}}</label>
					<select v-model="config.data[field.property]" :ref="'forminput' + index">
						<option v-for="(recipient, rindex) in $store.state.recipients" :key="rindex" :value="recipient.id">{{recipient.name}}</option>
					</select>	
					<button v-if="field.addNew" class="secondary" @click="subForm = 'recipient'" tabindex="-1">New</button>	
				</div>

				<div v-else-if="field.type == 'tags'">
					<CustomSelect
						:selectData="selectData"
						v-on:update="tagUpdate"
					/>
					<button v-if="field.addNew" class="secondary" @click="subForm = 'tags'" tabindex="-1">New</button>	
				</div>

				<div v-else-if="field.type == 'singleTag'">
					<label>{{`${field.label}: `}}</label>
					<select v-model="config.data[field.property]" :ref="'forminput' + index">
						<option value=""></option>
						<option v-for="(item, tindex) in $store.state.tags" :key="tindex" :value="item.id">{{item.name}}</option>
					</select>
				</div>
			</div>
			<button class="green" @click="send(true)">Save</button>
			<button class="red" @click="$emit('back')">Cancel</button>
			<button class="green" v-if="!config.noSaveAndNew" @click="send(false)">Save and New</button>
			<button class="red" v-if="config.deletable" @click="deleteThis">Delete</button>
		</div>

		<div v-if="subForm == 'account'" class="form">
			<DetailsForm
				:config="{...$detailPageConfig.account, noSaveAndNew: true}"
				v-on:back="closeSubForm"
			/>
		</div>

		<div v-if="subForm == 'recipient'" class="form">
			<DetailsForm
				:config="{...$detailPageConfig.recipient, noSaveAndNew: true}"
				v-on:back="closeSubForm"
			/>
		</div>

		<div v-if="subForm == 'tags'" class="form">
			<DetailsForm
				:config="{...$detailPageConfig.tags, noSaveAndNew: true}"
				v-on:back="closeSubForm"
			/>
		</div>

	</div>
</template>

<script>

export default {
	data: () => ({
		subForm: null,
		selectData: {}
	}),

	props: {
		config: Object
	},

	created() {
		this.config.data.tagIds = Array.isArray(this.config.data.tagIds) ? [...this.config.data.tagIds] : [null];
		this.updateSelectData();
	},

	mounted() {
		this.$nextTick(() => {this.$refs.forminput1[0].focus()});
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
		},

		async closeSubForm() {
			switch(this.subForm) {
				case 'account': {
					await this.$store.dispatch("fetchAccounts");
					this.$detailPageConfig.account.data = {...this.$detailPageConfig.account.defaultData};
				};
				case 'recipient': {
					await this.$store.dispatch("fetchRecipients");
					this.$detailPageConfig.recipient.data = {...this.$detailPageConfig.recipient.defaultData};
				};
				case 'tags': {
					await this.$store.dispatch("fetchTags");
					this.updateSelectData();
					this.$detailPageConfig.tags.data = {...this.$detailPageConfig.tags.defaultData};
				};
			}

			this.subForm = null;
		}
	}
}
</script>