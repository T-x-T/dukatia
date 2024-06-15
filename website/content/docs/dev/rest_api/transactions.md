---
title: 'transactions'
description: 'Transactions related API endpoints'
---

## Purpose

The transactions route contains all endpoints related to the management of transactions.


### get all

This endpoint is used to retrieve all transactions. 

Request:  
`GET /api/v1/transactions/all`

Response:
```json
[
	{
		"id": "b0e4eb0c-3719-4da8-bedc-27ca238b6ebc",
		"user_id": "6b875d6c-7d4e-4ae3-931f-270604df1d7f",
		"currency_id": "ffc53ce5-0bd2-4464-94b3-9866e506fd91",
		"account_id": "525c32a8-c200-4c5d-a3c7-ef8936cc1c84",
		"recipient_id": "e3001fbc-1e28-44ad-94f6-9900be5dbc52",
		"status": 1,
		"timestamp": "2023-01-01T10:10:00Z",
		"total_amount": {
			"major": 10,
			"minor": 50,
			"minor_in_major": 100,
			"symbol": "€",
			"is_negative": false
		},
		"comment": "hello world!",
		"tag_ids": ["baf5ba00-440c-4d8a-a614-be2cbf4c2beb", "61d6ffc7-9d41-4d4d-ac0b-1c4fb92606eb"],
		"asset": null,
		"positions": [
			{
				"id": "0a02167e-9a47-41ad-90b1-936b17e3a6f1",
				"amount": {
					"major": 10,
					"minor": 50,
					"minor_in_major": 100,
					"symbol": "€",
					"is_negative": false
				},
				"comment": "Im the only position!",
				"tag_id": "baf5ba00-440c-4d8a-a614-be2cbf4c2beb"
			}
		]
	}
]
```

| Property 					| Description 																							| Type				| Filterable 	| Sortable 		|
| ----------------- | --------------------------------------------------------- | ----------- | ----------- | ----------- |
| id								| the id of the transaction																	| UUIDv4			| Yes					| Yes					|
| user_id						| the id of the user owning this transaction								| UUIDv4			|	No					| Yes					|
| currency_id				| the id of the currency this transaction is using					| UUIDv4?			|	Yes					| Yes					|
| account_id				| the id of the account this transaction belongs to					| UUIDv4			|	Yes					| Yes					|
| recipient_id			| the id of the recipient this transaction belongs to				| UUIDv4			|	Yes					| Yes					|
| status						| the status of the transaction, will always be 1=completed	| number			|	No					| Yes					|
| timestamp					| the time when this transaction has taken place						| timestamp		|	Yes					| Yes					|
| total_amount			| the total amount of money of all positions combined				| Money?			|	Yes					| Yes					|
| comment						| an optional comment describing this transaction						| string?			|	Yes					| Yes					|
| tag_ids						| the ids of tags this transaction belongs to								| UUIDv4\[]		|	Yes					| No					|
| asset							| the asset this transactions belongs to										| Asset?			|	Yes					| No					|
| positions					| the positions this transaction consists of								| Position\[]	|	No					| No					|


### get by id

This endpoint is used to retrieve a single transaction by its id. 
You need to specify the id of the transaction you want to retrieve in the request path.

Request:  
`GET /api/v1/transactions/{transaction_id}`

Response:
```json
{
	"id": "b0e4eb0c-3719-4da8-bedc-27ca238b6ebc",
	"user_id": "6b875d6c-7d4e-4ae3-931f-270604df1d7f",
	"currency_id": "ffc53ce5-0bd2-4464-94b3-9866e506fd91",
	"account_id": "525c32a8-c200-4c5d-a3c7-ef8936cc1c84",
	"recipient_id": "e3001fbc-1e28-44ad-94f6-9900be5dbc52",
	"status": 1,
	"timestamp": "2023-01-01T10:10:00Z",
	"total_amount": {
		"major": 10,
		"minor": 50,
		"minor_in_major": 100,
		"symbol": "€",
		"is_negative": false
	},
	"comment": "hello world!",
	"tag_ids": ["baf5ba00-440c-4d8a-a614-be2cbf4c2beb", "61d6ffc7-9d41-4d4d-ac0b-1c4fb92606eb"],
	"asset": null,
	"positions": [
		{
			"id": "0a02167e-9a47-41ad-90b1-936b17e3a6f1",
			"amount": {
				"major": 10,
				"minor": 50,
				"minor_in_major": 100,
				"symbol": "€",
				"is_negative": false
			},
			"comment": "Im the only position!",
			"tag_id": "baf5ba00-440c-4d8a-a614-be2cbf4c2beb"
		}
	]
}
```

| Property 					| Description 																							| Type				|
| ----------------- | --------------------------------------------------------- | ----------- |
| id								| the id of the transaction																	| UUIDv4			|
| user_id						| the id of the user owning this transaction								| UUIDv4			|
| currency_id				| the id of the currency this transaction is using					| UUIDv4?			|
| account_id				| the id of the account this transaction belongs to					| UUIDv4			|
| recipient_id			| the id of the recipient this transaction belongs to				| UUIDv4			|
| status						| the status of the transaction, will always be 1=completed	| number			|
| timestamp					| the time when this transaction has taken place						| timestamp		|
| total_amount			| the total amount of money of all positions combined				| Money?			|
| comment						| an optional comment describing this transaction						| string?			|
| tag_ids						| the ids of tags this transaction belongs to								| UUIDv4\[]		|
| asset							| the asset this transactions belongs to										| Asset?			|
| positions					| the positions this transaction consists of								| Position\[]	|


### get summary

This endpoint is used to retrieve a summary of transactions.  
You probably need to apply a filter. If you apply the same filter as with the get all endpoint you can get a summary disregarding pagination.

Request:  
`GET /api/v1/transactions/summary`

Response:
```json
{
	"count": 123,
	"total_amount": "123.45€"
}
```

| Property 					| Description 																							| Type				|
| ----------------- | --------------------------------------------------------- | ----------- |
| count							| the number of transactions in filter											| number			|
| total_amount			| the total monetary amount of transactions in filter				| string			|


### create transaction

This endpoint is used to create new transactions. 

Request:  
`POST /api/v1/transactions`
```json
{
	"account_id": "525c32a8-c200-4c5d-a3c7-ef8936cc1c84",
	"recipient_id": "e3001fbc-1e28-44ad-94f6-9900be5dbc52",
	"status": 1,
	"timestamp": "2023-01-01T10:10:00Z",
	"comment": "hello world!",
	"tag_ids": ["baf5ba00-440c-4d8a-a614-be2cbf4c2beb", "61d6ffc7-9d41-4d4d-ac0b-1c4fb92606eb"],
	"asset_id": "43e14483-6c56-4e56-a6f9-17917ac5f4c4",
	"positions": [
		{
			"amount": {
				"major": 10,
				"minor": 50,
				"minor_in_major": 100,
				"symbol": "€",
				"is_negative": false
			},
			"comment": "Im the only position!",
			"tag_id": "baf5ba00-440c-4d8a-a614-be2cbf4c2beb"
		}
	]
}
```

| Property 					| Description 																								| Type				|
| ----------------- | -----------------------------------------------------------	| ----------- |
| account_id				| the id of the account this transaction belongs to						| UUIDv4			|
| recipient_id			| the id of the recipient this transaction belongs to					| UUIDv4			|
| status						| the status of the transaction, should always be 1=completed	| number			|
| timestamp					| the time when this transaction has taken place							| timestamp		|
| comment						| an optional comment describing this transaction							| string?			|
| tag_ids						| the ids of tags this transaction belongs to									| UUIDv4\[]		|
| asset_id					| the id of the asset this transactions belongs to				 		| Asset?			|
| positions					| the positions this transaction consists of									| Position\[]	|

Response:
```json
{
	"id": "b0e4eb0c-3719-4da8-bedc-27ca238b6ebc"
}
```


### modify transaction

This endpoint is used to modify existing transactions. 
You need to specify the id of the transaction you want to modify in the request path.

Request:  
`PUT /api/v1/transactions/{transaction_id}`
```json
{
	"account_id": "525c32a8-c200-4c5d-a3c7-ef8936cc1c84",
	"recipient_id": "e3001fbc-1e28-44ad-94f6-9900be5dbc52",
	"status": 1,
	"timestamp": "2023-01-01T10:10:00Z",
	"comment": "hello world!",
	"tag_ids": ["baf5ba00-440c-4d8a-a614-be2cbf4c2beb", "61d6ffc7-9d41-4d4d-ac0b-1c4fb92606eb"],
	"asset_id": "43e14483-6c56-4e56-a6f9-17917ac5f4c4",
	"positions": [
		{
			"amount": {
				"major": 10,
				"minor": 50,
				"minor_in_major": 100,
				"symbol": "€",
				"is_negative": false
			},
			"comment": "Im the only position!",
			"tag_id": "baf5ba00-440c-4d8a-a614-be2cbf4c2beb"
		}
	]
}
```

| Property 					| Description 																								| Type				|
| ----------------- | -----------------------------------------------------------	| ----------- |
| account_id				| the id of the account this transaction belongs to						| UUIDv4			|
| recipient_id			| the id of the recipient this transaction belongs to					| UUIDv4			|
| status						| the status of the transaction, should always be 1=completed	| number			|
| timestamp					| the time when this transaction has taken place							| timestamp		|
| comment						| an optional comment describing this transaction							| string?			|
| tag_ids						| the ids of tags this transaction belongs to									| UUIDv4\[]		|
| asset_id					| the id of the asset this transactions belongs to				 		| Asset?			|
| positions					| the positions this transaction consists of									| Position\[]	|


### delete transaction

This endpoint is used to delete an existing transaction.
You need to specify the id of the transaction you want to delete in the request path. 

Request:  
`DELETE /api/v1/transactions/{transaction_id}`