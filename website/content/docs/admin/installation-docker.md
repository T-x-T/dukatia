---
title: 'Installation using Docker'
description: 'How to install dukatia using Docker'
---

1. Download the [default docker-compose.yml file from Github](https://github.com/T-x-T/dukatia/blob/main/docker-compose.yml)
2. Make changes to the environment variables based on the [configuration guide](configuration)
3. Configure the `postgres` container to use persistant storage or leave it out completely if you already a postgresql database running somewhere else
4. Run `docker compose up -d` or `docker-compose up -d` depening on your docker version. You can leave out the `-d` option to get a direct view of the logs the containers produce, which is nice for debugging your configuration
5. Open the frontend in your browser at http://{your server address}:{NITRO_PORT}
6. Log in with the `ADMIN_USERNAME` and `ADMIN_PASSWORD`
7. Optionally create a new user account without admin privileges for normal use