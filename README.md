# Rust Backend Template

At some point i realized i was doing the same setup over and over.
So here is a prepared template for a rust backend project.

## Features

- Tokio running Axum, most used rust web framework
- OpenAPI documentation at `/api-docs/openapi.json` using **utoipa**, Swagger UI running on `/swagger-ui`
- Prepared Postgres with ORM and migrations using **Diesel** and connection pooling with **bb8**.
- Custom `Error` type with `reason` body.

## Usage

Copy the whole project, rename it and go nuts.

## License

Public domain. Go nuts.
