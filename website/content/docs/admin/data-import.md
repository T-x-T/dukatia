---
title: 'Data import'
description: 'How to import data into Dukatia'
---

## The officially supported way

You have the option to import transaction data directly from a csv file within the web frontend.  
The csv file needs to follow this format:  
`account_id,recipient_id,timestamp,comment,major_amount,minor_amount`

One line of your csv file might look something like this:
`ccac84c9-7f6f-4e01-86b7-24fd1559aedf,51ed755a-1c60-4ca2-ba04-668192f6683d,2024-07-06T18:42:00Z,This is a comment,-123,45`

The account and recipient IDs need to already exist, so you would have to create these first manually.


## Database manipulation

For more complicated data import or export tasks you will need to manipulate the database directly.  
Just keep in mind to take backups in case you break anything and have fun!