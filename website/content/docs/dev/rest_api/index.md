---
title: 'REST API'
description: 'Introduction to the REST API of dukatia'
---

## REST API

Dukatia provides a complete REST API to interface with the backend. The frontend only uses the REST API documented here to communicate with the backend. This allows you to extend the functionality of dukatia by writing your own programs and scripts that use and modify data via this API.

Each page contains one major API route that can contain multiple endpoints.

### Guide

Most API endpoints require authentication. For this you first need to acquire an access token.

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
	"access_token": "$access_token"
}
```
You then need to provide the `access_token` as a cookie called access_token for authenticated API endpoints.