---
name: data_migration_guard
description: Safely applies database migrations using SQLx.
version: 1.0.0
---

# Data Migration Guard

**Objective**: Ensure database schema changes are applied safely without data loss.

## Usage
Run this skill when you need to update the local or production database schema.

## Prerequisites
- `sqlx-cli`: `cargo install sqlx-cli`
- `DATABASE_URL`: Set in `.env`

## Procedure

### 1. Verification
Check if `sqlx` is installed:
```powershell
sqlx --version
```
If not installed, install it or warn the user.

### 2. Dry Run
Check pending migrations:
```powershell
sqlx migrate info
```

### 3. Application
Apply migrations:
```powershell
sqlx migrate run
```

### 4. Schema Update
If the migration was successful, update the offline schema file for SQLx (required for `query!` macro):
```powershell
cargo sqlx prepare --workspace
```
*Note: This usually requires the backend logic to be compilable.*

## Rollback
To undo the last migration:
```powershell
sqlx migrate revert
```
**CRITICAL**: Only revert if you are SURE the data loss is acceptable.
