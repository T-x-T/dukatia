<template>
	<div>
		<div id="formWrapper">
			<div class="formInput" v-for="(field, index) in config.fields" :key="index">
			
				<div v-if="field.type == 'number'">
					<label>{{`${field.label}: `}}</label>
					<input type="number" v-model="config.data[field.property]" :step="field.step" :disabled="field.disabled || (field.initial && config.data.id !== undefined) as boolean" :ref="'forminput' + index">
					<span v-if="field.suffix == 'currencyOfAccountSymbol'">{{(currencies as Currency[]).filter(y => y.id == (accounts as Account[]).filter(x => x.id == config.data.account_id)[0]?.default_currency_id)[0]?.symbol}}</span>
				</div>

				<div v-else-if="field.type == 'string'">
					<label>{{`${field.label}: `}}</label>
					<input type="text" v-model="config.data[field.property]" :disabled="field.disabled || (field.initial && config.data.id !== undefined) as boolean" :ref="'forminput' + index">
				</div>

				<div v-else-if="field.type == 'timestamp'">
					<label>{{`${field.label}: `}}</label>
					<input type="datetime-local" v-model="config.data[field.property]" :disabled="field.disabled || (field.initial && config.data.id !== undefined) as boolean" :ref="'forminput' + index">
				</div>

				<div v-else-if="field.type == 'currency'">
					<label>{{`${field.label}: `}}</label>
					<select v-model="config.data[field.property]" :disabled="field.disabled || (field.initial && config.data.id !== undefined) as boolean" :ref="'forminput' + index">
						<option v-for="(currency, cindex) in currencies" :key="cindex" :value="currency.id">{{currency.name}}</option>
					</select>
				</div>

				<div v-else-if="field.type == 'account'">
					<label>{{`${field.label}: `}}</label>
					<select v-model="config.data[field.property]" :disabled="field.disabled || (field.initial && config.data.id !== undefined) as boolean" :ref="'forminput' + index">
						<option v-for="(account, aindex) in accounts" :key="aindex" :value="account.id">{{account.name}}</option>
					</select>
					<button v-if="field.addNew" class="green" @click="subForm = 'account'" tabindex="-1">New</button>	
				</div>

				<div v-else-if="field.type == 'recipient'">
					<label>{{`${field.label}: `}}</label>
					<select v-model="config.data[field.property]" :disabled="field.disabled || (field.initial && config.data.id !== undefined) as boolean" :ref="'forminput' + index">
						<option v-for="(recipient, rindex) in recipients" :key="rindex" :value="recipient.id">{{recipient.name}}</option>
					</select>	
					<button v-if="field.addNew" class="green" @click="subForm = 'recipient'" tabindex="-1">New</button>	
				</div>

				<div v-else-if="field.type == 'asset'">
					<label>{{`${field.label}: `}}</label>
					<select v-model="config.data[field.property]" :disabled="field.disabled || (field.initial && config.data.id !== undefined) as boolean" :ref="'forminput' + index">
						<option v-for="(asset, aindex) in [...assets].sort((a, b) => a.name > b.name ? 1 : -1)" :key="aindex" :value="asset.id">{{asset.name}}</option>
					</select>	
				</div>

				<div v-else-if="field.type == 'tags'">
					<CustomSelect
						v-if="Object.keys(selectData).length > 0 && renderCustomSelect"
						:selectData="selectData"
						v-on:update="tagUpdate"
					/>
					<button v-if="field.addNew" class="green" @click="subForm = 'tags'" tabindex="-1">New</button>	
				</div>

				<div v-else-if="field.type == 'singleTag'">
					<label>{{`${field.label}: `}}</label>
					<select v-model="config.data[field.property]" :disabled="field.disabled || (field.initial && config.data.id !== undefined) as boolean" :ref="'forminput' + index">
						<option value=""></option>
						<option v-for="(item, tindex) in tags" :key="tindex" :value="item.id">{{item.name}}</option>
					</select>
				</div>

				<div v-else-if="field.type == 'positions'">
					<label>{{`${field.label}: `}}</label>
					<div v-for="(position_data, position_index) in config.data[field.property]">
						<label>Amount: </label>
						<input type="number" v-model="config.data[field.property][position_index].amount">
						<span>{{(currencies as Currency[]).filter(y => y.id == (accounts as Account[]).filter(x => x.id == config.data.account_id)[0]?.default_currency_id)[0]?.symbol}}</span>
						<br>
						<label>Comment: </label>
						<input type="text" v-model="config.data[field.property][position_index].comment">
						<br>
						<label>Tag: </label>
						<select v-model="config.data[field.property][position_index].tag_id">
							<option value=""></option>
							<option v-for="(item, tindex) in tags" :key="tindex" :value="item.id">{{item.name}}</option>
						</select>
						<br>
						<button class="red" @click="config.data[field.property].splice(position_index, 1)">Delete Position</button>
						<hr>
					</div>
					<button class="green" @click="config.data[field.property].push({...(config as any).defaultData[field.property][0]})">Add Position</button>
				</div>
			</div>
			<button class="green" @click="send(true)">Save</button>
			<button class="red" @click="$emit('back')">Cancel</button>
			<button class="green" v-if="!config.noSaveAndNew" @click="send(false)">Save and New</button>
			<button class="red" v-if="config.deletable" @click="deleteThis">Delete</button>
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
		subForm: null as null | "account" | "recipient" | "tags",
		selectData: {} as SelectData,
		tagsManuallyChanged: false,
		tags: [] as Tag[],
		assets: [] as Asset[],
		recipients: [] as Recipient[],
		accounts: [] as Account[],
		currencies: [] as Currency[],
		renderCustomSelect: true,
	}),

	props: {
		config: {
			type: Object as PropType<DetailFormConfig>,
			required: true,
		}
	},

	async mounted() {
		this.tags = await $fetch("/api/v1/tags/all");
		this.assets = await $fetch("/api/v1/assets/all");
		this.recipients = await $fetch("/api/v1/recipients/all");
		this.accounts = await $fetch("/api/v1/accounts/all");
		this.currencies = await $fetch("/api/v1/currencies/all");
		
		this.config.data.tag_ids = Array.isArray(this.config.data.tag_ids) ? [...this.config.data.tag_ids] : [null];
		await this.updateSelectData();

		this.$nextTick(() => {(this as any).$refs.forminput1?.[0].focus()});
	},

	methods: {
		tagUpdate(selected: number[]) {
			this.tagsManuallyChanged = true;
			this.config.data.tag_ids = selected;
		},

		async send(goBack: boolean) {
			let res = null;
			try {
				if(typeof this.config.data.id == "number") {
					res = await $fetch(`${this.config.apiEndpoint}/${this.config.data.id}`, {
						method: "PUT",
						body: await this.config.prepareForApi(this.config.data)
					});
				} else {
					res = await $fetch(this.config.apiEndpoint, {
						method: "POST",
						body: await this.config.prepareForApi(this.config.data)
					});
				}
			} catch(e: any) {
				console.error(e?.data?.data);
				window.alert(e?.data?.data);
				return;
			}

			if(!this.config.noGoBackOnSave && goBack) {
				this.$emit("back");
			} else {
				this.$emit("updateData", res);
				
				if(this.config.noGoBackOnSave) return;

				this.tagsManuallyChanged = false;
				this.config.data = {...this.config.defaultData};
				(this as any).$refs.forminput1[0].focus()
				if(this.config.reset_default_currency_id) this.config.data.default_currency_id = this.config.data.default_currency.id;
				this.renderCustomSelect = false;
				this.$nextTick(() => {
					this.renderCustomSelect = true;
					this.updateSelectData();
				})
			}
		},

		async updateSelectData() {
			this.selectData = {
				options: [...this.tags.map(x => ({id: (Number.isInteger(x.id) ? x.id : -1) as number, name: x.name}))],
				selected: this.config.data.tag_ids ? [...this.config.data.tag_ids] : [],
				label: "Tags:"
			};
		},

		async deleteThis() {
			try {
				await $fetch(`${this.config.apiEndpoint}/${this.config.data.id}`, { method: "DELETE" });
				this.$emit("back");
			} catch(e: any) {
				console.error(e);
				window.alert(e);
			}
		},

		async closeSubForm() {
			switch(this.subForm) {
				case "account": {
					this.accounts = await $fetch("/api/v1/accounts/all");
					this.$detailPageConfig().account.data = {...this.$detailPageConfig().account.defaultData};
				};
				case "recipient": {
					this.recipients = await $fetch("/api/v1/recipients/all");
					this.$detailPageConfig().recipient.data = {...this.$detailPageConfig().recipient.defaultData};
				};
				case "tags": {
					this.tags = await $fetch("/api/v1/tags/all");
					await this.updateSelectData();
					this.$detailPageConfig().tags.data = {...this.$detailPageConfig().tags.defaultData};
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