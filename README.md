# Payment Backend Service

**Payment Backend Service** contains RESTful API endpoints for User, Account and Transaction Management.

## 🛠️ Technologies Used

* **Programming Language**: Rust
* **Containerization**: Docker
* **Database**: PostgreSQL
* **ORM**: SeaORM
* **Web Framework**: Rocket

## 📁 Project Structure

```
├── api_docs..md
├── Cargo.lock
├── Cargo.toml
├── docker-compose.yml
├── README.md
├── src
│   ├── auth
│   │   └── mod.rs
│   ├── controllers
│   │   ├── accounts.rs
│   │   ├── auth.rs
│   │   ├── mod.rs
│   │   ├── profile.rs
│   │   └── transactions.rs
│   ├── db
│   │   └── mod.rs
│   ├── entities
│   │   ├── account.rs
│   │   ├── mod.rs
│   │   ├── prelude.rs
│   │   ├── transactions.rs
│   │   ├── txns.rs
│   │   └── users.rs
│   ├── fairings
│   │   ├── cors.rs
│   │   └── mod.rs
│   ├── kafka
│   │   ├── consumer.rs
│   │   ├── mod.rs
│   │   └── producer.rs
│   ├── lib.rs
│   ├── main.rs
│   ├── migrator
│   │   ├── m20250521_135328_create_users_table.rs
│   │   ├── m20250521_135711_create_accounts_table.rs
│   │   ├── m20250521_135737_store_transactions_table.rs
│   │   └── mod.rs
│   └── utils
│       ├── mod.rs
│       ├── random.rs
│       └── validations.rs
└── tests
    └── endpoint_test.rs

11 directories, 33 files
```



## ⚙️ Getting Started

### Prerequisites

* Rust (latest stable version)
* Docker & Docker Compose
* psql (PostgreSQL) 17.5 (Ubuntu 17.5-1.pgdg24.04+1)

### Installation

1. **Clone the repository**:

   ```bash
   git clone https://github.com/X0rD3v1L/payment-backend-service.git
   cd payment-backend-service
   ```

2. **Set up environment variables**:

   Create a `.env` file in the root directory and configure the necessary environment variables.
   ```
   PAYMENTS_JWT_SECRET="REDACTED"
   ```

   To generate,
   ```openssl rand -base64 64```


3. **Build and run the application**:
   
   Make sure a database named `payments` exists in your local psql.

   ```bash
   cargo build
   cargo run
   ```

   In another terminal,
   ```bash
   docker-compose up 
   ```

The service should now be running and accessible at `http://127.0.0.1:8000`.

## 🧪 Running Tests

To execute the test suite:

```bash
cargo test
```

A sample test case was written to test the usage, comprenshive unit tests and integration tests need to be written.

## 📄 API Documentation

Detailed API documentation is available in the [`api_docs.md`](./api_docs.md) file. It provides comprehensive information on available endpoints, request/response structures, and usage examples.