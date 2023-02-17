#![allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SourceInfo {
    line: usize,
    col: usize,
}

impl Default for SourceInfo {
    fn default() -> Self {
        SourceInfo { line: 0, col: 0 }
    }
}
