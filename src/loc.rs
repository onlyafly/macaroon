#[derive(Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub enum Loc {
    File {
        filename: String,
        line: i32,
        pos: i32,
    },
    Unknown,
}
