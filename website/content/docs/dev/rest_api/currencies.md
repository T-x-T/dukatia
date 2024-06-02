---
title: 'currencies'
description: 'Currency related API endpoints'
---

## Purpose

The currencies route contains all endpoints related to the management of currencies.
For more information on how monetary values are handled check out [this post](../currency)


### get all

This endpoint is used to retrieve all currencies. 

Request:  
`GET /api/v1/currencies/all`

Response:
```json
[
	{
		"id": "ffc53ce5-0bd2-4464-94b3-9866e506fd91",
		"name": "euro",
		"minor_in_major": 100,
		"symbol": "€"
	}
]
```

| Property 			 | Description 																											 | Type 			| Filterable 	| Sortable 		|
| -------------- | ----------------------------------------------------------------- | ---------- | ----------- | ----------- |
| id 						 | id of the currency 																							 | UUIDv4			| Yes					| No					|
| name 					 | name of the currency 																						 | string			| Yes					| No					|
| minor_in_major | for example there are 100 cents in 1 Euro -> minor_in_major = 100 | number			| Yes					| No					|
| symbol 				 | symbol of the currency 																					 | string			| Yes					| No					|


### get by id

This endpoint is used to retrieve a single currency by its id. 
You need to specify the id of the currency you want to retrieve in the request path.

Request:  
`GET /api/v1/currencies/{currency_id}`

Response:
```json
{
	"id": "ffc53ce5-0bd2-4464-94b3-9866e506fd91",
	"name": "euro",
	"minor_in_major": 100,
	"symbol": "€"
}
```

| Property 			 | Description 																											 | Type 			|
| -------------- | ----------------------------------------------------------------- | ---------- |
| id 						 | id of the currency 																							 | UUIDv4			|
| name 					 | name of the currency 																						 | string			|
| minor_in_major | for example there are 100 cents in 1 Euro -> minor_in_major = 100 | number			|
| symbol 				 | symbol of the currency 																					 | string			|


### create currency

This endpoint is used to create new currencies.  
Only superusers are allowed to use this endpoint. Regular users will receive a response with the status code 400.

Request:  
`POST /api/v1/currency`
```json
{
		"name": "euro",
		"minor_in_major": 100,
		"symbol": "€"
}
```

| Property 			 | Description 																											 | Type 			|
| -------------- | ----------------------------------------------------------------- | ---------- |
| name 					 | name of the currency 																						 | string			|
| minor_in_major | for example there are 100 cents in 1 Euro -> minor_in_major = 100 | number			|
| symbol 				 | symbol of the currency 																					 | string			|


### modify currency

This endpoint is used to modify existing currencies.  
You need to specify the id of the currency you want to modify in the request path.  
Only superusers are allowed to use this endpoint. Regular users will receive a response with the status code 400.  

Request:  
`PUT /api/v1/currency/{currency_id}`
```json
{
		"name": "euro",
		"minor_in_major": 100,
		"symbol": "€"
}
```

| Property 			 | Description 																											 | Type 			|
| -------------- | ----------------------------------------------------------------- | ---------- |
| name 					 | name of the currency 																						 | string			|
| minor_in_major | for example there are 100 cents in 1 Euro -> minor_in_major = 100 | number			|
| symbol 				 | symbol of the currency 																					 | string			|

