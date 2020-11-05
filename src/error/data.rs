#[derive(Debug)]
pub enum IncorrectData {
    Size { expected: usize, got: usize },
    Overflow,
}

impl std::error::Error for IncorrectData {}

impl std::fmt::Display for IncorrectData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            IncorrectData::Size {
                ref expected,
                ref got,
            } => write!(f, "wrong data size, expected {} got {}", expected, got),

            IncorrectData::Overflow => {
                write!(f, "operation was not performed, as it would overflow")
            }
        }
    }
}
