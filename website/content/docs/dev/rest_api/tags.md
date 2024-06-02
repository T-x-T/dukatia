---
title: 'tags'
description: 'Tag related API endpoints'
---

## Purpose

The tag route contains all endpoints related to tag management.


### get all

This endpoint is used to retrieve all tags. 

Request:  
`GET /api/v1/tags/all`

Response:
```json
[
	{
		"id": "baf5ba00-440c-4d8a-a614-be2cbf4c2beb",
		"name": "something",
		"user_id": "6b875d6c-7d4e-4ae3-931f-270604df1d7f",
		"parent_id": "1048e93b-b53e-4bdc-a1b7-98726ef65027"
	}
]
```

| Property 	| Description 												| Type 			| Filterable 	| Sortable 		|
| --------- | ----------------------------------- | --------- | ----------- | ----------- |
| id 				| id of the tag 											| UUIDv4		| Yes					| No					|
| name 			| name of the tag 										| string		| Yes					| No					|
| user_id 	| user_id that owns this tag 					|	UUIDv4		| Yes					| No					|
| parent_id | id of the parent tag, if specified	|	UUIDv4?		| Yes					| No					|


### get by id

This endpoint is used to retrieve a single tag by its id. 
You need to specify the id of the tag you want to retrieve in the request path. 

Request:  
`GET /api/v1/tags/{tag_id}`

Response:
```json
{
	"id": "baf5ba00-440c-4d8a-a614-be2cbf4c2beb",
	"name": "something",
	"user_id": "6b875d6c-7d4e-4ae3-931f-270604df1d7f",
	"parent_id": "1048e93b-b53e-4bdc-a1b7-98726ef65027"
}
```

| Property 	| Description 												| Type 			|
| --------- | ----------------------------------- | --------- |
| id 				| id of the tag 											| UUIDv4		|
| name 			| name of the tag 										| string		|
| user_id 	| user_id that owns this tag 					|	UUIDv4		|
| parent_id | id of the parent tag, if specified	|	UUIDv4?		|


### create tag

This endpoint is used to create a new tag.

Request:  
`POST /api/v1/tags`
```json
{
	"name": "something",
	"parent_id": "1048e93b-b53e-4bdc-a1b7-98726ef65027"
}
```

| Property 	| Description 												| Type 			|
| --------- | ----------------------------------- | --------- |
| name 			| name of the tag 										| string		|
| parent_id | id of the parent tag, if specified	|	UUIDv4?		|


### update tag

This endpoint is used to update an existing tag.  
You need to specify the id of the tag you want to modify in the request path. 

Request:  
`PUT /api/v1/tags/{tag_id}`
```json
{
	"name": "something else",
	"parent_id": "1048e93b-b53e-4bdc-a1b7-98726ef65027"
}
```

| Property 	| Description 												| Type 			|
| --------- | ----------------------------------- | --------- |
| name 			| name of the tag 										| string		|
| parent_id | id of the parent tag, if specified	|	UUIDv4?		|


### delete tag

This endpoint is used to delete an existing tag. This can be safely called even if the tag is still referenced in other places. Other tags that have the to be deleted tag as a parent will get `null` set as the parent_id. Transactions, etc will simply get this tag removed as well.  
You need to specify the id of the tag you want to delete in the request path. 

Request:  
`DELETE /api/v1/tags/{tag_id}`