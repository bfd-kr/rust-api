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

pub(crate) struct ApiResponse2<T> {
    pub(crate) data: Option<T>,
    pub(crate) error: Option<Error>,
}
