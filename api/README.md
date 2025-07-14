# 🔧 Features

- **Clean Architecture**: Clear separation of domain, infrastructure, and API layers
- **Modular Domains**: Self-contained features (auth, user, products, vb.)
- **SQLx Integration**: Compile-time-checked queries in offline mode
- **JWT Auth**: Secure authentication and authorization
- **OpenAPI Docs**: Swagger UI powered by Utoipa

---

## 📦 Project Structure

Recommended layout:

```text
├── src/
│   ├── main.rs                         # Application entry point
│   ├── app.rs                          # Router setup and middleware
│   ├── lib.rs                          # Module declarations
│   ├── common.rs
│   ├── common/                         # Shared components and utilities
│   │   ├── app_state.rs                # AppState struct for dependency injection
│   │   ├── bootstrap.rs                # Service initialization and AppState construction
│   │   ├── config.rs                   # Environment variable configuration loader
│   │   ├── dto.rs                      # Shared/global DTOs
│   │   ├── error.rs                    # AppError enum and error mappers
│   │   ├── hash_util.rs                # Hashing utilities (e.g., bcrypt)
│   │   ├── jwt.rs                      # JWT encoding, decoding, and validation
│   │   ├── multipart_helper.rs         # Multipart Helper
│   │   └── ts_format.rs                # Custom timestamp serialization formatting

│   ├── domains.rs                      # Domain modules declarations
│   ├── domains/                        # Feature modules
│   │   ├── <feature>/                  # e.g., auth, user, device, file
│   │   │   ├── api/
│   │   │   │   ├── handlers.rs         # Route handlers
│   │   │   │   └── routes.rs           # Route definitions
│   │   │   ├── domain/                 # Domain models, traits
│   │   │   │   ├── model.rs
│   │   │   │   ├── repository.rs
│   │   │   │   └── service.rs
│   │   │   ├── dto/                    # Data Transfer Objects
│   │   │   │   └── <feature>_dto.rs
│   │   │   └── infra/                  # Infrastructure-layer implementations
│   │   │       ├── impl_repository.rs
│   │   │       └── impl_service.rs
│   │   ├── <feature>.rs                 # Module entry point
├── .env                                # Environment variables for local development
├── .env.test                           # Environment overrides for test environment
```

### Prerequisites

- Rust (latest stable)
- PostgreSQL
- Docker & Docker Compose (optional)

### API Documentation

## 💡 Architecture

- **Domain**: Traits and models define core business logic.
- **Infra**: Concrete implementations (SQLx repositories, services)
- **API**: Axum handlers and route definitions
- **DTOs**: Typed request/response contracts
- **Bootstrap**: Wires dependencies into `AppState`

1. Create `domains/<feature>/` with `api/`, `domain/`, `infra/`, `dto/`
2. Register in `domains.rs`, `app.rs`, `common/app_state.rs`, `common/bootstrap.rs`

## 🧠 Domain-Driven Design & Architecture

### Domain Layer

- `model.rs`: holds your core structs and enums that represent entities or value objects.
- **Model Type Reference**: Conversions between Rust and PostgreSQL types.
  [See SQLx Postgres types mapping](https://docs.rs/sqlx/latest/sqlx/postgres/types/index.html)
- `repository.rs`: declares the trait(s) that encapsulate persistence operations for the feature (e.g., `UserRepository`).
- `service.rs`: declares the trait(s) for feature service operations.

### Infra Layer

Each feature owns its own `impl_repository.rs` and `impl_service.rs`

`sqlx::query`

- Runtime-checked
- Flexibility: Handy when the SQL must be constructed dynamically—adding WHERE clauses on the fly, for instance.

`sqlx::query!`

- Compile-time-checked: The macro reads your SQL at build time (in “offline mode” if configured) and verifies it against your database schema. Mistyped column names or wrong argument types become compiler errors, not runtime surprises.
- Automatic type inference: You list your Rust values after the SQL string, and SQLx figures out how to map them to the placeholder types ($1, $2, …).
- Struct-level safety: If you use query_as!, it also confirms that the columns you select match the fields of your target struct.

### API Layer

- Route handlers accept DTOs, invoke feature logic, and return serialized responses.
- Each feature owns its own `routes.rs` and `handlers.rs`.
- Supports asynchronous multipart file uploads with validation.
- Secure file serving validates user permissions and sanitizes file paths.

### DTOs & Validation

- Request and response DTOs reside in each feature's `dto.rs`.
- Explicit mapping between DTOs and feature models.
- Uses `serde` and optionally the [validator](https://docs.rs/validator) crate for input validation.

### Use Case Isolation & Dependency Inversion

- Domain service traits define business contracts.
- Concrete implementations live in `impl_service.rs`, constructed via factory methods.
- `bootstrap.rs` wires services and builds `AppState` for dependency injection.

---

## 📚 API Documentation

- Swagger UI is available at `/docs` (powered by Utoipa). Open [http://localhost:8080/docs](http://localhost:8080/docs) in your browser for Swagger UI.
- DTOs and endpoints are annotated for OpenAPI specification.

---

## 📦 API Response Format

All endpoints return a consistent JSON envelope:

```json
{
  "status": 200,
  "message": "success",
  "data": { ... }
}
```

Implemented as:

- `ApiResponse<T>` – generic response wrapper
- `RestApiResponse<T>` – wrapper implementing Axum's `IntoResponse` trait

See definitions in `common/dto.rs`.

---

## 🚨 Error Handling

- Centralized `AppError` enum implements `IntoResponse`.
- Errors map to appropriate HTTP status codes with JSON structure, e.g.:

```json
{
  "status": 400,
  "message": "Invalid request data",
  "data": null
}
```

## 🧪 Environment Configuration

Configure via `.env` at the project root.
Set database URL, JWT secret, service port, and asset settings.

Example `.env`:

```env
DATABASE_URL=postgres://testuser:pass@localhost:5432/testdb
JWT_SECRET_KEY=your_super_secret_key
SERVICE_PORT=8080
```

### Useful Links

- [Axum](https://docs.rs/axum)
- [SQLx](https://docs.rs/sqlx)
- [Utoipa (OpenAPI)](https://docs.rs/utoipa)
- [Tokio](https://tokio.rs/)
- [Validator (crate)](https://docs.rs/validator)
