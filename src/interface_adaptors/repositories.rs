use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{MysqlConnection, RunQueryDsl};
use failure::Error;

use crate::domains::documents::Document;
use crate::interface_adaptors::databases::models::DocumentEntity;
use crate::repositories::documents::DocumentRepository;

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
    fn insert(&mut self, document: &Document) -> Result<(), Error> {
        use super::super::interface_adaptors::databases::schema::documents::dsl;

        let entity = DocumentEntity::from(document);
        let conn = self.pool.get().unwrap();
        diesel::insert_into(dsl::documents)
            .values(entity)
            .execute(&conn)?;

        Ok(())
    }
}
