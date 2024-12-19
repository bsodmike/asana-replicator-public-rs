# asana-replicator-public-rs

This is a quick POC code-base, shared to illustrate my experience with Axum. Many rough edges included due to being a speed run.

Refer to https://github.com/bsodmike/rv8803-rs for a professional example of my work, albeit on an embedded context.

## TODO:

- [ ] Move the `bin/create_webhook` example into an integration test with tear-down functionality as well.
- [ ] Add test driving utility code into its own module.

## Get started

Build as follows:

```
cargo b --bin create_webhook

# Launch Axum server
cargo r --bin create_webhook
```

## Creating a Webhook

Example response:

```
[src/asana.rs:213:9] &url = "https://app.asana.com/api/1.0/webhooks?resource=REDACTED&target=https://REDACTED.ngrok-free.app/receive-webhook/2&"
[src/bin/create_webhook.rs:54:9] resp = Object {
    "data": Object {
        "gid": String("REDACTED"),
        "resource_type": String("webhook"),
        "is_workspace_webhook": Bool(false),
        "last_success_at": String("2024-10-24T04:14:33.531Z"),
        "last_failure_content": String(""),
        "last_failure_at": Null,
        "created_at": String("2024-10-24T04:14:32.793Z"),
        "target": String("https://REDACTED.ngrok-free.app/receive-webhook/2"),
        "next_attempt_after": Null,
        "active": Bool(true),
        "failure_deletion_timestamp": Null,
        "delivery_retry_count": Number(0),
        "filters": Array [],
        "resource": Object {
            "gid": String("REDACTED"),
            "resource_type": String("project"),
            "name": String("project-1"),
        },
    },
    "X-Hook-Secret": String("REDACTED"),
}
```

## Minimum supported Rust version (MSRV)

This project is tested against rust `beta`.

## License

Licensed under either of [Apache License Version 2.0](./LICENSE-APACHE) or [The MIT License](./LICENSE-MIT) at your option.

🦀 ノ( º \_ º ノ) - respect crables!

## Copyright

Copyright © 2024, [Michael de Silva](mailto:michael@cyberdynea.io)