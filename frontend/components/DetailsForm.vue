<template>
	<div>
		<div id="formWrapper">
			<div class="formInput" v-for="(field, index) in config.fields" :key="index">
			
				<div v-if="field.type == 'number'">
					<label>{{`${field.label}: `}}</label>
					<input type="number" v-model="config.data[field.property]" :step="field.step" :disabled="field.disabled || (field.initial && config.data.id !== '')" :ref="'forminput' + index">
					<span v-if="field.suffix == 'currencyOfAccountSymbol'">{{currencies.filter(y => y.id == accounts.filter(x => x.id == config.data.account_id)[0].default_currency_id)[0].symbol}}</span>
				</div>

				<div v-else-if="field.type == 'string'">
					<label>{{`${field.label}: `}}</label>
					<input type="text" v-model="config.data[field.property]" :disabled="field.disabled || (field.initial && config.data.id !== '')" :ref="'forminput' + index">
				</div>

				<div v-else-if="field.type == 'timestamp'">
					<label>{{`${field.label}: `}}</label>
					<input type="datetime-local" v-model="config.data[field.property]" :disabled="field.disabled || (field.initial && config.data.id !== '')" :ref="'forminput' + index">
				</div>

				<div v-else-if="field.type == 'currency'">
					<label>{{`${field.label}: `}}</label>
					<select v-model="config.data[field.property]" :disabled="field.disabled || (field.initial && config.data.id !== '')" :ref="'forminput' + index">
						<option v-for="(currency, cindex) in currencies" :key="cindex" :value="currency.id">{{currency.name}}</option>
					</select>
				</div>

				<div v-else-if="field.type == 'account'">
					<label>{{`${field.label}: `}}</label>
					<select v-model="config.data[field.property]" :disabled="field.disabled || (field.initial && config.data.id !== '')" :ref="'forminput' + index">
						<option v-for="(account, aindex) in accounts" :key="aindex" :value="account.id">{{account.name}}</option>
					</select>
					<button v-if="field.addNew" class="secondary" @click="subForm = 'account'" tabindex="-1">New</button>	
				</div>

				<div v-else-if="field.type == 'recipient'">
					<label>{{`${field.label}: `}}</label>
					<select v-model="config.data[field.property]" :disabled="field.disabled || (field.initial && config.data.id !== '')" :ref="'forminput' + index">
						<option v-for="(recipient, rindex) in recipients" :key="rindex" :value="recipient.id">{{recipient.name}}</option>
					</select>	
					<button v-if="field.addNew" class="secondary" @click="subForm = 'recipient'" tabindex="-1">New</button>	
				</div>

				<div v-else-if="field.type == 'asset'">
					<label>{{`${field.label}: `}}</label>
					<select v-model="config.data[field.property]" :disabled="field.disabled || (field.initial && config.data.id !== '')" :ref="'forminput' + index">
						<option v-for="(asset, aindex) in [...assets].sort((a, b) => a.name > b.name ? 1 : -1)" :key="aindex" :value="asset.id">{{asset.name}}</option>
					</select>	
				</div>

				<div v-else-if="field.type == 'tags'">
					<CustomSelect
						v-if="selectData"
						:selectData="selectData"
						v-on:update="tagUpdate"
					/>
					<button v-if="field.addNew" class="secondary" @click="subForm = 'tags'" tabindex="-1">New</button>	
				</div>

				<div v-else-if="field.type == 'singleTag'">
					<label>{{`${field.label}: `}}</label>
					<select v-model="config.data[field.property]" :disabled="field.disabled || (field.initial && config.data.id !== '')" :ref="'forminput' + index">
						<option value=""></option>
						<option v-for="(item, tindex) in tags" :key="tindex" :value="item.id">{{item.name}}</option>
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
		selectData: {},
		tagsManuallyChanged: false,
		tags: [],
		assets: [],
		recipients: [],
		accounts: [],
		currencies: []
	}),

	props: {
		config: Object
	},

	async created() {
		this.tags = (await $fetch("/api/v1/tags/all"));
		this.assets = (await fetch("/api/v1/assets/all"));
		this.recipients = (await fetch("/api/v1/recipients/all"));
		this.accounts = (await fetch("/api/v1/accounts/all"));
		this.currencies = (await fetch("/api/v1/currencies/all"));
		this.config.data.tag_ids = Array.isArray(this.config.data.tag_ids) ? [...this.config.data.tag_ids] : [null];
		await this.updateSelectData();
	},

	mounted() {
		this.$nextTick(() => {this.$refs.forminput1?.[0].focus()});
	},

	methods: {
		tagUpdate(selected) {
			this.tagsManuallyChanged = true;
			this.config.data.tag_ids = selected;
		},

		async send(goBack) {
			let res = null;
			try {
				if(typeof this.config.data.id == "number") {
					res = await this.$axios.$put(`${this.config.apiEndpoint}/${this.config.data.id}`, this.config.prepareForApi(this.config.data));
				} else {
					res = await this.$axios.$post(this.config.apiEndpoint, this.config.prepareForApi(this.config.data));
				}
			} catch(e) {
				console.error(e.response);
				window.alert(e.response.data);
				return;
			}

			if(!this.config.noGoBackOnSave && goBack) {
				this.$emit("back");
			} else {
				this.$emit("updateData", res);
				
				if(this.config.noGoBackOnSave) return;

				this.tagsManuallyChanged = false;
				this.config.data = {...this.config.defaultData};
				this.$refs.forminput1[0].focus()
				if(this.config.resetdefault_currency_id) this.config.data.default_currency_id = this.config.data.default_currency.id;
				await this.updateSelectData();
			}
		},

		async updateSelectData() {
			this.selectData = null;
			this.$nextTick(() => {
				this.selectData = {
					options: [...this.tags.map(x => ({id: x.id, name: x.name}))],
					selected: this.config.data.tag_ids ? [...this.config.data.tag_ids] : [],
					label: "Tags:"
				}
			});
		},

		async deleteThis() {
			try {
				await this.$axios.$delete(`${this.config.apiEndpoint}/${this.config.data.id}`);
			} catch(e) {
				console.error(e.response);
				window.alert(e.response.data);
				return;
			}
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
					await this.updateSelectData();
					this.$detailPageConfig.tags.data = {...this.$detailPageConfig.tags.defaultData};
				};
			}

			this.subForm = null;
		}
	},

	watch: {
		"config.data.recipient_id": async function(oldVal, newVal) {
			if(this.config.populateTagsUsingRecipient && !this.tagsManuallyChanged && typeof this.config.data.id != "number") {
				const tag_idsOfRecipient = this.recipients.filter(x => x.id === this.config.data.recipient_id)[0].tag_ids;
				this.config.data.tag_ids = tag_idsOfRecipient;
				await this.updateSelectData();
			}
		}
	}
}
</script>