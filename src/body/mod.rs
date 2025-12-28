#[derive(Debug)]
pub enum HttpBody {
    Empty,
    Content(Vec<u8>),
}
