---
title: 'accounts'
description: 'Account related API endpoints'
---

## Purpose

The accounts route contains all endpoints related to account management.  
Do not confuse this route with the [users route](./user)

### get all

This endpoint is used to retrieve all accounts. 

Request:  
`GET /api/v1/accounts/all`

Response:
```json
[
	{
		"id": 42,
		"name": "bank xy",
		"default_currency_id": 0,
		"user_id": 0,
		"tag_ids": [0, 2]
	}
]
```

| Property | Description |
| ----------- | ----------- |
| id | id of the account |
| name | name of the account |
| default_currency_id | currency id of all newly added transactions using this account |
| user_id | user_id that owns this account |
| tag_ids | all tags assigned to this account |

### get by id

This endpoint is used to retrieve a single account by its id. 
You need to specify the id of the account you want to retrieve in the request path.

Request:  
`GET /api/v1/accounts/{account_id}`

Response:
```json
{
	"id": 42,
	"name": "bank xy",
	"default_currency_id": 0,
	"user_id": 0,
	"tag_ids": [0, 2]
}
```

| Property | Description |
| ----------- | ----------- |
| id | id of the account |
| name | name of the account |
| default_currency_id | currency id of all newly added transactions using this account |
| user_id | user_id that owns this account |
| tag_ids | all tags assigned to this account |

### create account

This endpoint is used to create new accounts. 

Request:  
`POST /api/v1/accounts`
```json
{
	"name": "bank xy",
	"default_currency_id": 0,
	"tag_ids": [0, 2]
}
```

| Property | Description |
| ----------- | ----------- |
| name | name of the account |
| default_currency_id | currency id of all newly added transactions using this account |
| tag_ids | all tags assigned to this account |

### modify account

This endpoint is used to modify existing accounts.  
You need to specify the id of the account you want to modify in the request path.

Request:  
`PUT /api/v1/accounts/{account_id}`
```json
{
	"name": "bank xy",
	"default_currency_id": 0,
	"tag_ids": [0, 2]
}
```

| Property | Description |
| ----------- | ----------- |
| name | name of the account |
| default_currency_id | currency id of all newly added transactions using this account |
| tag_ids | all tags assigned to this account |
