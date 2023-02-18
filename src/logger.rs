use std::collections::HashMap;
use crate::{LoggerOperation, LoggingLevels};

#[derive(Eq, Debug, PartialEq, Default)]
pub struct Logger {
    pub service: String,
    pub meta: HashMap<String, String>,
}

impl Logger {
    /// Create new Logger
    ///
    /// ### Example
    /// ```
    /// use substreams_sink_winston::Logger;
    /// let mut logger = Logger::from("user-service");
    /// ```
    #[inline]
    #[must_use]
    pub fn from(service: &str) -> Self {
        Self {
            service: service.to_string(),
            meta: Default::default(),
        }
    }

    /// Set label to Counter
    /// Labels represents a collection of label name -> value mappings. 
    ///
    /// ### Example
    /// ```
    /// use std::collections::HashMap;
    /// use substreams_sink_winston::Logger;
    /// let mut logger = Logger::from("user-service");
    /// let meta = HashMap::from([("label1".to_string(), "value1".to_string())]);
    /// logger.with(meta);
    /// ```
    #[inline]
    pub fn with(mut self, meta: HashMap<String, String>) -> Self {
        self.meta = meta;
        self
    }

    /// Informational: informational messages
    ///
    /// ### Example
    /// ```
    /// use substreams_sink_winston::{Logger, LoggerOperations};
    /// let mut log_ops: LoggerOperations = Default::default();
    /// let mut logger = Logger::from("user-service");
    /// log_ops.push(logger.info("message"));
    /// ```
    #[inline]
    #[must_use]
    pub fn info(&mut self, message: &str ) -> LoggerOperation {
        LoggerOperation {
            message: message.to_string(),
            meta: self.meta.to_owned(),
            level: LoggingLevels::Info.into(),
            service: self.service.to_owned()
        }
    }
}