---
title: 'Backup'
description: 'How to backup and restore your Dukatia database'
---

Dukatia exclusively saves data in the postgresql database specified in the [configuration](configuration).  
You can use the native tools of postgresql to take and restore backups.  
Taking or restoring backups while Dukatia is running generally shouldn't lead to issues. For a restore I personally would stop the Dukatia backend service though.  
If you want to be able to log in you also should save the `pepper` value from your configuration as that is used to make the hashing of passwords stored in the database more secure.