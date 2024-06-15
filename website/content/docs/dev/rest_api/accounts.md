---
title: 'accounts'
description: 'Account related API endpoints'
---

## Purpose

The accounts route contains all endpoints related to the management of accounts.  
Do not confuse this route with the [users route](./users) used to manage user accounts in Dukatia.


### get all

This endpoint is used to retrieve all accounts. 

Request:  
`GET /api/v1/accounts/all`

Response:
```json
[
	{
		"id": "525c32a8-c200-4c5d-a3c7-ef8936cc1c84",
		"name": "bank xy",
		"default_currency_id": "ffc53ce5-0bd2-4464-94b3-9866e506fd91",
		"user_id": "6b875d6c-7d4e-4ae3-931f-270604df1d7f",
		"tag_ids": ["baf5ba00-440c-4d8a-a614-be2cbf4c2beb", "61d6ffc7-9d41-4d4d-ac0b-1c4fb92606eb"],
		"balance": 12345
	}
]
```

| Property 						| Description 																										| Type 			| Filterable 	| Sortable 		|
| ------------------- | --------------------------------------------------------------- | --------- | ----------- | ----------- |
| id 									| id of the account 																							| UUIDv4 		| Yes					| No					|
| name 								| name of the account 																						| string 		| Yes					| No					|
| default_currency_id | currency id of all newly added transactions using this account	| UUIDv4 		| Yes					| No					|
| user_id 						| user_id that owns this account 																	| UUIDv4 		| No					| No					|
| tag_ids 						| all tags assigned to this account 															| UUIDv4\[] | Yes					| No					|
| balance							|	current balance of account in [cents](../currency)							| number?		| Yes					| No					|


### get by id

This endpoint is used to retrieve a single account by its id. 
You need to specify the id of the account you want to retrieve in the request path.

Request:  
`GET /api/v1/accounts/{account_id}`

Response:
```json
{
	"id": "525c32a8-c200-4c5d-a3c7-ef8936cc1c84",
	"name": "bank xy",
	"default_currency_id": "ffc53ce5-0bd2-4464-94b3-9866e506fd91",
	"user_id": "6b875d6c-7d4e-4ae3-931f-270604df1d7f",
	"tag_ids": ["baf5ba00-440c-4d8a-a614-be2cbf4c2beb", "61d6ffc7-9d41-4d4d-ac0b-1c4fb92606eb"],
	"balance": 12345
}
```

| Property 						| Description 																										| Type 			|
| ------------------- | --------------------------------------------------------------- | --------- |
| id 									| id of the account 																							| UUIDv4 		|
| name 								| name of the account 																						| string 		|
| default_currency_id | currency id of all newly added transactions using this account	| UUIDv4 		|
| user_id 						| user_id that owns this account 																	| UUIDv4 		|
| tag_ids 						| all tags assigned to this account 															| UUIDv4\[] |
| balance							|	current balance of account in [cents](../currency)							| number?		|


### create account

This endpoint is used to create new accounts. 

Request:  
`POST /api/v1/accounts`
```json
{
	"name": "bank xy",
	"default_currency_id": "ffc53ce5-0bd2-4464-94b3-9866e506fd91",
	"tag_ids": ["baf5ba00-440c-4d8a-a614-be2cbf4c2beb", "61d6ffc7-9d41-4d4d-ac0b-1c4fb92606eb"]
}
```

| Property 						| Description 																										| Type 			 |
| ------------------- | --------------------------------------------------------------- | ---------- |
| name 								| name of the account 																						| string 		 |
| default_currency_id | currency id of all newly added transactions using this account	| UUIDv4 		 |
| tag_ids 						| all tags assigned to this account 															| UUIDv4\[]? |

Response:
```json
{
	"id": "525c32a8-c200-4c5d-a3c7-ef8936cc1c84"
}
```


### modify account

This endpoint is used to modify existing accounts.  
You need to specify the id of the account you want to modify in the request path.

Request:  
`PUT /api/v1/accounts/{account_id}`
```json
{
	"name": "bank xy",
	"default_currency_id": "ffc53ce5-0bd2-4464-94b3-9866e506fd91",
	"tag_ids": ["baf5ba00-440c-4d8a-a614-be2cbf4c2beb", "61d6ffc7-9d41-4d4d-ac0b-1c4fb92606eb"]
}
```

| Property 						| Description 																										| Type 			 |
| ------------------- | --------------------------------------------------------------- | ---------- |
| name 								| name of the account 																						| string 		 |
| default_currency_id | currency id of all newly added transactions using this account	| UUIDv4 		 |
| tag_ids 						| all tags assigned to this account 															| UUIDv4\[]? |
