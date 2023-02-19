//! ### Example
//! ```
//! use std::collections::HashMap;
//! use substreams_sink_winston::{Logger, LoggerOperations};
//!
//! // Initialize Winston Logger operations container
//! let mut log_ops: LoggerOperations = Default::default();
//!
//! // Create Logger
//! // ==============
//! let mut logger = Logger::new("user-service");
//!
//! // Informational: informational messages
//! log_ops.push(logger.info("message"));
//!
//! // Error: error conditions
//! log_ops.push(logger.error("error message"));
//!
//! // Create a HashMap of metadata
//! let meta = HashMap::from([("label1".to_string(), "value1".to_string())]);
//! log_ops.push(logger.info("message").with(meta));
//! ```
#[path = "pb/pinax.substreams.sink.winston.v1.rs"]
#[allow(dead_code)]
pub mod pb;
pub use self::pb::*;

mod helpers;
mod logger;
pub use self::logger::*;
