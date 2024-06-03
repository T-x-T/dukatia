---
title: 'users'
description: 'User related API endpoints'
---

## Purpose

The users route contains all endpoints related to user management and login.

### login

This endpoint is used to log the user in. This returns an `access_token` that can be used in future requests.  

Request:  
`POST /api/v1/login`
```json
{
	"name": "admin",
	"secret": "mysupersecretpassword1"
}
```

| Property 		| Description 			| Type		|
| ----------- | ----------------- | ------- |
| name 				| name of the user	| string	|
| secret 			| users password		| string	|

Response:
```json
{
	"access_token": "9aa1cd4df27bd44b71de575c82968f876c296cd7e451c8633ebd8c64b0dfffaabcdd13891fa1b0c7a8ead6690e9fdfb422a636e5a282727936465db8641ba8bf",
	"user_id": "6b875d6c-7d4e-4ae3-931f-270604df1d7f",
	"first_login": true,
}
```

| Property 			| Description 																			| Type		|
| ------------	| ------------------------------------------------- | -------	|
| access_token	| access token used for authenticating api requests	| string	|
| user_id 			| id of the user that the `access_token` belongs to	| UUIDv4	|
| first_login		| true if this was the first login of the user			| boolean	|

You then need to provide the `access_token` as a cookie called accessToken for authenticated API endpoints.


### logout

This endpoint is used to log the current session out. This will invalidate the `access_token` sent in the cookie.

Request:  
`POST /api/v1/logout`


### get me

This endpoint is used to get the information about the user that owns the current session.

Request:  
`POST /api/v1/users/me`

Response:
```json
{
	"id": "6b875d6c-7d4e-4ae3-931f-270604df1d7f",
	"name": "Bobby",
	"secret": null,
	"encrypted_secret": null,
	"superuser": true,
	"active": true,
	"last_logon": "2024-06-02T15:27:18.896236Z"
}
```

| Property		 			| Description 																									| Type			|
| ----------------	| ------------------------------------------------------------- | ---------	|
| id								| id of the currently logged in user														| UUIDv4		|
| name							| name of the currently logged in user													| string		|
| secret						| password of the currently logged in user											| null			|
| encrypted_secret	| encrypted password of the currently logged in user						| null			|
|	superuser					| if the currently logged in user is a superuser								| boolean		|
|	active						| if the currently logged in user is active											| boolean		|
| last_logon				| timestampt of the last logon of the currently logged in user	| timestamp	|


### get all users

This endpoint is used to get information about all users.  
Only superusers are allowed to use this endpoint. Regular users will receive a response with the status code 400.  

Request:  
`POST /api/v1/users/all`

Response:
```json
[
	{
		"id": "6b875d6c-7d4e-4ae3-931f-270604df1d7f",
		"name": "Bobby",
		"secret": null,
		"encrypted_secret": null,
		"superuser": true,
		"active": true,
		"last_logon": "2024-06-02T15:27:18.896236Z"
	}
]
```

| Property		 			| Description 															| Type			|
| ----------------	| ----------------------------------------- | ---------	|
| id								| id of the user														| UUIDv4		|
| name							| name of the user													| string		|
| secret						| password of the user											| null			|
| encrypted_secret	| encrypted password of the user						| null			|
|	superuser					| if the user is a superuser								| boolean		|
|	active						| if the user is active											| boolean		|
| last_logon				| timestampt of the last logon of the user	| timestamp	|


### create user

This endpoint is used to create new users.  
Only superusers are allowed to use this endpoint. Regular users will receive a response with the status code 400. 

Request:  
`POST /api/v1/users`
```json
[
	{
		"name": "Bobby",
		"secret": "my_super_secret_p4ssw0rd",
		"superuser": true
	}
]
```

| Property		 			| Description 															| Type			|
| ----------------	| ----------------------------------------- | ---------	|
| name							| name of the user													| string		|
| secret						| password of the user											| string		|
|	superuser					| if the user is a superuser								| boolean		|


### update user

This endpoint is used to update existing users.  
Only superusers are allowed to use this endpoint. Regular users will receive a response with the status code 400. 
Note that all the fields in the request body are optional.

Request:  
`POST /api/v1/users`
```json
[
	{
		"name": "Bobby",
		"secret": "my_super_secret_p4ssw0rd",
		"superuser": true
	}
]
```

| Property		 			| Description 															| Type			|
| ----------------	| ----------------------------------------- | ---------	|
| name							| name of the user													| string?		|
| secret						| password of the user											| string?		|
|	superuser					| if the user is a superuser								| boolean?	|


### update secret

This endpoint is used to change the secret of the currently logged in user. 

Request:  
`PUT /api/v1/users/me/secret`
```json
{
	"old_secret": "myoldinsecurepassword",
	"new_secret": "myshinynewpassword1"
}
```

| Property		| Description									 | Type		|
| ----------- | ---------------------------- | ------	|
| old_secret	| current password of the user | string	|
| new_secret	| new password of the user		 | string	|


### get dashboards

This endpoint is used to retrieve the dashboards of the currently logged in user. 

Request:  
`GET /api/v1/users/me/dasboards`
```json
[
	{
		"id":	"cdf8a7b5-4758-48b0-806e-63df6744e48d",
		"user_id": "6b875d6c-7d4e-4ae3-931f-270604df1d7f",
		"name": "Default",
		"description": "The default Dashboard"
	}
]
```

| Property		| Description 													| Type		|
| ----------- | ------------------------------------- | -------	|
| id 					| id of the dashboard 									| UUIDv4	|
| user_id 		| id of the user owning the dashboard 	| UUIDv4	|
| name 				| name of the dashboard 								|	string	|
| description | optional description of the dashboard | string?	|
