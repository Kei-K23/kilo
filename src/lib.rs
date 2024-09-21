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
    string: Option<String>,
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
}

#[cfg(test)]
mod tests {}
