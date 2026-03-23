# Advanced Engineer Challenge

Build a simple auth module ( register / login / reset) using DDD patterns.

- Map out the bounded context and ubiquitous language
- Model the aggregates
- Define domain events and transaction boundaries
- Keep layers clean (domain vs application vs infrastructure)
- In-memory storages are totally fine

Make a short README explaining your architectural decisions.

Show conscious design, not just CRUD.


## App schema

![App schema!](./schema.excalidraw.svg "App schema")

## Setup

Build project

```
cargo build
```

Generate client and server gRPC code

```
cargo build --package grpc
```

Run tests

```
cargo test
```

Start server

```
cargo run --bin server_app
```


## Example of requests

If you have a gRPC GUI client such as [Postman] you should be able to send requests to the server

Or if you use [grpcurl] then you can simply try send requests like this:

[postman]: https://www.postman.com/
[grpcurl]: https://github.com/fullstorydev/grpcurl

Register new user account

```
grpcurl -plaintext -import-path ./grpc/proto -proto auth_service.proto -d '{"email": "user1@host.com", "password": "12345678", "confirm_password": "12345678"}' '[::1]:50051' auth_service.AuthService/RegisterUser
```

Login by email and password
```
grpcurl -v -plaintext -import-path ./grpc/proto -proto auth_service.proto -d '{"email": "user1@host.com", "password": "12345678"}' '[::1]:50051' auth_service.AuthService/AuthUser
```
```
Response headers received:
authorization: eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJhY2NvdW50X2lkIjoiOTA1ZmFhMTMtM2MyZC00YzVkLWFlOGItYWRhYWE0MmYyMTM0IiwiYWNjb3VudF9lbWFpbCI6InVzZXIxQGhvc3QuY29tIiwiZXhwIjoxNzc0Mjc3OTYzfQ.WLGTbKOzmwmnnDnQlIicf0T_9SpdTQHvIexyh3csgiU
```

Access to protected endpoint by specifying authorization token (from previous step)

```
grpcurl -v -plaintext -import-path ./grpc/proto -proto auth_service.proto -H "authorization: eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJhY2NvdW50X2lkIjoiOTA1ZmFhMTMtM2MyZC00YzVkLWFlOGItYWRhYWE0MmYyMTM0IiwiYWNjb3VudF9lbWFpbCI6InVzZXIxQGhvc3QuY29tIiwiZXhwIjoxNzc0Mjc3OTYzfQ.WLGTbKOzmwmnnDnQlIicf0T_9SpdTQHvIexyh3csgiU" '[::1]:50051' auth_service.AuthService/WhoAmI
```

Reset password


1. Ask to send reset password code

```
grpcurl -plaintext -import-path ./grpc/proto -proto auth_service.proto -d '{"email": "user1@host.com"}' '[::1]:50051' auth_service.AuthService/SendResetPasswordCode
```
```
To:user1@host.com
Subject:Reset password code
Body:de39615a-2de3-43e1-8448-2e81fc3d5f12
```

2. Reset password by providing a reset password code (from previous step) and new password

```
grpcurl -plaintext -import-path ./grpc/proto -proto auth_service.proto -d '{"email": "user1@host.com", "password": "123456789", "confirm_password": "123456789", "reset_password_code": "de39615a-2de3-43e1-8448-2e81fc3d5f12"}' '[::1]:50051' auth_service.AuthService/ResetPassword
```

## Why Rust?

Statically typed language

Predictable Performance (no garbage collector)

Safe (ownership & borrowing)

Excellent portability (web browsers (via WASM), microcontrollers (via no_std), Android and iOS (via UniFFI))

## Why Free Monads?

Using Free Monads in DDD separates domain logic ("what to do") from implementation details ("how to do it") by representing business operations as pure data structures (ASTs).

This allows modeling complex workflows as operations that are later interpreted and executed by executors (e.g., database, API).

## What todo next

Collect telemetry

Improve error types (from String to Enums)

