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
		"id": 42,
		"name": "something",
		"user_id": 0,
		"parent_id": 3 | null
	}
]
```

| Property | Description |
| ----------- | ----------- |
| id | id of the tag |
| name | name of the tag |
| user_id | user_id that created this tag |
| parent_id | id of the parent tag or `null` if not set |
### get by id

This endpoint is used to retrieve a single tag by its id. 
You need to specify the id of the tag you want to retrieve in the request path. 

Request:  
`GET /api/v1/tags/{tag_id}`

Response:
```json
{
	"id": 42,
	"name": "something",
	"user_id": 0,
	"parent_id": 3 | null
}
```

| Property | Description |
| ----------- | ----------- |
| id | id of the tag |
| name | name of the tag |
| user_id | user_id that created this tag |
| parent_id | id of the parent tag or `null` if not set |

### create tag

This endpoint is used to create a new tag.

Request:  
`POST /api/v1/tags`
```json
{
	"name": "something",
	"parent_id": 3
}
```

| Property | Description |
| ----------- | ----------- |
| name | name of the tag |
| parent_id | (optional) id of the parent tag | 

Response: empty  

### update tag

This endpoint is used to update an existing tag.

Request:  
`PUT /api/v1/tags/{tag_id}`
```json
{
	"name": "something",
	"parent_id": 3
}
```

| Property | Description |
| ----------- | ----------- |
| name | name of the tag |
| parent_id | (optional) id of the parent tag | 

Response: empty  

### delete tag

This endpoint is used to delete an existing tag. This can be safely called even if the tag is still referenced in other places. Other tags that have the deleted tag as a parent will get `null` set as the parent_id. Transactions, etc will simply get this tag removed as well.

Request:  
`DELETE /api/v1/tags/{tag_id}`

Response: empty  