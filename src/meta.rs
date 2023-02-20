use crate::LoggerOperation;
use std::{collections::HashMap, convert::From};

#[derive(Eq, Debug, Clone, PartialEq, Default)]
pub struct Meta(HashMap<String, String>);

impl Meta {
    pub fn new() -> Self {
        Meta(HashMap::default())
    }
    pub fn insert(&mut self, k: &str, v: &str) {
        self.0.insert(k.to_string(), v.to_string());
    }
    pub fn push(&mut self, value: &str) {
        self.0.insert(self.0.len().to_string(), value.to_string());
    }
}

impl From<Vec<&str>> for Meta {
    #[inline]
    #[must_use]
    fn from(items: Vec<&str>) -> Self {
        let mut object: HashMap<String, String> = Default::default();
        for (key, value) in items.into_iter().enumerate() {
            object.insert(key.to_string(), value.to_string());
        }
        Meta(object)
    }
}

impl From<Vec<[&str; 2]>> for Meta {
    #[inline]
    #[must_use]
    fn from(items: Vec<[&str; 2]>) -> Self {
        let mut object: HashMap<String, String> = Default::default();
        for item in items {
            if item.len() != 2 {
                continue;
            }
            let k = item[0];
            let v = item[1];
            object.insert(k.to_string(), v.to_string());
        }
        Meta(object)
    }
}

impl From<Meta> for HashMap<String, String> {
    #[inline]
    #[must_use]
    fn from(meta: Meta) -> HashMap<String, String> {
        meta.0
    }
}

impl From<HashMap<String, String>> for Meta {
    #[inline]
    #[must_use]
    fn from(object: HashMap<String, String>) -> Self {
        Meta(object)
    }
}

impl LoggerOperation {
    /// Set label to Counter
    /// Labels represents a collection of label name -> value mappings.
    ///
    /// ### Example
    /// ```
    /// use substreams_sink_winston::{Logger, Meta};
    /// let mut logger = Logger::new("user-service");
    /// let meta = Meta::from(vec!(["key", "value"]));
    /// logger.info("message").with(meta);
    /// ```
    #[inline]
    pub fn with(self, meta: Meta) -> Self {
        LoggerOperation {
            level: self.level,
            message: self.message,
            meta: meta.into(),
            service: self.service,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Logger, Meta};
    use std::collections::HashMap;

    #[test]
    fn test_meta_array() {
        let logger = Logger::new("user-service");
        let items = vec!["value"];
        let meta = Meta::from(items);

        logger.info("info message").with(meta);
    }

    #[test]
    fn test_meta_hashmap() {
        let logger = Logger::new("user-service");
        let items = HashMap::from([("key".to_string(), "value".to_string())]);
        let meta = Meta::from(items);

        logger.info("info message").with(meta);
    }

    #[test]
    fn test_meta_array_object() {
        let logger = Logger::new("user-service");
        let meta = Meta::from(vec![["key", "value"]]);

        logger.info("info message").with(meta);
    }

    #[test]
    fn test_meta_insert() {
        let logger = Logger::new("user-service");
        let mut meta = Meta::new();
        meta.insert("key", "value");

        logger.info("info message").with(meta);
    }

    #[test]
    fn test_meta_push() {
        let logger = Logger::new("user-service");
        let mut meta = Meta::new();
        meta.push("value");

        logger.info("info message").with(meta);
    }
}
