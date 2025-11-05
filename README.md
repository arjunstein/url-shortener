# 🔗 URL Shortener API (Rust + Salvo + DDD)

This project is a modern **URL Shortener API** based on **Rust** using the **Salvo** framework with the **Domain-Driven Design (DDD)** pattern.
This API allows users to create short links, set expiration dates, perform automatic redirects, and display a list of all stored URLs.

---

## 🧱 Technologies Used

- **Rust** – primary language, secure and fast ⚙️
- **Salvo** – lightweight and flexible asynchronous web framework
- **Tokio** – async runtime for high performance
- **SQLx** – asynchronous ORM for PostgreSQL
- **PostgreSQL** – primary database
- **salvo_oapi (OpenAPI)** – automatic Swagger documentation
- **chrono** – time management (UTC & local)
- **uuid, serde, anyhow, tracing** – helper crates

---

## 📂 Directory Structure (DDD Pattern)

```
src/
├── application/
│   ├── dtos.rs              # Data Transfer Objects (Request/Response)
│   └── services.rs          # Business logic dan interface service
│
├── domain/
│   ├── entities.rs          # Domain model (UrlEntity)
│   ├── repositories.rs      # Repository trait
│   ├── validators/
│   │   └── url_validator.rs # Validation & normalization URL
│   └── datetime_format.rs   # Format date (UTC <-> Local)
│
├── infrastructure/
│   ├── database.rs          # Database connection initialization
│   └── repositories.rs      # Implementation repository for Postgres
│
├── presentation/
│   ├── handlers.rs          # Salvo endpoint (create, redirect, list)
│   └── routes.rs            # Route definition & Swagger documentation
│
└── main.rs                  # Application entry point
```

---

## 🚀 Running the Project

### 1️⃣ Clone repository

```bash
git clone https://github.com/username/url-shortener.git
cd url-shortener
```

### 2️⃣ Copy environment file

```bash
cp .env.example .env
```

Fill in the `.env` file like this:

```env
DATABASE_URL=postgres://postgres:password@db:5432/db_name
RUST_LOG=info
PORT=8000
```

---

## 🧩 API Endpoints

### 1. **Create Short URL**

`POST /api/v1/shorten`

**Request**

```json
{
  "target_url": "example.com",
  "expires_at": "2025-10-29 14:20:30"
}
```

**Response**

```json
{
  "id": "eb492660-18e8-4c8f-8a8b-a32744c0c316",
  "short_code": "DZMXE5QR",
  "target_url": "https://example.com/",
  "clicks": 0,
  "created_at": "2025-10-29 14:42:59",
  "expires_at": "2025-10-29 14:42:30"
}
```

---

### 2. **Redirect Short URL**

`GET /api/v1/{code}`

**If valid**

> Redirects to the target URL.

**If expired**

```json
{
  "message": "url expired",
  "expired_at": "2025-10-29 14:20:30"
}
```

**If not found**

```json
{
  "message": "Not Found URL"
}
```

---

### 3. **Get All Short URLs**

`GET /api/v1/shorten`

**Response**

```json
[
  {
    "id": "8a192f9a-4f9d-4512-91da-81f36b3a412a",
    "short_code": "QkW3pLrT",
    "target_url": "https://rust-lang.org/",
    "clicks": 99,
    "created_at": "2025-10-29 14:42:59",
    "expires_at": "2025-11-01 12:00:00"
  }
]
```

---

### 4. **Delete Short URL**

`DELETE /api/v1/shorten/{code}`

**If not found**

```json
{
  "message": "short url not found"
}
```

**If success**

```json
{
  "message": "short url deleted successfully",
  "code": "code"
}
```

---

## 📘 API Documentation

Swagger UI is automatically available at:

```
http://localhost:8000/documentation
```

The OpenAPI JSON file can be accessed at:

```
http://localhost:8000/api-doc/openapi.json
```

---

## 🧠 Important Notes

- Field `expires_at` uses **local format**: `YYYY-MM-DD HH:MM:SS`
- If field `expires_at` is empty, the URL is considered **without expiration time**
- Automatically adds the prefix `https://` if the user enters a domain without a protocol
- All times (`created_at`, `expires_at`) are automatically formatted to local timezone

---

## 📄 License

This project is licensed under the **MIT** license.  
Free to use and modify with attribution.
