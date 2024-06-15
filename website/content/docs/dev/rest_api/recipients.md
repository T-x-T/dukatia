---
title: 'recipients'
description: 'Recipient related API endpoints'
---

## Purpose

The recipients route contains all endpoints related to the management of recipients.  


### get all

This endpoint is used to retrieve all recipients. 

Request:  
`GET /api/v1/recipients/all`

Response:
```json
[
	{
		"id": "e8d92360-51df-4827-abc9-a173a6913b06",
		"name": "berta's bakery",
		"user_id": "6b875d6c-7d4e-4ae3-931f-270604df1d7f",
		"tag_ids": ["baf5ba00-440c-4d8a-a614-be2cbf4c2beb", "61d6ffc7-9d41-4d4d-ac0b-1c4fb92606eb"]
	}
]
```

| Property 						| Description 												| Type 			| Filterable 	| Sortable 		|
| ------------------- | ----------------------------------- | --------- | ----------- | ----------- |
| id 									| id of the recipient									| UUIDv4 		| Yes					| No					|
| name 								| name of the recipient								| string 		| Yes					| No					|
| user_id 						| user_id that owns this recipient		| UUIDv4 		| No					| No					|
| tag_ids 						| all tags assigned to this recipient	| UUIDv4\[] | Yes					| No					|


### get by id

This endpoint is used to retrieve a single recipient by its id. 
You need to specify the id of the recipient you want to retrieve in the request path.

Request:  
`GET /api/v1/recipients/{recipient_id}`

Response:
```json
{
	"id": "e8d92360-51df-4827-abc9-a173a6913b06",
	"name": "berta's bakery",
	"user_id": "6b875d6c-7d4e-4ae3-931f-270604df1d7f",
	"tag_ids": ["baf5ba00-440c-4d8a-a614-be2cbf4c2beb", "61d6ffc7-9d41-4d4d-ac0b-1c4fb92606eb"]
}
```

| Property 						| Description 												| Type 			|
| ------------------- | ----------------------------------- | --------- |
| id 									| id of the recipient									| UUIDv4 		|
| name 								| name of the recipient								| string 		|
| user_id 						| user_id that owns this recipient		| UUIDv4 		|
| tag_ids 						| all tags assigned to this recipient	| UUIDv4\[] |


### create recipient

This endpoint is used to create new recipients. 

Request:  
`POST /api/v1/recipients`
```json
{
	"name": "berta's bakery",
	"tag_ids": ["baf5ba00-440c-4d8a-a614-be2cbf4c2beb", "61d6ffc7-9d41-4d4d-ac0b-1c4fb92606eb"]
}
```

| Property 						| Description 												| Type 			|
| ------------------- | ----------------------------------- | --------- |
| name 								| name of the recipient								| string 		|
| tag_ids 						| all tags assigned to this recipient	| UUIDv4\[] |

Response:
```json
{
	"id": "e8d92360-51df-4827-abc9-a173a6913b06"
}
```


### modify recipient

This endpoint is used to modify existing recipients.  
You need to specify the id of the recipient you want to modify in the request path.

Request:  
`PUT /api/v1/recipients`
```json
{
	"name": "berta's bakery",
	"tag_ids": ["baf5ba00-440c-4d8a-a614-be2cbf4c2beb", "61d6ffc7-9d41-4d4d-ac0b-1c4fb92606eb"]
}
```

| Property 						| Description 												| Type 			|
| ------------------- | ----------------------------------- | --------- |
| name 								| name of the recipient								| string 		|
| tag_ids 						| all tags assigned to this recipient	| UUIDv4\[] |
```