#[derive(Debug, PartialEq, Clone)]
pub enum Loc {
    File {
        filename: String,
        line: i32,
        pos: i32,
    },
    Unknown,
}
