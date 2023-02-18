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
    use crate::{LoggerOperation, LoggerOperations};

    #[test]
    fn test_push() {
        let mut log_ops: LoggerOperations = Default::default();
        let mut log = LoggerOperation::from("custom_log");
        log_ops.push(log.inc());
        log_ops.push(log.inc());

        assert_eq!(log_ops.operations.len(), 2);
    }

    #[test]
    fn test_extend() {
        let mut log_ops: LoggerOperations = Default::default();
        let mut log = LoggerOperation::from("custom_log");
        log_ops.extend(vec![log.inc(), log.inc()]);

        assert_eq!(log_ops.operations.len(), 2);
    }
}
