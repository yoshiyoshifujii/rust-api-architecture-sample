use super::super::databases::schema::*;

#[derive(Insertable)]
#[table_name = "documents"]
pub struct DocumentEntity {
    pub id: String,
    pub title: String,
    pub body: String,
}
