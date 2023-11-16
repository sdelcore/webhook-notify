# webhook-notify
Simple Rust application for triggering a webhook

## Prerequisites

- Rust programming language
- Cargo, Rust's package manager

## Environment Variables

This application requires the following environment variable:

- `WEBHOOK_URL`: The URL of the webhook you want to trigger.

## Running the Application

```sh
export WEBHOOK_URL=<your_webhook_url>
cargo build
cargo run <title> <message>
```
