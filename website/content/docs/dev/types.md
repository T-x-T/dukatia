---
title: 'Types'
description: 'Defining types used in the developer rest api documentation'
---

## types
- `UUIDv4`: Version 4 UUID, always in the long form with dashes
- `string`: a regular string
- `number`: a regular floating point json number
- `Money`: an object containing numbers major, minor, minor_in_major, a string for the symbol and a bool is_negative
- `Object`: an object whose contents arent clearly defined
- `timestamp`: an iso8601 timestamp encoded as a string, like "2023-01-01T10:10:00Z". always in UTC
- `boolean`: true or false
- `Position`: a position of a Transaction, contains an optional id as UUIDv4, amount as Money and an optional comment as string? and tag_id as UUIDv4?

## suffixes
Types can have suffixes that give additional info.
- `?`: this value is optional. can be null
- `[]`: this is an array