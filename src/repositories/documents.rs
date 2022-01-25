use failure::Error;

use crate::domains::documents::{Document, DocumentId};

pub trait DocumentRepository {
    fn insert(&mut self, document: &Document) -> Result<DocumentId, Error>;
}
