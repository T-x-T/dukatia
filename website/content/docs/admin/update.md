---
title: 'Installing updates'
description: 'How to keep Dukatia up-to-date'
---

It is recommended to always [take a backup](backup) before updating to a new Dukatia version!  

Updating is as simple as pulling in the new docker images.
If you run on docker compose you can update with a simple command: `docker compose pull && docker compose down && docker compose up -d`. Should your docker version be a bit older you need to alter the command slightly: `docker-compose pull && docker-compose down && docker-compose up -d`.  