use diesel::r2d2::{ConnectionManager, Pool};
use diesel::MysqlConnection;
use failure::Error;

use crate::domains::documents::{Document, DocumentId};
use crate::repositories::documents::DocumentRepository;

//
// Entity
//

pub struct DocumentEntity {
    pub id: String,
    pub title: String,
    pub body: String,
}

impl DocumentEntity {
    fn from(model: &Document) -> Self {
        Self {
            id: model.id.value.to_owned(),
            title: model.title.value.to_owned(),
            body: model.body.value.to_owned(),
        }
    }
}

pub struct DocumentRepositoryImplOnMySQL {
    pub pool: Box<Pool<ConnectionManager<MysqlConnection>>>,
}

impl DocumentRepository for DocumentRepositoryImplOnMySQL {
    fn insert(&mut self, document: &Document) -> Result<DocumentId, Error> {
        DocumentEntity::from(document);
        todo!()
    }
}
