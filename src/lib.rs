#[derive(Debug)]
pub struct MagicString {
    original: String,
    modified: String,
    operations: Vec<Operation>,
}

#[derive(Debug)]
enum OperationType {
    INSERT,
    REPLACE,
    REMOVE,
}

#[derive(Debug)]
struct Operation {
    op_type: OperationType,
    start: usize,
    end: usize,
    text: Option<String>,
}

impl MagicString {
    // Create new magic string instance
    pub fn new(text: &str) -> Self {
        Self {
            original: text.to_string(),
            modified: text.to_string(),
            operations: vec![],
        }
    }

    // Insert string to specific index
    pub fn insert(&mut self, index: usize, text: &str) {
        self.operations.push(Operation {
            op_type: OperationType::INSERT,
            start: index,
            end: index,
            text: Some(text.to_string()),
        });
        // Insert the string
        self.modified.insert_str(index, text);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_magic_string() {
        let mut ms = MagicString::new("Hello, world!");

        ms.insert(7, "beautiful ");
        assert_eq!(ms.get_modified(), "Hello, beautiful world!");
    }
}
