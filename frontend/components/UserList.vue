<template>
	<div>
		<h3>Users</h3>
		<button @click="add">Add</button>
		<CustomTable 
			v-if="Object.keys(table_data).length > 0"
			:tableDataProp="table_data"
			@rowClick="row_click"
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
		<Popup v-if="edit_popup_open" @close="edit_popup_open = false">
			<label>
				Name:
				<input type="text" v-model="new_user.name" disabled>
			</label>
			
			<br>
			<label>
				<br>
				Reset Password:
				<input type="password" v-model="new_user.secret">
			</label>
			<br>
			<button @click="update">Save password</button>
			
			<br>
			<br>
			<label>
				Is Admin?:
				<input type="checkbox" v-model="new_user.superuser">
			</label>
			<br>
			<button @click="update">Save admin</button>
			<br>
		</Popup>
	</div>
</template>

<script lang="ts">
export default {
	data: () => ({
		table_data: {} as TableData,
		add_popup_open: false,
		edit_popup_open: false,
		new_user: {
			name: "",
			secret: "",
			superuser: false,
		} as User,
	}),

	async mounted() {
		this.me = await $fetch("/api/v1/users/me") as User;
		await this.reload();
	},

	methods: {
		async reload() {
			const users = await $fetch("/api/v1/users/all") as User[];

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
		},

		async update() {
			try {
				await $fetch(`/api/v1/users/${this.new_user.id}`, {
					method: "PUT",
					body: this.new_user,
				});
				this.edit_popup_open = false;
				await this.reload();
			} catch(e: any) {
				console.error(e);
				window.alert(e?.data);
				return;
			}
		},

		row_click(row: any) {
			this.new_user = {
				id: row[0],
				name: row[1],
				secret: "",
				superuser: row[2] === "Admin",
			} as User;

			this.edit_popup_open = true;
		},
		
		add() {
			this.new_user = {
				name: "",
				secret: "",
				superuser: false,
			} as User;

			this.add_popup_open = true;
		},
	}
}
</script>