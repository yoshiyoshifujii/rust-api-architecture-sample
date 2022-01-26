use crate::domains::documents::DocumentId;
use serde::Serialize;

#[derive(Serialize)]
pub struct DocumentIdDto {
    id: String,
}

impl DocumentIdDto {
    pub fn new(id: &DocumentId) -> DocumentIdDto {
        DocumentIdDto {
            id: id.value.to_owned(),
        }
    }
}
