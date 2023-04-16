<template>
	<div>
		<div id="formWrapper">
			<div class="formInput" v-for="(field, index) in (config as any).fields" :key="index">
			
				<div v-if="field.type == 'number'">
					<label>{{`${field.label}: `}}</label>
					<input type="number" v-model="(config as any).data[field.property]" :step="field.step" :disabled="field.disabled || (field.initial && (config as any).data.id !== '')" :ref="'forminput' + index">
					<span v-if="field.suffix == 'currencyOfAccountSymbol'">{{(currencies as any).filter((y: any) => y.id == (accounts as any).filter((x: any) => x.id == (config as any).data.account_id)[0].default_currency_id)[0].symbol}}</span>
				</div>

				<div v-else-if="field.type == 'string'">
					<label>{{`${field.label}: `}}</label>
					<input type="text" v-model="(config as any).data[field.property]" :disabled="field.disabled || (field.initial && (config as any).data.id !== '')" :ref="'forminput' + index">
				</div>

				<div v-else-if="field.type == 'timestamp'">
					<label>{{`${field.label}: `}}</label>
					<input type="datetime-local" v-model="(config as any).data[field.property]" :disabled="field.disabled || (field.initial && (config as any).data.id !== '')" :ref="'forminput' + index">
				</div>

				<div v-else-if="field.type == 'currency'">
					<label>{{`${field.label}: `}}</label>
					<select v-model="(config as any).data[field.property]" :disabled="field.disabled || (field.initial && (config as any).data.id !== '')" :ref="'forminput' + index">
						<option v-for="(currency, cindex) in currencies" :key="cindex" :value="(currency as any).id">{{(currency as any).name}}</option>
					</select>
				</div>

				<div v-else-if="field.type == 'account'">
					<label>{{`${field.label}: `}}</label>
					<select v-model="(config as any).data[field.property]" :disabled="field.disabled || (field.initial && (config as any).data.id !== '')" :ref="'forminput' + index">
						<option v-for="(account, aindex) in accounts" :key="aindex" :value="(account as any).id">{{(account as any).name}}</option>
					</select>
					<button v-if="field.addNew" class="secondary" @click="(subForm as any) = 'account'" tabindex="-1">New</button>	
				</div>

				<div v-else-if="field.type == 'recipient'">
					<label>{{`${field.label}: `}}</label>
					<select v-model="(config as any).data[field.property]" :disabled="field.disabled || (field.initial && (config as any).data.id !== '')" :ref="'forminput' + index">
						<option v-for="(recipient, rindex) in recipients" :key="rindex" :value="(recipient as any).id">{{(recipient as any).name}}</option>
					</select>	
					<button v-if="field.addNew" class="secondary" @click="(subForm as any) = 'recipient'" tabindex="-1">New</button>	
				</div>

				<div v-else-if="field.type == 'asset'">
					<label>{{`${field.label}: `}}</label>
					<select v-model="(config as any).data[field.property]" :disabled="field.disabled || (field.initial && (config as any).data.id !== '')" :ref="'forminput' + index">
						<option v-for="(asset, aindex) in [...assets].sort((a: any, b: any) => a.name > b.name ? 1 : -1)" :key="aindex" :value="(asset as any).id">{{(asset as any).name}}</option>
					</select>	
				</div>

				<div v-else-if="field.type == 'tags'">
					<CustomSelect
						v-if="selectData"
						:selectData="selectData"
						v-on:update="tagUpdate"
					/>
					<button v-if="field.addNew" class="secondary" @click="(subForm as any) = 'tags'" tabindex="-1">New</button>	
				</div>

				<div v-else-if="field.type == 'singleTag'">
					<label>{{`${field.label}: `}}</label>
					<select v-model="(config as any).data[field.property]" :disabled="field.disabled || (field.initial && (config as any).data.id !== '')" :ref="'forminput' + index">
						<option value=""></option>
						<option v-for="(item, tindex) in tags" :key="tindex" :value="(item as any).id">{{(item as any).name}}</option>
					</select>
				</div>
			</div>
			<button class="green" @click="send(true)">Save</button>
			<button class="red" @click="$emit('back')">Cancel</button>
			<button class="green" v-if="!(config as any).noSaveAndNew" @click="send(false)">Save and New</button>
			<button class="red" v-if="(config as any).deletable" @click="deleteThis">Delete</button>
		</div>

		<div v-if="subForm == 'account'" class="form">
			<DetailsForm
				:config="{...$detailPageConfig().account, noSaveAndNew: true}"
				v-on:back="closeSubForm"
			/>
		</div>

		<div v-if="subForm == 'recipient'" class="form">
			<DetailsForm
				:config="{...$detailPageConfig().recipient, noSaveAndNew: true}"
				v-on:back="closeSubForm"
			/>
		</div>

		<div v-if="subForm == 'tags'" class="form">
			<DetailsForm
				:config="{...$detailPageConfig().tags, noSaveAndNew: true}"
				v-on:back="closeSubForm"
			/>
		</div>

	</div>
</template>

<script lang="ts">
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
		this.tags = await $fetch("/api/v1/tags/all");
		this.assets = await $fetch("/api/v1/assets/all");
		this.recipients = await $fetch("/api/v1/recipients/all");
		this.accounts = await $fetch("/api/v1/accounts/all");
		this.currencies = await $fetch("/api/v1/currencies/all");
		(this as any).config.data.tag_ids = Array.isArray((this as any).config.data.tag_ids) ? [...(this as any).config.data.tag_ids] : [null];
		await this.updateSelectData();
	},

	mounted() {
		this.$nextTick(() => {(this as any).$refs.forminput1?.[0].focus()});
	},

	methods: {
		tagUpdate(selected: any) {
			this.tagsManuallyChanged = true;
			(this as any).config.data.tag_ids = selected;
		},

		async send(goBack: boolean) {
			let res = null;
			try {
				if(typeof (this as any).config.data.id == "number") {
					res = await $fetch(`${(this as any).config.apiEndpoint}/${(this as any).config.data.id}`, {
						method: "PUT",
						body: (this as any).config.prepareForApi((this as any).config.data)
					});
				} else {
					res = await $fetch((this as any).config.apiEndpoint, {
						method: "POST",
						body: (this as any).config.prepareForApi((this as any).config.data)
					});
				}
			} catch(e: any) {
				console.error(e?.data?.data);
				window.alert(e?.data?.data?.error);
				return;
			}

			if(!(this as any).config.noGoBackOnSave && goBack) {
				this.$emit("back");
			} else {
				this.$emit("updateData", res);
				
				if((this as any).config.noGoBackOnSave) return;

				this.tagsManuallyChanged = false;
				(this as any).config.data = {...(this as any).config.defaultData};
				(this as any).$refs.forminput1[0].focus()
				if((this as any).config.resetdefault_currency_id) (this as any).config.data.default_currency_id = (this as any).config.data.default_currency.id;
				await this.updateSelectData();
			}
		},

		async updateSelectData() {
			(this as any).selectData = null;
			this.$nextTick(() => {
				this.selectData = {
					options: [...this.tags.map((x: any) => ({id: x.id, name: x.name}))],
					selected: (this as any).config.data.tag_ids ? [...(this as any).config.data.tag_ids] : [],
					label: "Tags:"
				}
			});
		},

		async deleteThis() {
			try {
				await $fetch(`${(this as any).config.apiEndpoint}/${(this as any).config.data.id}`, { method: "DELETE" });
			} catch(e: any) {
				console.error(e?.data?.data);
				window.alert(e?.data?.data?.error);
				return;
			}
			this.$emit("back");
		},

		async closeSubForm() {
			switch((this as any).subForm) {
				case 'account': {
					(this as any).accounts = await $fetch("/api/v1/accounts/all");
					this.$detailPageConfig().account.data = {...this.$detailPageConfig().account.defaultData};
				};
				case 'recipient': {
					(this as any).recipients = await $fetch("/api/v1/recipients/all");
					this.$detailPageConfig().recipient.data = {...this.$detailPageConfig().recipient.defaultData};
				};
				case 'tags': {
					this.tags = (await $fetch("/api/v1/tags/all"));
					await this.updateSelectData();
					this.$detailPageConfig().tags.data = {...this.$detailPageConfig().tags.defaultData};
				};
			}

			this.subForm = null;
		}
	},

	watch: {
		"config.data.recipient_id": async function(oldVal, newVal) {
			if((this as any).config.populateTagsUsingRecipient && !this.tagsManuallyChanged && typeof (this as any).config.data.id != "number") {
				const tag_idsOfRecipient = (this.recipients.filter((x: any) => x.id === (this as any).config.data.recipient_id)[0] as any).tag_ids;
				(this as any).config.data.tag_ids = tag_idsOfRecipient;
				await this.updateSelectData();
			}
		}
	}
}
</script>