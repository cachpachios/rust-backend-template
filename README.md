# Rust Backend Template

At some point I realized I was doing the same setup over and over.

So here is a prepared template for a good Rust backend project.

Currently it is a simple CRUD API for weather forecasts, but it can be used as a starting point for any other project where you are using postgres and want a OpenAPI documented API.

## Features

- Tokio running Axum, most used Rust web framework
- OpenAPI documentation at `/api-docs/openapi.json` using **utoipa**, Swagger UI running on `/swagger-ui`
- Prepared postgres with ORM and migrations using **Diesel** and connection pooling with **bb8**
- Custom `Error` type with `reason` body

## Usage

Copy the whole project, rename stuff and go nuts.
The database URL needs to be provided by the `DATABASE_URL` environment variable.
See `db.rs` if you want to change this...

### Example run

1. Spin up a PostgreSQL database in whatever preferred way. Example with Docker: 
```bash
docker run --name postgres -e POSTGRES_PASSWORD=password -p 5432:5432 -d postgres
```

2. Set the `DATABASE_URL` environment variable:
```bash
export DATABASE_URL=postgres://postgres:password@localhost/postgres
```
3. Run the database migrations with [Diesel CLI](https://diesel.rs/guides/getting-started#installing-diesel-cli):

```bash
diesel migration run
```

4. Run the project with `cargo run`

5. Open the Swagger UI at `http://localhost:8080/swagger-ui`

## Production Improvements

From this template, to build a production-ready backend, consider the following improvements:

- Abstract the database layer with a trait following a repository pattern to allow testing. Use Axum's `Extension` to inject the database implementation.

- You probably want some kind of authentication validation, ideally as a middleware if using JWTs or similar.

## License

Public domain. Go nuts, no attribution required.
