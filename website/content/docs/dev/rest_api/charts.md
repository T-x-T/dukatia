---
title: 'charts'
description: 'Chart related API endpoints'
---

## Purpose

The charts route contains all endpoints related to the management of charts. They also allow you to retrieve chart data.


### get by id

This endpoint is used to retrieve the configuration of a single chart by its id. 
You need to specify the id of the chart you want to retrieve in the request path.

Request:  
`GET /api/v1/charts/{chart_id}`

Response:
```json
{
	"id": "a6a49dc0-9118-41f4-bf47-f12a83b0f91a",
	"user_id": "6b875d6c-7d4e-4ae3-931f-270604df1d7f",
	"chart_type": "line",
	"title": "My very fun chart",
	"filter_from": "2023-01-01T10:10:00Z",
	"filter_to": "2023-02-01T10:10:00Z",
	"filter_collection": "get_per_recipient_over_time",
	"date_period": "monthly",
	"asset_id": null,
	"budget_id": null,
	"max_items": 10,
	"date_range": null,
	"only_positive": false,
	"only_negative": false,
	"top_left_x": 2,
	"top_left_y": 2,
	"bottom_right_x": 4,
	"bottom_right_y": 6,
	"dashboard_id": null
}
```

| Property 					| Description 																																									| Type				|
| ----------------- | ---------------------------------------------------------------------------------------------	| ----------- |
| id								| the id of the chart																																						| UUIDv4			|
| user_id						| the id of the user owning this chart																													| UUIDv4			|
| chart_type				| the type of the chart, current used values are line, pie and table														| string			|
| title		 					| the title of the chart																																				| string			|
| filter_from				| the default value for the from field																													| timestamp?	|
| filter_to					| the default value for the to field																														| timestamp?	|
| filter_collection	| the collection to use for this chart																													| string?			|
| date_period				| the default value for the period field, defines over which period values get grouped together	| string?			|
| asset_id					| the id of the asset that this chart relates to																								| UUIDv4?			|
| budget_id					| the id of the budget that this chart relates to																								| UUIDv4?			|
| max_items					| the maximum number of items, like slices in the pie																						| number?			|
| date_range				| the default value for the range field, goes from 0 (last 28 days) to 7 (total)								| number?			|
| only_positive			| only return datasets were the last value is positive																					| boolean?		|
| only_negative			| only return datasets were the last value is negative																					| boolean?		|
| top_left_x				| value used for positioning chart on the dashboard grid																				| number?			|
| top_left_y				| value used for positioning chart on the dashboard grid																				| number?			|
| bottom_right_x		| value used for positioning chart on the dashboard grid																				| number?			|
| bottom_right_y		| value used for positioning chart on the dashboard grid																				| number?			|
| dashboard_id			| the id of the dashboard this chart belongs to, in reality always null													| UUIDv4?			|


### get all charts in dashboard

This endpoint is used to retrieve the configuration of all charts that are part of a particular dashboard. 
You need to specify the id of the dashboard you want to retrieve the charts of in the request path.

Request:  
`GET /api/v1/dashboards/{dashboard_id}/charts`

Response:
```json
[
	{
		"id": "a6a49dc0-9118-41f4-bf47-f12a83b0f91a",
		"user_id": "6b875d6c-7d4e-4ae3-931f-270604df1d7f",
		"chart_type": "line",
		"title": "My very fun chart",
		"filter_from": "2023-01-01T10:10:00Z",
		"filter_to": "2023-02-01T10:10:00Z",
		"filter_collection": "get_per_recipient_over_time",
		"date_period": "monthly",
		"asset_id": null,
		"budget_id": null,
		"max_items": 10,
		"date_range": null,
		"only_positive": false,
		"only_negative": false,
		"top_left_x": 2,
		"top_left_y": 2,
		"bottom_right_x": 4,
		"bottom_right_y": 6,
		"dashboard_id": null
	}
]
```

| Property 					| Description 																																									| Type				|
| ----------------- | ---------------------------------------------------------------------------------------------	| ----------- |
| id								| the id of the chart																																						| UUIDv4			|
| user_id						| the id of the user owning this chart																													| UUIDv4			|
| chart_type				| the type of the chart, current used values are line, pie and table														| string			|
| title		 					| the title of the chart																																				| string			|
| filter_from				| the default value for the from field																													| timestamp?	|
| filter_to					| the default value for the to field																														| timestamp?	|
| filter_collection	| the collection to use for this chart																													| string?			|
| date_period				| the default value for the period field, defines over which period values get grouped together	| string?			|
| asset_id					| the id of the asset that this chart relates to																								| UUIDv4?			|
| budget_id					| the id of the budget that this chart relates to																								| UUIDv4?			|
| max_items					| the maximum number of items, like slices in the pie																						| number?			|
| date_range				| the default value for the range field, goes from 0 (last 28 days) to 7 (total)								| number?			|
| only_positive			| only return datasets were the last value is positive																					| boolean?		|
| only_negative			| only return datasets were the last value is negative																					| boolean?		|
| top_left_x				| value used for positioning chart on the dashboard grid																				| number?			|
| top_left_y				| value used for positioning chart on the dashboard grid																				| number?			|
| bottom_right_x		| value used for positioning chart on the dashboard grid																				| number?			|
| bottom_right_y		| value used for positioning chart on the dashboard grid																				| number?			|
| dashboard_id			| the id of the dashboard this chart belongs to, in reality always null													| UUIDv4?			|


### get data of chart by id

This endpoint is used to retrieve the data of a chart that has been created previously. 
You need to specify the id of the chart you want to retrieve the data of in the request path.
You can use request parameters to override the default parameters of the charts configuration. Possible properties are: from_date, to_date, date_period, asset_id, budget_id, max_items, date_range, only_positive, only_negative, filter_collection.

Request:  
`GET /api/v1/charts/{chart_id}/data`

Response:
```json
{
	"datasets": [
		{
			"label": "My Dataset label",
			"data": [
				{
					"name": null,
					"timestamp": "2024-01-01",
					"value": -12.34,
					"label": "-12.34€"
				}
			]
		}
	]
}
```

| Property 					| Description 													| Type				|
| ----------------- | -------------------------------------	| ----------- |
| datasets					| array of datasets											| dataset[]		|

Dataset:
| Property 					| Description 													| Type				|
| ----------------- | -------------------------------------	| ----------- |
| label							| the label of the dataset							| string			|
| data							| array of chart data										| datapoint[]	|

Datapoint:
| Property 					| Description 													| Type				|
| ----------------- | -------------------------------------	| ----------- |
| name							| the name of the datapoint							| string			|
| timestamp					| the timestamp of the datapoint				| timestamp		|
| value							| the numerical value of the datapoint	| number			|
| label							| the label of the datapoint						| label				|


### get data of chart by filter collection

This endpoint is used to retrieve the data for a chart on demand, without having to first create a chart. 
You need to specify the filter collection of the chart you want to retrieve the data of in the request path.
You can use request parameters to override the default parameters. Possible properties are: from_date, to_date, date_period, asset_id, budget_id, max_items, date_range, only_positive, only_negative, filter_collection.

Request:  
`GET /api/v1/charts/by_collection/{filter_collection}`

Response:
```json
{
	"datasets": [
		{
			"label": "My Dataset label",
			"data": [
				{
					"name": null,
					"timestamp": "2024-01-01",
					"value": -12.34,
					"label": "-12.34€"
				}
			]
		}
	]
}
```

| Property 					| Description 													| Type				|
| ----------------- | -------------------------------------	| ----------- |
| datasets					| array of datasets											| dataset[]		|

Dataset:
| Property 					| Description 													| Type				|
| ----------------- | -------------------------------------	| ----------- |
| label							| the label of the dataset							| string			|
| data							| array of chart data										| datapoint[]	|

Datapoint:
| Property 					| Description 													| Type				|
| ----------------- | -------------------------------------	| ----------- |
| name							| the name of the datapoint							| string			|
| timestamp					| the timestamp of the datapoint				| timestamp		|
| value							| the numerical value of the datapoint	| number			|
| label							| the label of the datapoint						| label				|


### create chart

This endpoint is used to create new charts. 

Request:  
`POST /api/v1/charts`
```json
{
	"chart_type": "line",
	"title": "My very fun chart",
	"filter_from": "2023-01-01T10:10:00Z",
	"filter_to": "2023-02-01T10:10:00Z",
	"filter_collection": "get_per_recipient_over_time",
	"date_period": "monthly",
	"max_items": 10,
	"date_range": null,
	"only_positive": false,
	"only_negative": false,
	"top_left_x": 2,
	"top_left_y": 2,
	"bottom_right_x": 4,
	"bottom_right_y": 6,
	"dashboard_id": null
}
```

| Property 					| Description 																																									| Type				|
| ----------------- | ---------------------------------------------------------------------------------------------	| ----------- |
| chart_type				| the type of the chart, current used values are line, pie and table														| string			|
| title		 					| the title of the chart																																				| string			|
| filter_from				| the default value for the from field																													| timestamp?	|
| filter_to					| the default value for the to field																														| timestamp?	|
| filter_collection	| the collection to use for this chart																													| string?			|
| date_period				| the default value for the period field, defines over which period values get grouped together	| string?			|
| max_items					| the maximum number of items, like slices in the pie																						| number?			|
| date_range				| the default value for the range field, goes from 0 (last 28 days) to 7 (total)								| number?			|
| only_positive			| only return datasets were the last value is positive																					| boolean?		|
| only_negative			| only return datasets were the last value is negative																					| boolean?		|
| top_left_x				| value used for positioning chart on the dashboard grid																				| number?			|
| top_left_y				| value used for positioning chart on the dashboard grid																				| number?			|
| bottom_right_x		| value used for positioning chart on the dashboard grid																				| number?			|
| bottom_right_y		| value used for positioning chart on the dashboard grid																				| number?			|
| dashboard_id			| the id of the dashboard this chart should be added to																					| UUIDv4?			|

Response:
```json
{
	"id": "a6a49dc0-9118-41f4-bf47-f12a83b0f91a"
}
```


### modify chart

This endpoint is used to modify an existing chart.  
You need to specify the id of the chart you want to modify in the request path.

Request:  
`PUT /api/v1/charts`
```json
{
	"chart_type": "line",
	"title": "My very fun chart",
	"filter_from": "2023-01-01T10:10:00Z",
	"filter_to": "2023-02-01T10:10:00Z",
	"filter_collection": "get_per_recipient_over_time",
	"date_period": "monthly",
	"max_items": 10,
	"date_range": null,
	"only_positive": false,
	"only_negative": false,
	"top_left_x": 2,
	"top_left_y": 2,
	"bottom_right_x": 4,
	"bottom_right_y": 6,
	"dashboard_id": null
}
```

| Property 					| Description 																																									| Type				|
| ----------------- | ---------------------------------------------------------------------------------------------	| ----------- |
| chart_type				| the type of the chart, current used values are line, pie and table														| string			|
| title		 					| the title of the chart																																				| string			|
| filter_from				| the default value for the from field																													| timestamp?	|
| filter_to					| the default value for the to field																														| timestamp?	|
| filter_collection	| the collection to use for this chart																													| string?			|
| date_period				| the default value for the period field, defines over which period values get grouped together	| string?			|
| max_items					| the maximum number of items, like slices in the pie																						| number?			|
| date_range				| the default value for the range field, goes from 0 (last 28 days) to 7 (total)								| number?			|
| only_positive			| only return datasets were the last value is positive																					| boolean?		|
| only_negative			| only return datasets were the last value is negative																					| boolean?		|
| top_left_x				| value used for positioning chart on the dashboard grid																				| number?			|
| top_left_y				| value used for positioning chart on the dashboard grid																				| number?			|
| bottom_right_x		| value used for positioning chart on the dashboard grid																				| number?			|
| bottom_right_y		| value used for positioning chart on the dashboard grid																				| number?			|
| dashboard_id			| the id of the dashboard this chart should be added to																					| UUIDv4?			|


### delete chart

This endpoint is used to delete an existing chart.
You need to specify the id of the chart you want to delete in the request path. 

Request:  
`DELETE /api/v1/charts/{budget_id}`