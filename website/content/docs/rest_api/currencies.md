---
title: 'currencies'
description: 'Currency related API endpoints'
---

## Purpose

The users route contains all endpoints related to currency management.

### get all

This endpoint is used to retrieve all currencies. 

Request:  
`GET /api/v1/currencies/all`

Response:
```json
[
	{
		"id": 0,
		"name": "euro",
		"minor_in_mayor": 100,
		"symbol": "€"
	}
]
```

| Property | Description |
| ----------- | ----------- |
| id | id of the currency |
| name | name of the currency |
| minor_in_mayor | for example there are 100 cents in 1 Euro -> minor_in_mayor = 100 |
| symbol | symbol of the currency |

### get by id

This endpoint is used to retrieve a single currency by its id. 
You need to specify the id of the currency you want to retrieve in the request path.

Request:  
`GET /api/v1/currencies/{currency_id}`

Response:
```json
{
	"id": 0,
	"name": "euro",
	"minor_in_mayor": 100,
	"symbol": "€"
}
```

| Property | Description |
| ----------- | ----------- |
| id | id of the currency |
| name | name of the currency |
| minor_in_mayor | for example there are 100 cents in 1 Euro -> minor_in_mayor = 100 |
| symbol | symbol of the currency |

### create currency

This endpoint is used to create new currencies. 

Request:  
`POST /api/v1/currency`
```json
{
		"name": "euro",
		"minor_in_mayor": 100,
		"symbol": "€"
}
```

| Property | Description |
| ----------- | ----------- |
| name | name of the currency |
| minor_in_mayor | for example there are 100 cents in 1 Euro -> minor_in_mayor = 100 |
| symbol | symbol of the currency |

### modify currency

This endpoint is used to modify existing currencies.  
You need to specify the id of the currency you want to modify in the request path.

Request:  
`PUT /api/v1/currency/{currency_id}`
```json
{
		"name": "euro",
		"minor_in_mayor": 100,
		"symbol": "€"
}
```

| Property | Description |
| ----------- | ----------- |
| name | name of the currency |
| minor_in_mayor | for example there are 100 cents in 1 Euro -> minor_in_mayor = 100 |
| symbol | symbol of the currency |

