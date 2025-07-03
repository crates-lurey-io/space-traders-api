# space-traders-api

Generated API client for [SpaceTraders](https://spacetraders.io).

[![Test](https://github.com/crates-lurey-io/space-traders-api/actions/workflows/test.yml/badge.svg)](https://github.com/crates-lurey-io/space-traders-api/actions/workflows/test.yml)
[![Crates.io Version](https://img.shields.io/crates/v/space-traders-api)](https://crates.io/crates/space-traders-api)

## Usage

```rust
use space_traders_api::apis::{configuration::Configuration, global_api};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let mut config = Configuration::new();
  
   if let Ok(token) = std::env::var("SPACE_TRADERS_AGENT_TOKEN") {
       config.bearer_access_token = Some(token);
   }

   let status = global_api::get_status(&config).await?;
   println!("Status: {}", status.status);
}
```

See [`examples/main.rs`](examples/main.rs) for an example to get started.

```sh
cargo run --example main
```

To provide an agent token to the example, set `SPACE_TRADERS_AGENT_TOKEN`:

```sh
export SPACE_TRADERS_AGENT_TOKEN=...
cargo run --example main
```

## Contributing

This project uses [`just`][] to run commands the same way as the CI:

- `cargo just check` to check formatting and lints.
- `cargo just doc` to generate and preview docs.

[`just`]: https://crates.io/crates/just

For a full list of commands, see the [`Justfile`](./Justfile).
