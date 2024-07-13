---
title: 'Currency handling'
description: 'How currencies are handled'
---

Handling currencies in programming is always a complicated topic. In designing Dukatia I wanted to completely avoid handling currencies as floating point numbers, [because that will lead to inaccuracy](https://stackoverflow.com/questions/3730019/why-not-use-double-or-float-to-represent-currency).  
I started out with just saving monetary values in the minor denomination. So 1 Euro and 23 Cents would be stored as 123. The currency then has a `minor_in_major` value desscribing how many of the smaller denomination fit into the larger one. For most currencies this will be 100, but not for all!  
In the API you will mostly encounter monetary values in the following format:

```json
{
	"major": 1,
	"minor": 23,
	"minor_in_major": 100,
	"symbol": "€",
	"is_negative": false
}
```

Displaying a monetary value with this is quite simple:  

```js
const display = `${major}.${minor}${symbol}`;
```

It is actually a little bit more complicated if you want to properly handle displaying negative amounts that have 0 as their major:  

```js
const display = `${major >= 0 && is_negative ? "-" : ""}${major}.${minor}${symbol}`;
```

You should always set `is_negative` to true when sending a negative monetary value to the API. Additionaly, you should set `major` to a negative number as well, as long as its not 0.