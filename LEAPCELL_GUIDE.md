# 🚀 Deploying to Leapcell

This guide explains how to deploy the `axur-backend` to Leapcell using their native Rust builder.

## 1. Create a Service
1.  Go to the [Leapcell Dashboard](https://leapcell.io/dashboard).
2.  Click **New Service**.
3.  Connect your GitHub account and select the `axur-web` repository.

## 2. Configuration & Build
Configure the service with the following settings:

*   **Language**: Rust
*   **Build Command**: 
    ```bash
    cargo build --release --bin axur-backend
    ```
*   **Start Command**:
    ```bash
    ./target/release/axur-backend
    ```
*   **Port**: `3001` (or leave as default if Leapcell assigns one, the app now adapts)

## 3. Environment Variables
Add the following environment variables in the Leapcell dashboard:

| Key | Value | Description |
| :--- | :--- | :--- |
| `RUST_LOG` | `axur_backend=info,tower_http=info` | Logging level |
| `DATABASE_URL` | *Your Supabase/Postgres URL* | Database connection string |
| `GITHUB_TOKEN` | *Your GitHub Token* | For creating issues/feedback |
| `AXUR_API_TOKEN` | *Your Axur Token* | For fetching reports |

> [!NOTE]
> Leapcell will automatically inject a `PORT` environment variable. The application has been updated to listen on this port automatically.
