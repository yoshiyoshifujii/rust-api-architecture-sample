use serde::Deserialize;

use crate::domains::documents::{DocumentBody, DocumentTitle};
use crate::usecases::documents::PostDocumentInput;

#[derive(Deserialize)]
pub struct PostDocumentRequest {
    title: String,
    body: String,
}

impl PostDocumentRequest {
    pub fn of(&self) -> PostDocumentInput {
        let title = DocumentTitle::new(self.title.to_owned());
        let body = DocumentBody::new(self.body.to_owned());
        PostDocumentInput::new(title, body)
    }
}
