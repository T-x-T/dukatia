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
		context.commit("accounts", (await this.$axios.$get("/api/v1/accounts/all")).sort((a, b) => a.id - b.id));
	},

	async fetchCurrencies(context) {
		context.commit("currencies", (await this.$axios.$get("/api/v1/currencies/all")).sort((a, b) => a.id - b.id));
	},

	async fetchRecipients(context) {
		context.commit("recipients", (await this.$axios.$get("/api/v1/recipients/all")).sort((a, b) => a.id - b.id));
	},

	async fetchTransactions(context) {
		context.commit("transactions", (await this.$axios.$get("/api/v1/transactions/all")).sort((a, b) => a.id - b.id));
	}
}