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

#[cfg(test)]
mod tests {}
