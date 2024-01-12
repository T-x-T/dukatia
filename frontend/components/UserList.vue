<template>
	<div>
		<h3>Users</h3>
		<button @click="add_popup_open = true">Add</button>
		<CustomTable 
			v-if="Object.keys(table_data).length > 0"
			:tableDataProp="table_data"
		/>
		<Popup v-if="add_popup_open" @close="add_popup_open = false">
			<label>
				Name:
				<input type="text" v-model="new_user.name">
			</label>
			<br>
			<label>
				Password:
				<input type="password" v-model="new_user.secret">
			</label>
			<br>
			<label>
				Is Admin?:
				<input type="checkbox" v-model="new_user.superuser">
			</label>
			<br>
			<button @click="save">Save</button>
		</Popup>
	</div>
</template>

<script lang="ts">
export default {
	data: () => ({
		table_data: {} as TableData,
		add_popup_open: false,
		new_user: {
			name: "",
			secret: "",
			superuser: false,
		} as User,
	}),

	async mounted() {
		await this.reload();
	},

	methods: {
		async reload() {
			const users = await $fetch("/api/v1/users") as User[];

			this.table_data = {
				multiSelect: false,
				disable_pagination: true,
				defaultSort: {
					column: 0,
					sort: "asc"
				},
				columns: [
					{name: "ID", type: "number", no_filter: true},
					{name: "Name", type: "string", no_filter: true},
					{name: "Type", type: "string", no_filter: true},
				],
				rows: users.map(x => ([
					x.id,
					x.name,
					x.superuser ? "Admin" : "User",
				]))
			};
		},

		async save() {
			try {
				await $fetch("/api/v1/users", {
					method: "POST",
					body: this.new_user,
				});
				this.add_popup_open = false;
				await this.reload();
			} catch(e: any) {
				console.error(e);
				window.alert(e?.data);
				return;
			}
		}
	}
}
</script>