use crate::{LoggerOperation, LoggerOperations};

impl LoggerOperations {
    pub fn push(&mut self, operation: LoggerOperation) {
        self.operations.push(operation);
    }

    pub fn extend(&mut self, operations: Vec<LoggerOperation>) {
        self.operations.extend(operations);
    }
}

#[cfg(test)]
mod tests {
    use crate::{Logger, LoggerOperations};

    #[test]
    fn test_push() {
        let mut log_ops: LoggerOperations = Default::default();
        let logger = Logger::new("user-service");
        log_ops.push(logger.info("message1"));
        log_ops.push(logger.info("message2"));

        assert_eq!(log_ops.operations.len(), 2);
    }

    #[test]
    fn test_extend() {
        let mut log_ops: LoggerOperations = Default::default();
        let logger = Logger::new("user-service");
        log_ops.extend(vec![logger.info("message1"), logger.info("message2")]);

        assert_eq!(log_ops.operations.len(), 2);
    }
}
