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
    /// Create new magic string instance
    pub fn new(text: &str) -> Self {
        Self {
            original: text.to_string(),
            modified: text.to_string(),
            operations: vec![],
        }
    }

    /// Insert string to specific index
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

    /// Replace the string within specific index
    pub fn replace(&mut self, start: usize, end: usize, text: &str) {
        self.operations.push(Operation {
            op_type: OperationType::REPLACE,
            start,
            end,
            text: Some(text.to_string()),
        });
        // Update the string within specific index
        self.modified.replace_range(start..end, text);
    }

    /// Remove the string within specific index
    pub fn remove(&mut self, start: usize, end: usize) {
        self.operations.push(Operation {
            op_type: OperationType::REMOVE,
            start,
            end,
            text: None,
        });
        // Remove the string within specific index
        self.modified.replace_range(start..end, "");
    }

    pub fn prepend(&mut self, text: &str) -> &mut Self {
        self.operations.push(Operation {
            op_type: OperationType::INSERT,
            start: 0,
            end: 0,
            text: Some(text.to_string()),
        });
        // Remove the string within specific index
        self.modified.insert_str(0, text);
        self
    }

    pub fn append(&mut self, text: &str) -> &mut Self {
        let len = self.modified.len();
        self.operations.push(Operation {
            op_type: OperationType::INSERT, // Treat append as insert at the end
            start: len,
            end: len,
            text: Some(text.to_string()),
        });
        self.modified.push_str(text); // Append text at the end of the modified string
        self // Return mutable reference to `self` for chaining
    }

    /// Get the last char
    pub fn last_char(&self) -> char {
        self.modified.chars().last().unwrap()
    }

    /// Get the modified string
    pub fn get_modified(&self) -> &str {
        &self.modified
    }

    /// Get the modified string value
    pub fn to_string(&self) -> String {
        self.modified.clone()
    }

    /// Get the original string
    pub fn get_original(&self) -> &str {
        &self.original
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

        ms.replace(0, 5, "Hi");
        assert_eq!(ms.get_modified(), "Hi, beautiful world!");

        ms.remove(19, 20); // Remove exclamation
        assert_eq!(ms.get_modified(), "Hi, beautiful world");

        assert_eq!(ms.last_char(), 'd');
        assert_eq!(ms.to_string(), "Hi, beautiful world");
        ms.prepend("Hi").append(" WORLD");
        assert_eq!(ms.get_modified(), "HiHi, beautiful world WORLD");
    }
}
