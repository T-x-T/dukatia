export const state = () => ({
	accounts: [],
	currencies: [],
	recipients: [],
	transactions: []
});

export const mutations = {
	accounts(state, payload) {
		state.accounts = payload;
	},

	currencies(state, payload) {
		state.currencies = payload;
	},

	recipients(state, payload) {
		state.recipients = payload;
	},

	transactions(state, payload) {
		state.transactions = payload;
	}
}

export const actions = {
	async fetchAccounts(context) {
		context.commit("accounts", await this.$axios.$get("/api/v1/accounts/all"));
	},

	async fetchCurrencies(context) {
		context.commit("currencies", await this.$axios.$get("/api/v1/currencies/all"));
	},

	async fetchRecipients(context) {
		context.commit("recipients", await this.$axios.$get("/api/v1/recipients/all"));
	},

	async fetchTransactions(context) {
		context.commit("transactions", await this.$axios.$get("/api/v1/transactions/all"));
	}
}