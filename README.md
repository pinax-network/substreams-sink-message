# [`Substreams`](https://substreams.streamingfast.io/) [Winston](https://github.com/winstonjs/winston) `Logger` sink module

[<img alt="github" src="https://img.shields.io/badge/Github-substreams.winston-8da0cb?style=for-the-badge&logo=github" height="20">](https://github.com/pinax-network/substreams-sink-winston)
[<img alt="crates.io" src="https://img.shields.io/crates/v/substreams-sink-winston.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/substreams-sink-winston)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-substreams.winston-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/substreams-sink-winston)
[<img alt="GitHub Workflow Status" src="https://img.shields.io/github/actions/workflow/status/pinax-network/substreams-sink-winston/ci.yml?branch=main&style=for-the-badge" height="20">](https://github.com/pinax-network/substreams-sink-winston/actions?query=branch%3Amain)

> `substreams-sink-winston` is a tool that allows developers to pipe data extracted metrics from a blockchain into a standard Winston Logging message conforming to the severity ordering specified by [RFC5424](https://tools.ietf.org/html/rfc5424).

## 📖 Documentation

### https://docs.rs/substreams-sink-winston

### Further resources

- [Substreams documentation](https://substreams.streamingfast.io)
- [Winston documentation](https://github.com/winstonjs/winston)

## Related Sinks

- [ ] **Substreams GoogleSheet** sink module
- [ ] **Substreams CSV** sink module
- [ ] **Substreams Telegram** sink module
- [ ] **Substreams Discord** sink module

## 🛠 Feature Roadmap

### Create Logger
- [x] service
- [ ] defaultMeta

### Logging
- [x] **Emergency**: system is unusable
- [x] **Alert**: action must be taken immediately
- [x] **Critical**: critical conditions
- [x] **Error**: error conditions
- [x] **Warning**: warning conditions
- [x] **Notice**: normal but significant condition
- [x] **Informational**: informational messages
- [x] **Debug**: debug-level messages

### Filtering info Objects
- [ ] ~ignorePrivate~
- [ ] ~private~

## Install

```bash
$ cargo add substreams-sink-winston
```

## Quickstart

**Cargo.toml**

```toml
[dependencies]
substreams = "0.5"
substreams-sink-winston = "0.1"
```

**src/lib.rs**

```rust
use std::collections::HashMap;
use substreams::errors::Error;
use substreams_sink_winston::{Logger, LoggerOperations};

#[substreams::handlers::map]
fn prom_out(
    ... some stores ...
) -> Result<LoggerOperations, Error> {
    // Initialize Winston Logger operations container
    let mut log_ops: LoggerOperations = Default::default();

    // Create Logger
    // ==============
    let mut logger = Logger::from("user-service");

    // Informational: informational messages
    log_ops.push(logger.info("info message"));

    // Error: error conditions
    log_ops.push(logger.error("error message"));

    // Create a HashMap of metadata
    let meta = HashMap::from([("label1".to_string(), "value1".to_string())]);
    log_ops.push(logger.info("message").with(meta));

    Ok(log_ops)
}
```
