Proof-of-concept implementation demonstrating:

- HTTP JSON API with ergonomics of _serde_ ecosystem and _axum_

- CRUD operations on PostgreSQL with ergonomics of _diesel_

- Using _actor pattern_ in _tokio_ ecosystem (inspired by Alice Ryhl: _Actors
  with Tokio_, RustLab Conference 2022)

### Cheatsheet

- Starting a containerized PostgreSQL instance (using Podman v4.3.1):

  ```console
  podman run --rm \
    --name poc-postgres \
    -e POSTGRES_PASSWORD=postgres \
    -p 127.0.0.1:5432:5432/tcp \
    docker.io/library/postgres:17.6-trixie@sha256:feff5b24fedd610975a1f5e743c51a4b360437f4dc3a11acf740dcd708f413f6
  ```

- Creating a table named `books` in the containerized PostgreSQL instance:

  ```console
  podman exec -it poc-postgres psql -U postgres -d postgres -c '
    CREATE TABLE books (
      id    UUID PRIMARY KEY,
      title VARCHAR NOT NULL
    );'
  ```

- POST a book:

  ```console
  curl http://127.0.0.1:8080/api/books/v1 --json '{"id":"00000000-0000-0000-0000-000000000000","title":"Foo Bar!"}'
  ```

- GET books:

  ```console
  curl http://127.0.0.1:8080/api/books/v1
  ```

  ```json
  [{ "id": "00000000-0000-0000-0000-000000000000", "title": "Foo Bar!" }]
  ```
