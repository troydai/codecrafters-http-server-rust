#[derive(Debug)]
pub enum HttpBody {
    Empty,
    Content(Vec<u8>),
}

impl HttpBody {
    pub const fn len(&self) -> usize {
        match self {
            Self::Empty => 0,
            Self::Content(bytes) => bytes.len(),
        }
    }
}
