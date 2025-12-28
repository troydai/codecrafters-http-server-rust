#[derive(Debug)]
pub enum HttpBody {
    Empty,
    #[allow(dead_code)]
    Content(Vec<u8>),
}