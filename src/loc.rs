#[derive(Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub struct Loc {
    pub filename: String,
    pub pos: i32,
    pub line: i32,
}
