#[derive(Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub struct Loc {
    pub filename: String,
    pub line: i32,
    pub pos: i32,
}

impl Loc {
    pub fn empty() -> Self {
        Loc {
            filename: "".to_string(),
            pos: 0,
            line: 0,
        }
    }
}
