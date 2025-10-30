# 🔗 URL Shortener API (Rust + Salvo + DDD)

Proyek ini adalah **URL Shortener API** modern berbasis **Rust** menggunakan framework **Salvo** dengan pola **Domain-Driven Design (DDD)**.  
API ini memungkinkan pengguna membuat tautan pendek, mengatur tanggal kedaluwarsa, melakukan redirect otomatis, serta menampilkan daftar semua URL yang tersimpan.

---

## 🧱 Teknologi yang Digunakan

- **Rust** – bahasa utama, aman dan cepat ⚙️
- **Salvo** – framework web asinkron yang ringan dan fleksibel
- **Tokio** – runtime async untuk performa tinggi
- **SQLx** – ORM asynchronous untuk PostgreSQL
- **PostgreSQL** – database utama
- **Docker + Docker Compose** – containerization & environment setup
- **salvo_oapi (OpenAPI)** – dokumentasi Swagger otomatis
- **chrono** – manajemen waktu (UTC & lokal)
- **uuid, serde, anyhow, tracing** – helper crates

---

## 📂 Struktur Direktori (DDD Pattern)

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
│   │   └── url_validator.rs # Validasi & normalisasi URL
│   └── datetime_format.rs   # Format tanggal (UTC <-> Local)
│
├── infrastructure/
│   ├── database.rs          # Inisialisasi koneksi PostgreSQL
│   └── repositories.rs      # Implementasi repository untuk Postgres
│
├── presentation/
│   ├── handlers.rs          # Endpoint Salvo (create, redirect, list)
│   └── routes.rs            # Definisi route & dokumentasi Swagger
│
└── main.rs                  # Entry point aplikasi
```

---

## 🚀 Menjalankan Proyek

### 1️⃣ Clone repository

```bash
git clone https://github.com/username/url-shortener-rust.git
cd url-shortener-rust
```

### 2️⃣ Copy file environment

```bash
cp .env.example .env
```

Isi `.env` seperti berikut:

```env
DATABASE_URL=postgres://postgres:password@db:5432/db_name
RUST_LOG=info
PORT=8000
```

---

## 🧩 Endpoint API

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
  "created_at": "2025-10-29 14:42:59",
  "expires_at": "2025-10-29 14:42:30"
}
```

---

### 2. **Redirect Short URL**

`GET /api/v1/{code}`

**Jika valid**

> Mengarahkan pengguna ke URL tujuan.

**Jika sudah kedaluwarsa**

```json
{
  "error": "url expired",
  "expired_at": "2025-10-29 14:20:30"
}
```

**Jika tidak ditemukan**

```json
{
  "error": "Not Found"
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
    "created_at": "2025-10-29 14:42:59",
    "expires_at": "2025-11-01 12:00:00"
  }
]
```

---

## 📘 Dokumentasi API

Swagger UI otomatis tersedia di:

```
http://localhost:8000/documentation
```

File OpenAPI JSON dapat diakses di:

```
http://localhost:8000/api-doc/openapi.json
```

---

## 🧠 Catatan Penting

- Field `expires_at` menggunakan **format lokal**: `YYYY-MM-DD HH:MM:SS`
- Jika field `expires_at` kosong, URL dianggap **tidak memiliki batas waktu**
- Otomatis menambahkan prefix `https://` jika user memasukkan domain tanpa protokol
- Semua waktu (`created_at`, `expires_at`) otomatis diformat ke zona waktu lokal

---

## 🧑‍💻 Kontributor

- **Nama:** Arjun Gunawan
- **Stack:** Rust, Salvo, PostgreSQL
- **Pattern:** Domain-Driven Design (DDD)

---

## 📄 Lisensi

Proyek ini dirilis di bawah lisensi **MIT**.  
Bebas digunakan dan dimodifikasi dengan mencantumkan atribusi.
