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
