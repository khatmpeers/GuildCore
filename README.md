# guild-core

> Minimal state machine for request-based work coordination.

[![Crates.io](https://img.shields.io/crates/v/guild-core.svg)](https://crates.io/crates/guild-core)
[![Documentation](https://docs.rs/guild-core/badge.svg)](https://docs.rs/guild-core)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE-MIT)

## What is guild-core?

Guild-core provides the bare-bones infrastructure for coordinating request-based work:

1. Someone posts a request
2. Someone claims it
3. They complete it (or abandon it)
4. Rewards are distributed

Guild-core is domain-agnostic, meaning it can work for technical cooperatives, mutual aid networks, research groups, creative collectives, and anything else that features this dynamic.

## Quick Start

Add to your `Cargo.toml`:
```toml
[dependencies]
guild-core = "0.1.0"
tokio = { version = "1", features = ["full"] }
```

Basic example:
```rust
use guild_core::{init_board, RequestDraft, Rewardable};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

// Define your reward type with your preferred payment gateway provider
#[derive(Serialize, Deserialize)]
struct Payment {
    amount_usd: u64,
}

impl Rewardable for Payment {
    fn release_reward(&self) {
        println!("Releasing ${}", self.amount_usd);
        // Integrate with preferred payment gateway provider API
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize the request board, gives you the SqlitePool object
    let pool = init_board("sqlite:board.db").await?;

    // Retrieve these from wherever you're storing your client/user data
    let client_id: Uuid = ...;
    let user_id: Uuid = ...;
    
    // Client creates a request
    let draft = RequestDraft::new(
        "Build landing page",
        "Need a React landing page with responsive design",
        None, // labels
        Some(vec!["react"]), // tags
        client_id
    );
    
    // Publish with reward
    let request = draft.publish(
        Some(Payment { amount_usd: 500 }),
        &pool
    ).await?;

    ...
    
    // Member claims the request
    let accepted = request.claim(member_uuid, &pool).await?;
    
    // Member completes work
    accepted.complete(); // Triggers reward.release_reward()
    
    Ok(())
}
```

## Features

- `Rewardable` trait enables generalization; can be fitted to work with any form of compensation so long as the trait is implemented.
- SQLite integration for internal request tracking.

## Use Cases

Guild-core can be used in any request-based application. Some example applications are:
- Technical Work
- Research Assistance
- Mutual Aid
- Open Source
- Creative Work
- Skill Exchange

## How It Works

### The Request Lifecycle

The State Machine is as such.

```
(RequestStub) -> RequestDraft -(consumes `Rewardable`)-> Request -(consumes `member_id`)-> AcceptedRequest
```

The entry point to the state machine is the `RequestDraft`, which exposes a `new` method. Drafts can be published using the `publish` method, which will return a `Request` object and write the request to the `board.db` file for public viewing. Requests can be delisted, which consumes the object and deletes it from `board.db`. They can also be claimed using the `claim` method, which returns an `AcceptedRequest` which can either be abandoned or completed. Completion will automatically release the trait `Rewardable`. And by the same token, abandonment will return the request to `board.db`.

### The Rewardable Trait
```rust
pub trait Rewardable {
    fn release_reward(&self);
}
```

Implement this for your reward type. Examples:
```rust
// Monetary reward
struct StripePayment { payment_intent_id: String }

// Academic credit
struct ResearchCredit { course: String, credits: f32 }

// Time bank
struct TimeHours { hours: f32 }

// Portfolio credit (no money)
struct PortfolioCredit { project: String, role: String }
```

### The Request Board

Guild-core's database only stores currently available requests. RequestDrafts and AcceptedRequests will not show up. That is to say, requests are removed from the board once claimed. As such, you should:
- Store claimed/in-progress requests in a dedicated location
- Store completed requests in a dedicated location
- Track member history
- Handle payment processing

Or anything else you'd like persisted.

## Documentation

- [API Documentation](https://docs.rs/guild-core)
- [Examples](./examples)

## Design Philosophy

Guild-core is intentionally minimal.

**What is included:**
- State machine for request lifecycle
- Ephemeral storage for available requests
- Generic trait for any reward system

**What isn't:**
- User management
- Payment processing
- Reputation/ranking
- Historical records

This allows the state machine to remain applicable to various contexts. 

## Building a Guild

Since guild-core specifically handles this core dynamic, you are encouraged to create your own extensions of the dynamic to fully create your ideal "guild." 

See [The Lake](https://www.google.com) for a reference implementation.

## Examples

Check the [`examples/`](./examples) directory:

- `basic_usage.rs` - Simple request lifecycle

## Status

**Early Development**: Guild-core is in active development. The API may change before 1.0.0.

Current version: 0.1.0 (experimental)

## Contributing

Contributions welcome! Please:

1. Open an issue to discuss changes
2. Fork the repository
3. Create a feature branch
4. Submit a pull request

## Related Projects

- [The Lake](https://www.google.com) - Technical cooperation guild (reference implementation)
- [Guild Registry](https://www.google.com) - Discover guilds built on guild-core

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.