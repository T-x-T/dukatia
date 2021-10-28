import postgresql from "./postgresql/index.js";

export default async (options: any) => {
	const database = "postgresql";

	switch(database) {
		case "postgresql": {
			return await postgresql.getPostgresqlConnection(options);
		}
	}
}