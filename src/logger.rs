use crate::{LoggerOperation, LoggingLevels};
use std::collections::HashMap;

#[derive(Eq, Debug, PartialEq, Default)]
pub struct Logger {
    pub service: String,
}

impl LoggerOperation {
    /// Set label to Counter
    /// Labels represents a collection of label name -> value mappings.
    ///
    /// ### Example
    /// ```
    /// use std::collections::HashMap;
    /// use substreams_sink_winston::Logger;
    /// let mut logger = Logger::new("user-service");
    /// let meta = HashMap::from([("label1".to_string(), "value1".to_string())]);
    /// logger.info("message").with(meta);
    /// ```
    #[inline]
    pub fn with(self, meta: HashMap<String, String>) -> Self {
        LoggerOperation {
            message: self.message,
            meta,
            level: self.level,
            service: self.service,
        }
    }
}

impl Logger {
    /// Create new Logger
    ///
    /// ### Example
    /// ```
    /// use substreams_sink_winston::Logger;
    /// let mut logger = Logger::new("user-service");
    /// ```
    #[inline]
    #[must_use]
    pub fn new(service: &str) -> Self {
        Self {
            service: service.to_string(),
        }
    }

    /// Emergency: system is unusable
    ///
    /// ### Example
    /// ```
    /// use substreams_sink_winston::{Logger, LoggerOperations};
    /// let mut log_ops: LoggerOperations = Default::default();
    /// let mut logger = Logger::new("user-service");
    /// log_ops.push(logger.emerg("emergy message"));
    /// ```
    #[inline]
    #[must_use]
    pub fn emerg(&mut self, message: &str) -> LoggerOperation {
        LoggerOperation {
            message: message.to_string(),
            meta: Default::default(),
            level: LoggingLevels::Emerg.into(),
            service: self.service.to_owned(),
        }
    }
    /// Alert: action must be taken immediately
    ///
    /// ### Example
    /// ```
    /// use substreams_sink_winston::{Logger, LoggerOperations};
    /// let mut log_ops: LoggerOperations = Default::default();
    /// let mut logger = Logger::new("user-service");
    /// log_ops.push(logger.alert("alert message"));
    /// ```
    #[inline]
    #[must_use]
    pub fn alert(&mut self, message: &str) -> LoggerOperation {
        LoggerOperation {
            message: message.to_string(),
            meta: Default::default(),
            level: LoggingLevels::Alert.into(),
            service: self.service.to_owned(),
        }
    }
    /// Critical: critical conditions
    ///
    /// ### Example
    /// ```
    /// use substreams_sink_winston::{Logger, LoggerOperations};
    /// let mut log_ops: LoggerOperations = Default::default();
    /// let mut logger = Logger::new("user-service");
    /// log_ops.push(logger.crit("message"));
    /// ```
    #[inline]
    #[must_use]
    pub fn crit(&mut self, message: &str) -> LoggerOperation {
        LoggerOperation {
            message: message.to_string(),
            meta: Default::default(),
            level: LoggingLevels::Crit.into(),
            service: self.service.to_owned(),
        }
    }

    /// Error: error conditions
    ///
    /// ### Example
    /// ```
    /// use substreams_sink_winston::{Logger, LoggerOperations};
    /// let mut log_ops: LoggerOperations = Default::default();
    /// let mut logger = Logger::new("user-service");
    /// log_ops.push(logger.error("message"));
    /// ```
    #[inline]
    #[must_use]
    pub fn error(&mut self, message: &str) -> LoggerOperation {
        LoggerOperation {
            message: message.to_string(),
            meta: Default::default(),
            level: LoggingLevels::Error.into(),
            service: self.service.to_owned(),
        }
    }

    /// Warning: warning conditions
    ///
    /// ### Example
    /// ```
    /// use substreams_sink_winston::{Logger, LoggerOperations};
    /// let mut log_ops: LoggerOperations = Default::default();
    /// let mut logger = Logger::new("user-service");
    /// log_ops.push(logger.warning("message"));
    /// ```
    #[inline]
    #[must_use]
    pub fn warning(&mut self, message: &str) -> LoggerOperation {
        LoggerOperation {
            message: message.to_string(),
            meta: Default::default(),
            level: LoggingLevels::Warning.into(),
            service: self.service.to_owned(),
        }
    }

    /// Notice: normal but significant condition
    ///
    /// ### Example
    /// ```
    /// use substreams_sink_winston::{Logger, LoggerOperations};
    /// let mut log_ops: LoggerOperations = Default::default();
    /// let mut logger = Logger::new("user-service");
    /// log_ops.push(logger.notice("message"));
    /// ```
    #[inline]
    #[must_use]
    pub fn notice(&mut self, message: &str) -> LoggerOperation {
        LoggerOperation {
            message: message.to_string(),
            meta: Default::default(),
            level: LoggingLevels::Notice.into(),
            service: self.service.to_owned(),
        }
    }

    /// Informational: informational messages
    ///
    /// ### Example
    /// ```
    /// use substreams_sink_winston::{Logger, LoggerOperations};
    /// let mut log_ops: LoggerOperations = Default::default();
    /// let mut logger = Logger::new("user-service");
    /// log_ops.push(logger.info("message"));
    /// ```
    #[inline]
    #[must_use]
    pub fn info(&self, message: &str) -> LoggerOperation {
        LoggerOperation {
            message: message.to_string(),
            meta: Default::default(),
            level: LoggingLevels::Info.into(),
            service: self.service.to_owned(),
        }
    }

    /// Debug: debug-level messages
    ///
    /// ### Example
    /// ```
    /// use substreams_sink_winston::{Logger, LoggerOperations};
    /// let mut log_ops: LoggerOperations = Default::default();
    /// let mut logger = Logger::new("user-service");
    /// log_ops.push(logger.debug("message"));
    /// ```
    #[inline]
    #[must_use]
    pub fn debug(&mut self, message: &str) -> LoggerOperation {
        LoggerOperation {
            message: message.to_string(),
            meta: Default::default(),
            level: LoggingLevels::Debug.into(),
            service: self.service.to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::LoggerOperations;
    use std::collections::HashMap;

    #[test]
    fn test_counter() {
        let mut log_ops: LoggerOperations = Default::default();
        let mut logger = Logger::new("user-service");
        let meta = HashMap::from([("label1".to_string(), "value1".to_string())]);

        log_ops.push(logger.info("info message"));
        log_ops.push(logger.info("message with meta").with(meta));
        log_ops.push(logger.warning("warning"));
        assert_eq!(log_ops.operations.len(), 3);
    }
}
