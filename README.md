## Overview

Role and database are created so the role owns the data; the server exposes task CRUD via `/tasks` (list/create) and `/tasks/{id}` (update/delete) using JSON bodies.
PostgreSQL setup establishes authentication; HTTP requests supply path IDs plus JSON fields (`name`, `priority`) to create or modify rows.

## PostgreSQL Setup

```sql
CREATE ROLE axum_postgres WITH LOGIN PASSWORD 'axum_postgres';
CREATE DATABASE axum_postgres OWNER axum_postgres;
```

```sql
CREATE TABLE tasks (
    task_id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    priority INT
);
```

## Routes

| Method | Path        | Purpose        |
| ------ | ----------- | -------------- |
| GET    | /tasks      | List all tasks |
| POST   | /tasks      | Create a task  |
| PATCH  | /tasks/{id} | Update a task  |
| DELETE | /tasks/{id} | Delete a task  |

## Examples

Create:

```bash
curl -X POST http://localhost:7878/tasks \
    -H 'Content-Type: application/json' \
    -d '{"name":"play cricket","priority":100304234}'
```

Update:

```bash
curl -X PATCH http://localhost:7878/tasks/2 \
    -H 'Content-Type: application/json' \
    -d '{"name":"play football","priority":3}'
```

Delete:

```bash
curl -X DELETE http://localhost:7878/tasks/2
```

List:

```bash
curl http://localhost:7878/tasks
```

## Notes

- The current server configuration does not define a `GET /tasks/{id}` route.
- Adjust the `DATABASE_URL` if you change credentials or port.
