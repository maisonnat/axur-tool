# API Reference

## Base URL
- Development: `http://localhost:3001/api`
- Production: Configured via environment

## Authentication

### Login
```http
POST /api/auth/login
Content-Type: application/json

{
  "email": "user@example.com",
  "password": "password123"
}
```

**Response (200)**
```json
{
  "status": "2fa_required",
  "token": "eyJ..."
}
```

### 2FA Verification
```http
POST /api/auth/2fa
Content-Type: application/json

{
  "code": "123456"
}
```

**Response (200)**
```json
{
  "status": "success"
}
```

### Finalize Session
```http
POST /api/auth/finalize
```

**Response (200)**
Sets session cookie and returns:
```json
{
  "status": "authenticated"
}
```

### Logout
```http
POST /api/auth/logout
```

**Response (200)**
Clears session cookie.

---

## Tenants

### List Tenants
```http
GET /api/tenants
Cookie: session=...
```

**Response (200)**
```json
[
  {
    "key": "tenant-uuid-1",
    "name": "Tenant Name 1"
  },
  {
    "key": "tenant-uuid-2",
    "name": "Tenant Name 2"
  }
]
```

---

## Reports

### Generate Report
```http
POST /api/report
Cookie: session=...
Content-Type: application/json

{
  "tenant_key": "tenant-uuid",
  "from_date": "2024-01-01",
  "to_date": "2024-01-31",
  "language": "es"
}
```

**Response (200)**
```json
{
  "html": "<!DOCTYPE html>..."
}
```

**Errors**
- `400`: Invalid parameters
- `401`: Not authenticated
- `403`: No access to tenant
- `500`: Report generation failed

---

## Error Responses

All error responses follow this format:
```json
{
  "error": "Error message description"
}
```

### Status Codes
| Code | Description |
|------|-------------|
| 200 | Success |
| 400 | Bad Request - Invalid parameters |
| 401 | Unauthorized - Not authenticated |
| 403 | Forbidden - No access |
| 500 | Internal Server Error |
