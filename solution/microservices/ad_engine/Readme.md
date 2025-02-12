# Configuration

The project is configured from the [/conf](/solution/conf/) and `env`.

It is recommended to explicitly use the environment variable `APP_ENVIRONMENT` (values: `prod`, `local`), otherwise the default value (`prod`) will be substituted. For example:

```python
APP_ENVIRONMENT="local"
```

The configuration consists of three consecutive steps:

1. **Load** configuration from [base.yaml](/solution/conf/base.yaml).
2. **Load or rewrite** configuration from [local.yaml](/solution/conf/local.yaml) or [prod.yaml](/solution/conf/prod.yaml) file according to the variable `APP_ENVIRONMENT`.
3. **Load or rewrite** confguration from `ENV` variable or [.env](/solution/.env) file. In this case, the nesting is determined by the separator `__`. You should definitely add a prefix `APP__` to the variable. For example:

   ```python
   APP_ENVIRONMENT="local"

   APP__HTTP_SERVER__PORT=8000  # http_server.port
   APP__HTTP_SERVER__HOST="127.0.0.1"   # http_server.host
   ```

All settings in configuration files:
```yml

http_server:
  port: 8000
  host: "127.0.0.1"
  path_swagger_docs: "/docs"
  path_openapi_docs: "/docs"
  start_workers: 8
  timeout_shutdown_workers: 30 # sec
  keep_alive: 75 # sec
  limit_size_json: 2048 # 2048 = 2kb

logger:
  max_level_cmd: "debug"
  max_level_file: "debug"
  max_level_error_file: "warn"
  log_dir: "./log"

database:
  postgres:
    postgres_conn: "postgres://postgres:44541@localhost:5555/pulse"
    max_connections: 5

cors:
  allowed_origin: "http://localhost:8001/"
  allowed_methods: ["GET", "POST", "PUT", "UPDATE", "DELETE"]
  allowed_headers: ["Content-Type", "Authorization"]
  max_age: 3600

hashing:
  random_secret: "random1234"
```


# Architecture

Project uses `clean architecture` with the following folder structure:

| Name of Folder                                  | Description                                                 | Example Body                                           | Depends On                                                                       |
| ----------------------------------------------- | ----------------------------------------------------------- | ------------------------------------------------------ | -------------------------------------------------------------------------------- |
| [infrastructure](/solution/src/infrastructure/) | Contains components for interaction with external resources | `redis`, `database`, `config`, `repository`            | `None`                                                                           |
| [domain](/solution/src/domain/)                 | Holds core business logic, types, and services              | `validators`, `types`, `services`, `schemas`, `traits` | [infrastructure](/solution/src/infrastructure/)                                  |
| [interface](/solution/src/interface/)           | Manages communication between user interactions and backend | `http_client`, `middleware`, `websocket`, `routers`    | [infrastructure](/solution/src/infrastructure/), [domain](/solution/src/domain/) |

When adding new folders or features, keep in mind **_the convention of organizing folders_**.

| Name folder                                     | Note                                                                                                                                                                        | Template                               | Example                                     |
| ----------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------- | ------------------------------------------- |
| [infrastructure](/solution/src/infrastructure/) | In the `<name_infra>` module, provide the traits that the `<lib_or_framefork>` module implements. This is the only reason why the library is connected to the `lib.rs`.     | `/<name_infra>/<lib_or_framefork>`     | `/database_connection/sqlx`, `/hash/argon2` |
| [domain](/solution/src/domain/)                 | Avoid large folder nesting. (no more than 3 is recommended)                                                                                                                 | `/{name_folder}`                       | `/schemas`, `/services`                     |
| [interface](/solution/src/interface/)           | In the `<lib_or_framework>` module, provide the traits that the `<name_interface>` module implements. This is the only reason why the library is connected to the `lib.rs`. Each subsequent folder should be an abstraction (represent a logical unit that contains related elements). | `/<lib_or_framework>/<name_interface>` | `/actix/http_client`, `/actix/routers`      |


There should be **no** rigid binding to the framework and lib. 

To add a `new router`, you need to go through several stages:
1) Create or update [usecase](./src/domain/usecase/)
2) Create or update [service](./src/domain/services/)
3) Create or update [repository](./src/infrastructure/repository/)

```mermaid
graph LR;
    A[Router] -->|Request Processing| B[Use Case]
    B -->|Business Logic| C[Service]
    C -->|Data Access| D[Repository]

```

- [usecase](./src/domain/usecase/):
Use case performs a specific business operation and can interact with multiple services. It manages the logic that defines how data and operations are related to each other.

- [service](./src/domain/services/):
Each service can manage multiple repositories, providing access to data from different sources. Services usually contain business logic related to changes or manipulations of the data they provide.

- [repository](./src/infrastructure/repository/):
Repositories are responsible for interacting with data sources (for example, databases, APIs, etc.). They implement a pattern of storing and providing data.


For example:

```mermaid
graph TD;
    A[Router] -->|Calls| B[Use Case: Add User]
    B -->|Calls| C[Service: Auth User]
    B -->|Calls| D[Service: User Service]
    C -->|Data Access| E[Repository: User Repository]
    D -->|Data Access| E
    B -->|Send Notifications| F[Service: Notifications]
    F -->|Data Access| G[Repository: Notifications Repository]
```

# Documentation

You can get the **_swagger_** documentation for the written API by going to [/docs/](http://127.0.0.1:8000/docs/). If this not work would look in `conf` [files](/conf/base.yaml) (There is has settings `path_swagger_docs`).

If you are backend developer:

- Write `rust` docs on [domain](/solution/src/domain/)
- Write `swagger` docs on [interface](/solution/src/interface/)
- Get docs:

  ```powershell
  cargo doc  --lib --open --no-deps --document-private-items
  ```
