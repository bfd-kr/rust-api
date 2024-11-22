use serde::Serialize;

#[derive(Serialize)]
pub struct Error {
    pub(crate) code: String,
    pub(crate) message: String,
}

#[derive(Serialize)]
pub enum ApiResponse<T> {
    Success { data: T },
    Error { error: Error },
}
