use failure::Error;

use crate::domains::documents::Document;

pub trait DocumentRepository {
    fn insert(&mut self, document: &Document) -> Result<(), Error>;
}
