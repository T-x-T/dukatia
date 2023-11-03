<template>
	<div>
		<p>Batch import Transactions</p>
		<p>Format:<br>account_id,recipient_id,timestamp,comment,major_amount,minor_amount<br>(No headers)</p>
		<input type="file" accept=".csv" @change="load_data">
		<br>
		<button @click="upload_data">Start import</button>
		<br>
		<p v-if="total > 0 && completed === 0">{{ `Detected ${total} transactions to import` }}</p>
		<p v-if="total > 0 && completed > 0">{{ `${completed}/${total} imported` }}</p>
	</div>
</template>

<script lang="ts">
export default {
	data: () => ({
		import_file_contents: "",
		total: 0,
		completed: 0,
	}),

	methods: {
		load_data(x: Event) {
			const reader = new FileReader();
			reader.addEventListener("load", () => {
				this.import_file_contents = reader.result as string;
				this.total = this.import_file_contents.split("\n").length;
			});
			reader.readAsText(((x.target as HTMLInputElement).files as FileList)[0]);
		},

		async upload_data() {
			for (const line of this.import_file_contents.split("\n")) {
				if(line.length < 4) return;
				await $fetch("/api/v1/transactions", {
					method: "POST",
					body: this.turn_line_into_transaction(line)
				});
				this.completed++;
			}
		},

		turn_line_into_transaction(line: string) {
			const parts = line.split(",");

			return {
				account_id: Number(parts[0]),
				recipient_id: Number(parts[1]),
				status: 1,
				timestamp: new Date(parts[2]),
				comment: parts[3],
				positions: [{major_amount: Number(parts[4]), minor_amount: Number(parts[5])}],
			};
		}
	}
}
</script>