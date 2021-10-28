import fs from "fs";
import pgClient from "pg";

export default {
	async getPostgresqlConnection(options: pgClient.PoolConfig) {
		if (!await existDatabase(options)) await createDatabase(options);
		const pool = new pgClient.Pool(options);
		return pool;
	}
}

async function existDatabase(options: pgClient.PoolConfig) {
	const client = await getPostgresDatabaseConnection(options);
	const res = (await client.query("SELECT datname FROM pg_catalog.pg_database WHERE lower(datname) = lower($1);", [options.database])).rowCount === 1 ? true : false;
	await client.end();
	return res;
}

function createDatabase(options: pgClient.PoolConfig) {
	return new Promise(async (resolve, reject) => {
		const client = await getPostgresDatabaseConnection(options);
		await client.query(`CREATE DATABASE ${options.database.toLowerCase()} WITH OWNER = postgres ENCODING = 'UTF8' CONNECTION LIMIT = -1;`);
		await client.end();

		const schemaPath = process.env.NODE_ENV === "testing" ? "./../src/sql/schema.sql" : "./../src/sql/schema.sql"
		fs.readFile(schemaPath, async (err, data) => {
			if(err) reject(new Error("Couldnt read schema.sql: " + err.message));
			const clientOfNewDatabase = new pgClient.Client(options);
			await clientOfNewDatabase.connect();
			await clientOfNewDatabase.query(data.toString());
			await clientOfNewDatabase.end();
			resolve(null);
		});

	});
}

async function getPostgresDatabaseConnection(options: pgClient.PoolConfig) {
	const client = new pgClient.Client({
		host: options.host,
		port: options.port,
		user: options.user,
		password: options.password,
		database: "postgres",
	});
	await client.connect();
	return client;
}