use failure::Error;

use crate::domains::documents::Document;

pub trait DocumentRepository {
    fn insert(&self, document: &Document) -> Result<(), Error>;
}
