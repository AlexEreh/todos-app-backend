use serde::Deserialize;

// The query parameters for todos index
#[derive(Debug, Deserialize, Default)]
pub struct Pagination {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}
