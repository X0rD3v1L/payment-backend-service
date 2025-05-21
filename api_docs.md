# Payments Backend REST API Documentation

This document provides details about the endpoints available in the Payments Backend REST API, including request formats, response examples, and authentication requirements.

## Table of Contents
- [Authentication](#authentication)
- [User Management](#user-management)
  - [Register a New User](#register-a-new-user)
  - [User Login](#user-login)
- [Account Management](#account-management)
  - [Get User Balance](#get-user-balance)
- [Transaction Management](#transaction-management)
  - [Create Transaction](#create-transaction)
  - [List Transactions](#list-transactions)
  - [Get Transaction Status](#get-transaction-status)
- [Profile Management](#profile-management)
  - [View Profile](#view-profile)
  - [Update Profile](#update-profile)

## Authentication

The API uses token-based authentication. After successful login, you will receive a JWT token that must be included in the `token` header for all authenticated requests.

Example:
```
token: eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...
```

## User Management

### Register a New User

Creates a new user account in the system.

**Endpoint:** `POST /auth/register`

**Request Body:**
```json
{
    "email": "john.doe@example.com",
    "password": "SecureP@ssw0rd!",
    "profile": {
        "first_name": "John",
        "last_name": "Doe"
    }
}
```

**Response:**
```json
{
    "status": "success",
    "message": "Account created"
}
```

### User Login

Authenticates a user and returns an access token.

**Endpoint:** `POST /auth/login`

**Request Body:**
```json
{
    "email": "john.doe@example.com",
    "password": "SecureP@ssw0rd!"
}
```

**Response:**
```json
{
    "status": "success",
    "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...",
}
```

## Account Management

### Get User Balance

Retrieves the current balance for the authenticated user.

**Endpoint:** `GET /accounts/balance`

**Headers:**
```
token: eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...
```

**Response:**
```json
{
    "status": "success",
    "balance": 3592.2136,
    "currency_code": "INR"
}
```

## Transaction Management

### Create Transaction

Creates a new transaction for the authenticated user.

**Endpoint:** `POST /transactions/create`

**Headers:**
```
token: eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...
```

**Request Body:**
```json
{
    "amount": 300.00,
    "txn_type": "purchase"
}
```

**Response:**
```json
{
    "status": "pending",
    "message": "Transaction created and queued for processing.",
    "transaction": {
        "txn_id": "tx-6632dc59-cfb4-4b4b-81e3-6a1ed9285ca2",
        "account_id": "acc-495e273e-dbfc-4349-b601-1bcf472d4c73",
        "amount": 300.0,
        "currency_code": "INR",
        "txn_type": "purchase",
        "status": "pending",
        "created_at": "2025-05-21T15:52:17.358231+00:00"
    }
}
```

### List Transactions

Retrieves a list of transactions for the authenticated user.

**Endpoint:** `GET /transactions/list`

**Headers:**
```
token: eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...
```

**Response:**
```json
{
    "status": "success",
    "message": "Transactions retrieved successfully.",
    "transactions": [
        {
            "txn_id": "tx-6632dc59-cfb4-4b4b-81e3-6a1ed9285ca2",
            "account_id": "acc-495e273e-dbfc-4349-b601-1bcf472d4c73",
            "amount": 300.0,
            "currency_code": "INR",
            "txn_type": "purchase",
            "status": "success",
            "created_at": "2025-05-21T15:52:17.358231+00:00"
        }
    ]
}
```

### Get Transaction Status

Retrieves the status of a specific transaction.

**Endpoint:** `GET /transactions/status/{transaction_id}`

**Headers:**
```
token: eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...
```

**Example URL:** `/transactions/status/tx-23b53c6a-c993-4e03-bb42-1913c4a2588c`

**Response:**
```json
{
    "status": "success",
    "message": "Transaction status retrieved.",
    "transaction": {
        "txn_id": "tx-6632dc59-cfb4-4b4b-81e3-6a1ed9285ca2",
        "account_id": "acc-495e273e-dbfc-4349-b601-1bcf472d4c73",
        "amount": 300.0,
        "currency_code": "INR",
        "txn_type": "purchase",
        "status": "success",
        "created_at": "2025-05-21T15:52:17.358231+00:00"
    }
}
```

## Profile Management

### View Profile

Retrieves the profile information for the authenticated user.

**Endpoint:** `GET /profile`

**Headers:**
```
token: eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...
```

**Response:**
```json
{
    "status": "success",
    "first_name": "John",
    "last_name": "Doe"
}
```

### Update Profile

Updates the profile information for the authenticated user.

**Endpoint:** `PUT /profile`

**Headers:**
```
token: eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...
```

**Request Body:**
```json
{
    "first_name": "Benarjee",
    "last_name": "Sambangi"
}
```

**Response:**
```json
{
    "status": "success",
    "message": "Profile updated successfully."
}
```