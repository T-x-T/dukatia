---
title: 'budgets'
description: 'Budget related API endpoints'
---

## Purpose

The budgets route contains all endpoints related to the management of budgets.


### get all

This endpoint is used to retrieve all budgets.   

Request:  
`GET /api/v1/budgets/all`

Response:
```json
[
	{
		"id": "42d46365-1219-4a15-a536-4c2eca8e4bdf",
		"name": "technology",
		"user_id": "6b875d6c-7d4e-4ae3-931f-270604df1d7f",
		"amount": {
			"major": 123,
			"minor": 45,
			"minor_in_major": 100,
			"symbol": "€",
			"is_negative": false
		},
		"rollover": false,
		"period": 2,
		"filter_tag_ids": ["baf5ba00-440c-4d8a-a614-be2cbf4c2beb", "61d6ffc7-9d41-4d4d-ac0b-1c4fb92606eb"],
		"currency_id": "ffc53ce5-0bd2-4464-94b3-9866e506fd91",
		"active_from": "2023-01-01T10:10:00Z",
		"active_to": null,
		"used_amount": {
			"major": 10,
			"minor": 50,
			"minor_in_major": 100,
			"symbol": "€",
			"is_negative": false
		},
		"available_amount": {
			"major": 112,
			"minor": 95,
			"minor_in_major": 100,
			"symbol": "€",
			"is_negative": false
		},
		"utilization": 8.5054678007
	}
]
```

| Property 					| Description 																																			| Type				| Filterable 	| Sortable	|
| ----------------- | --------------------------------------------------------------------------------- | ----------- | -----------	| --------- |
| id								| the id of the budget																															| UUIDv4			|	Yes					| No				|
| name		 					| the name of the budget																														| string			| Yes					| No				|
| user_id						| the id of the user owning this budget																							| UUIDv4			| No					| No				|
| amount						| the total amount set for the budget																								| Money				| Yes					| No				|
| rollover					| if unused budget is rolled over into the next period															| boolean 		| Yes					| No				|	
| period						| how often the budget resets 0=daily, 1=weekly, 2=monthly, 3=quarterly, 4=yearly		| number			| No					| No				|	
| filter_tag_ids		| ids of the tags whose transactions get included in this budget										| UUIDv4\[]		| Yes					| No				|
| currency_id				| if of the currency used in this budget																						| UUIDv4			| Yes					| No				|
| active_from				| from which point in time onwards transactions should get included in this budget	| timestamp 	| Yes					| No				|
| active_to					| until which point in time transactions should get included in this budget					| timestamp?	| Yes					| No				|
| used_amount				| amount of money spent in current period																						| Money?			| No					| No				|
| available_amount	|	amount of money still available in current period																	| Money?			| No					| No				|
| utilization				| percentage of used money in current period																				| number?			| No					| No				|


### get by id

This endpoint is used to retrieve a single budget by its id. 
You need to specify the id of the budget you want to retrieve in the request path.

Request:  
`GET /api/v1/budgets/{budget_id}`

Response:
```json
{
	"id": "42d46365-1219-4a15-a536-4c2eca8e4bdf",
	"name": "technology",
	"user_id": "6b875d6c-7d4e-4ae3-931f-270604df1d7f",
	"amount": {
		"major": 123,
		"minor": 45,
		"minor_in_major": 100,
		"symbol": "€",
		"is_negative": false
	},
	"rollover": false,
	"period": 2,
	"filter_tag_ids": ["baf5ba00-440c-4d8a-a614-be2cbf4c2beb", "61d6ffc7-9d41-4d4d-ac0b-1c4fb92606eb"],
	"currency_id": "ffc53ce5-0bd2-4464-94b3-9866e506fd91",
	"active_from": "2023-01-01T10:10:00Z",
	"active_to": null,
	"used_amount": {
		"major": 10,
		"minor": 50,
		"minor_in_major": 100,
		"symbol": "€",
		"is_negative": false
	},
	"available_amount": {
		"major": 112,
		"minor": 95,
		"minor_in_major": 100,
		"symbol": "€",
		"is_negative": false
	},
	"utilization": 8.5054678007
}
```

| Property 					| Description 																																			| Type				|
| ----------------- | --------------------------------------------------------------------------------- | ----------- |
| id								| the id of the budget																															| UUIDv4			|
| name		 					| the name of the budget																														| string			|
| user_id						| the id of the user owning this budget																							| UUIDv4			|
| amount						| the total amount set for the budget																								| Money				|
| rollover					| if unused budget is rolled over into the next period															| boolean 		|	
| period						| how often the budget resets 0=daily, 1=weekly, 2=monthly, 3=quarterly, 4=yearly		| number			|	
| filter_tag_ids		| ids of the tags whose transactions get included in this budget										| UUIDv4\[]		|
| currency_id				| if of the currency used in this budget																						| UUIDv4			|
| active_from				| from which point in time onwards transactions should get included in this budget	| timestamp 	|
| active_to					| until which point in time transactions should get included in this budget					| timestamp?	|
| used_amount				| amount of money spent in current period																						| Money?			|
| available_amount	|	amount of money still available in current period																	| Money?			|
| utilization				| percentage of used money in current period																				| number?			|



### get transactions

This endpoint is used to retrieve all transactions matching this budget in its current period.  
You need to specify the id of the budget you want to retrieve transactions of in the request path.   

Request:  
`GET /api/v1/budgets/{budget_id}/transactions`

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


### create budget

This endpoint is used to create new budgets. 

Request:  
`POST /api/v1/budgets`
```json
{
	"name": "technology",
	"amount": {
		"major": 123,
		"minor": 45,
		"minor_in_major": 100,
		"symbol": "€",
		"is_negative": false
	},
	"rollover": false,
	"period": 2,
	"filter_tag_ids": ["baf5ba00-440c-4d8a-a614-be2cbf4c2beb", "61d6ffc7-9d41-4d4d-ac0b-1c4fb92606eb"],
	"currency_id": "ffc53ce5-0bd2-4464-94b3-9866e506fd91",
	"active_from": "2023-01-01T10:10:00Z",
	"active_to": null,
}
```

| Property 					| Description 																																			| Type				|
| ----------------- | --------------------------------------------------------------------------------- | ----------- |
| name		 					| the name of the budget																														| string			|
| amount						| the total amount set for the budget																								| Money				|
| rollover					| if unused budget is rolled over into the next period															| boolean 		|	
| period						| how often the budget resets 0=daily, 1=weekly, 2=monthly, 3=quarterly, 4=yearly		| number			|	
| filter_tag_ids		| ids of the tags whose transactions get included in this budget										| UUIDv4\[]		|
| currency_id				| if of the currency used in this budget																						| UUIDv4			|
| active_from				| from which point in time onwards transactions should get included in this budget	| timestamp 	|
| active_to					| until which point in time transactions should get included in this budget					| timestamp?	|


Response:
```json
{
	"id": "42d46365-1219-4a15-a536-4c2eca8e4bdf"
}
```

| Property 					| Description 												| Type				|
| ----------------- | ----------------------------------- | ----------- |
| id								| the id of the newly created budget	| UUIDv4			|


### modify budget

This endpoint is used to modify an existing budget.  
You need to specify the id of the budget you want to modify in the request path.

Request:  
`PUT /api/v1/budget/{budget_id}`
```json
{
	"name": "technology",
	"amount": {
		"major": 123,
		"minor": 45,
		"minor_in_major": 100,
		"symbol": "€",
		"is_negative": false
	},
	"rollover": false,
	"period": 2,
	"filter_tag_ids": ["baf5ba00-440c-4d8a-a614-be2cbf4c2beb", "61d6ffc7-9d41-4d4d-ac0b-1c4fb92606eb"],
	"currency_id": "ffc53ce5-0bd2-4464-94b3-9866e506fd91",
	"active_from": "2023-01-01T10:10:00Z",
	"active_to": null,
}
```

| Property 					| Description 																																			| Type				|
| ----------------- | --------------------------------------------------------------------------------- | ----------- |
| name		 					| the name of the budget																														| string			|
| amount						| the total amount set for the budget																								| Money				|
| rollover					| if unused budget is rolled over into the next period															| boolean 		|	
| period						| how often the budget resets 0=daily, 1=weekly, 2=monthly, 3=quarterly, 4=yearly		| number			|	
| filter_tag_ids		| ids of the tags whose transactions get included in this budget										| UUIDv4\[]		|
| currency_id				| if of the currency used in this budget																						| UUIDv4			|
| active_from				| from which point in time onwards transactions should get included in this budget	| timestamp 	|
| active_to					| until which point in time transactions should get included in this budget					| timestamp?	|


### delete budget

This endpoint is used to delete an existing budget.
You need to specify the id of the budget you want to delete in the request path. 

Request:  
`DELETE /api/v1/budgets/{budget_id}`