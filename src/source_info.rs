use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SourceInfo {
    line: usize,
    col: usize,
}
