---
title: 'assets'
description: 'Assets related API endpoints'
---

## Purpose

The assets route contains all endpoints related to the management of assets.


### get all

This endpoint is used to retrieve all assets. 

Request:  
`GET /api/v1/assets/all`

Response:
```json
[
	{
		"id": "43e14483-6c56-4e56-a6f9-17917ac5f4c4",
		"user_id": "6b875d6c-7d4e-4ae3-931f-270604df1d7f",
		"name": "SuperStock inc.",
		"description": "A description describing this asset",
		"currency_id": "ffc53ce5-0bd2-4464-94b3-9866e506fd91",
		"value_per_unit": {
			"major": 10,
			"minor": 50,
			"minor_in_major": 100,
			"symbol": "€",
			"is_negative": false
		},
		"amount": 1.2345,
		"tag_ids": ["baf5ba00-440c-4d8a-a614-be2cbf4c2beb", "61d6ffc7-9d41-4d4d-ac0b-1c4fb92606eb"],
		"total_cost_of_ownership": {
			"total": {
				"major": 10,
				"minor": 50,
				"minor_in_major": 100,
				"symbol": "€",
				"is_negative": false
			},
			"monthly": {
				"major": 10,
				"minor": 50,
				"minor_in_major": 100,
				"symbol": "€",
				"is_negative": false
			},
			"yearly": {
				"major": 10,
				"minor": 50,
				"minor_in_major": 100,
				"symbol": "€",
				"is_negative": false
			}
		}
	}
]
```

| Property 								| Description 																											| Type				| Filterable 	| Sortable 		|
| ----------------------- | -----------------------------------------------------------------	| ----------- | ----------- | ----------- |
| id											| the id of the asset																								| UUIDv4			| Yes					| Yes					|
| user_id									| the id of the user owning this asset															| UUIDv4			|	No					| No					|
| name										| the name of the asset																							| string			| Yes					| Yes					|
| description							| the description of the asset																			| string?			| Yes					| Yes					|
| currency_id							| the id of the currency this asset is using												| UUIDv4			|	No					| No					|
| value_per_unit					| the value of a single unit of this asset													| Money?			| Yes					| Yes					|
| amount									| the total amount of items belonging to this asset									| number?			| Yes					| Yes					|		
| tag_ids									| the ids of tags this transaction belongs to												| UUIDv4\[]		|	Yes					| No					|
| total_cost_of_ownership	|	the total cost of ownership on a total, monthly and yearly basis	|	Object?			| No					| No					|


### get by id

This endpoint is used to retrieve a single asset by its id. 
You need to specify the id of the asset you want to retrieve in the request path.

Request:  
`GET /api/v1/assets/{assets_id}`

Response:
```json
{
	"id": "43e14483-6c56-4e56-a6f9-17917ac5f4c4",
	"user_id": "6b875d6c-7d4e-4ae3-931f-270604df1d7f",
	"name": "SuperStock inc.",
	"description": "A description describing this asset",
	"currency_id": "ffc53ce5-0bd2-4464-94b3-9866e506fd91",
	"value_per_unit": {
		"major": 10,
		"minor": 50,
		"minor_in_major": 100,
		"symbol": "€",
		"is_negative": false
	},
	"amount": 1.2345,
	"tag_ids": ["baf5ba00-440c-4d8a-a614-be2cbf4c2beb", "61d6ffc7-9d41-4d4d-ac0b-1c4fb92606eb"],
	"total_cost_of_ownership": {
		"total": {
			"major": 10,
			"minor": 50,
			"minor_in_major": 100,
			"symbol": "€",
			"is_negative": false
		},
		"monthly": {
			"major": 10,
			"minor": 50,
			"minor_in_major": 100,
			"symbol": "€",
			"is_negative": false
		},
		"yearly": {
			"major": 10,
			"minor": 50,
			"minor_in_major": 100,
			"symbol": "€",
			"is_negative": false
		}
	}
}
```

| Property 								| Description 																											| Type				|
| ----------------------- | -----------------------------------------------------------------	| ----------- |
| id											| the id of the asset																								| UUIDv4			|
| user_id									| the id of the user owning this asset															| UUIDv4			|
| name										| the name of the asset																							| string			|
| description							| the description of the asset																			| string?			|
| currency_id							| the id of the currency this asset is using												| UUIDv4			|
| value_per_unit					| the value of a single unit of this asset													| Money?			|
| amount									| the total amount of items belonging to this asset									| number?			|		
| tag_ids									| the ids of tags this transaction belongs to												| UUIDv4\[]		|
| total_cost_of_ownership	|	the total cost of ownership on a total, monthly and yearly basis	|	Object?			|


### create asset

This endpoint is used to create new assets. 

Request:  
`POST /api/v1/assets`
```json
{
	"name": "SuperStock inc.",
	"description": "A description describing this asset",
	"currency_id": "ffc53ce5-0bd2-4464-94b3-9866e506fd91",
	"tag_ids": ["baf5ba00-440c-4d8a-a614-be2cbf4c2beb", "61d6ffc7-9d41-4d4d-ac0b-1c4fb92606eb"],
}
```

| Property 								| Description 																											| Type				|
| ----------------------- | -----------------------------------------------------------------	| ----------- |
| name										| the name of the asset																							| string			|
| description							| the description of the asset																			| string?			|
| currency_id							| the id of the currency this asset is using												| UUIDv4			|	
| tag_ids									| the ids of tags this transaction belongs to												| UUIDv4\[]		|

Response:
```json
{
	"id": "43e14483-6c56-4e56-a6f9-17917ac5f4c4"
}
```


### modify asset

This endpoint is used to modify existing assets. 
You need to specify the id of the asset you want to modify in the request path.

Request:  
`POST /api/v1/assets/{asset_id}`
```json
{
	"name": "SuperStock inc.",
	"description": "A description describing this asset",
	"currency_id": "ffc53ce5-0bd2-4464-94b3-9866e506fd91",
	"tag_ids": ["baf5ba00-440c-4d8a-a614-be2cbf4c2beb", "61d6ffc7-9d41-4d4d-ac0b-1c4fb92606eb"],
}
```

| Property 								| Description 																											| Type				|
| ----------------------- | -----------------------------------------------------------------	| ----------- |
| name										| the name of the asset																							| string			|
| description							| the description of the asset																			| string?			|
| currency_id							| the id of the currency this asset is using												| UUIDv4			|	
| tag_ids									| the ids of tags this transaction belongs to												| UUIDv4\[]		|


### delete asset

This endpoint is used to delete an existing asset.
You need to specify the id of the asset you want to delete in the request path. 

Request:  
`DELETE /api/v1/assets/{asset_id}`


### get valuation history

This endpoint is used to retrieve the valuation history of a given asset.
You need to specify the id of the asset you want to get the valuation history of in the request path.

Request:  
`GET /api/v1/assets/{asset_id}/valuation_history`

Response:
```json
[
	{
		"value_per_unit": {
			"major": 10,
			"minor": 50,
			"minor_in_major": 100,
			"symbol": "€",
			"is_negative": false
		},
		"amount": 1.2345,
		"timestamp": "2023-01-01T10:10:00Z",
		"asset_id": "43e14483-6c56-4e56-a6f9-17917ac5f4c4"
	}
]
```

| Property 								| Description 																											| Type				|
| ----------------------- | -----------------------------------------------------------------	| ----------- |
| value_per_unit					| the value of a single unit of this asset													| Money				|
| amount									| the total amount of items belonging to this asset									| number			|		
| timestamp								| the time of the valuation																					| timestamp		|
| asset_id								| the id of the asset this valuation belongs to											| UUIDv4			|


### replace valuation history

This endpoint is used to replace the entire valuation history of an asset. Used to fix errors in past valuations.
You need to specify the id of the asset you want to replace the valuation history of in the request path.

Request:  
`POST /api/v1/assets/{asset_id}/valuation_history`

```json
[
	{
		"value_per_unit": {
			"major": 10,
			"minor": 50,
			"minor_in_major": 100,
			"symbol": "€",
			"is_negative": false
		},
		"amount": 1.2345,
		"amount_change": null,
		"timestamp": "2023-01-01T10:10:00Z",
		"cost": {
			"major": 1,
			"minor": 0,
			"minor_in_major": 100,
			"symbol": "€",
			"is_negative": false
		},
		"total_value": {
			"major": 9,
			"minor": 50,
			"minor_in_major": 100,
			"symbol": "€",
			"is_negative": false
		},
		"account_id": "525c32a8-c200-4c5d-a3c7-ef8936cc1c84"
	}
]
```

| Property 								| Description 																																												| Type				|
| ----------------------- | --------------------------------------------------------------------------------------------------- | ----------- |
| value_per_unit					| the value of a single unit of this asset																														| Money				|
| amount									| the total amount of items belonging to this asset																										| number?			|		
| amount_change						| difference to the previous value																																		| number?			|
| timestamp								| the time of the valuation																																						| timestamp		|
| cost										| additional costs incurred, doesnt go into valuation calculation, but gets added to the transaction	| Money?			|
| total_value							| overwrite the amount * value_per_unit calculation result																						| Money?			|
| account_id							| the id of the account to use for the transaction that gets created for this valuation								| UUIDv4			|


### post valuation

This endpoint is used to add a new valuation for a given asset.
You need to specify the id of the asset you want to add the valuation to in the request path.

Request:  
`POST /api/v1/assets/{asset_id}/valuations`

```json
{
	"value_per_unit": {
		"major": 10,
		"minor": 50,
		"minor_in_major": 100,
		"symbol": "€",
		"is_negative": false
	},
	"amount": 1.2345,
	"amount_change": null,
	"timestamp": "2023-01-01T10:10:00Z",
	"cost": {
		"major": 1,
		"minor": 0,
		"minor_in_major": 100,
		"symbol": "€",
		"is_negative": false
	},
	"total_value": {
		"major": 9,
		"minor": 50,
		"minor_in_major": 100,
		"symbol": "€",
		"is_negative": false
	},
	"account_id": "525c32a8-c200-4c5d-a3c7-ef8936cc1c84"
}
```

| Property 								| Description 																																												| Type				|
| ----------------------- | --------------------------------------------------------------------------------------------------- | ----------- |
| value_per_unit					| the value of a single unit of this asset																														| Money				|
| amount									| the total amount of items belonging to this asset																										| number?			|		
| amount_change						| difference to the previous value																																		| number?			|
| timestamp								| the time of the valuation																																						| timestamp		|
| cost										| additional costs incurred, doesnt go into valuation calculation, but gets added to the transaction	| Money?			|
| total_value							| overwrite the amount * value_per_unit calculation result																						| Money?			|
| account_id							| the id of the account to use for the transaction that gets created for this valuation								| UUIDv4			|