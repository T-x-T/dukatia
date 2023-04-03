---
title: 'users'
description: 'User related API endpoints'
---

## Purpose

The users route contains all endpoints related to user management and login.

### login

This endpoint is used to log the user in. This returns an accessToken that can be used in future requests.

Request:  
`POST /api/v1/login`
```json
{
	"name": "admin",
	"secret": "mysupersecretpassword1"
}
```

| Property | Description |
| ----------- | ----------- |
| name | name of the user |
| secret | users password | 

Response:
```json
{
	"accessToken": "$accessToken"
}
```
You then need to provide the `accessToken` as a cookie called accessToken for authenticated API endpoints.

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

| Property | Description |
| ----------- | ----------- |
| old_secret | current password of the user |
| new_secret | new password of the user | 

Response:   
Empty body