---
title: 'Configuration'
description: 'How to configure Dukatia'
---

Configuration is handled through environment variables. If you download the [default docker-compose.yml file from Github](https://github.com/T-x-T/dukatia/blob/main/docker-compose.yml) you don't have to worry about most of these. You should still read through the rest of this article and set the environment variables that make the most sense for your deployment.

## Backend

- `POSTGRES_USER`: the username used to log into the postgresql database. Should have permissions to create a new schema, if you don't want to create one by hand; default value: `postgres`
- `POSTGRES_PASSWORD`: the password of the `POSTGRES_USER`; default value: `password`
- `POSTGRES_HOST`: the hostname or IP address of your postgresql server; ; default value: `127.0.0.1`
- `POSTGRES_PORT`: the port your postgresql database server runs on; default value: `5432`
- `DATABASE_NAME`: the name of the schema to use. gets created automatically if it doesnt exist; default value: `txts_treasury_staging`
- `ADMIN_USERNAME`: the username of the admin account that gets created in a fresh database; default value: `admin`
- `ADMIN_PASSWORD`: the initial password of the admin account. Can be safely deleted after the admin account got created; default value: `password`
- `PEPPER`: Some random value used to make the hashing of passwords stronger, you should store this in your password manager to make your life easier if you lose the config; default value: `supersecret`
- `PORT`: the port the backend service should listen on; default value: `4000`
- `SESSION_EXPIRY_DAYS`: how many days an access token should be valid for. after the time is up users need to log in again; default value: `30`

## Frontend

- `NITRO_PORT`: the port the frontend http server should listen on
- `API_HOST`: information on how to reach the backend server from the frontend server. should be something like `http://backend:4000`