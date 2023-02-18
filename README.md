# [`Substreams`](https://substreams.streamingfast.io/) [Winston](https://github.com/winstonjs/winston) `Logger` sink module

[<img alt="github" src="https://img.shields.io/badge/Github-substreams.winston-8da0cb?style=for-the-badge&logo=github" height="20">](https://github.com/pinax-network/substreams-sink-winston)
[<img alt="crates.io" src="https://img.shields.io/crates/v/substreams-sink-winston.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/substreams-sink-winston)
[<img alt="npm" src="https://img.shields.io/npm/v/substreams-sink-winston.svg?style=for-the-badge&color=CB0001&logo=npm" height="20">](https://www.npmjs.com/package/substreams-sink-winston)
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
- [ ] **Substreams Telegram** sink module
- [ ] **Substreams Discord** sink module

## 🛠 Feature Roadmap

### [Logging](https://github.com/winstonjs/winston)

- [ ] error
- [ ] warn
- [ ] info
- [ ] http
- [ ] verbose
- [ ] debug
- [ ] silly

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
