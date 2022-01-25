use failure::Error;

use crate::domains::documents::{Document, DocumentBody, DocumentId, DocumentTitle};
use crate::repositories::documents::DocumentRepository;

#[derive(Debug, Clone)]
pub struct PostDocumentInput {
    title: DocumentTitle,
    body: DocumentBody,
}

impl PostDocumentInput {
    pub fn new(title: DocumentTitle, body: DocumentBody) -> Self {
        Self { title, body }
    }
}

#[derive(Debug, Clone)]
pub struct PostDocumentOutput {
    id: DocumentId,
}

impl PostDocumentOutput {
    pub fn new(id: DocumentId) -> Self {
        Self { id }
    }
}

pub fn post_document(
    repository: &mut impl DocumentRepository,
    input: PostDocumentInput,
) -> Result<PostDocumentOutput, Error> {
    let document = Document::create(DocumentId::new(), input.title, input.body);
    let result = repository.insert(&document);
    result.map(|id| PostDocumentOutput::new(id))
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use failure::Error;
    use ulid::Ulid;

    use crate::domains::documents::{Document, DocumentBody, DocumentId, DocumentTitle};
    use crate::repositories::documents::DocumentRepository;
    use crate::usecases::documents::{post_document, PostDocumentInput};

    pub struct DocumentRepositoryImplOnMemory {
        pub pool: HashMap<DocumentId, Vec<Document>>,
    }

    impl DocumentRepository for DocumentRepositoryImplOnMemory {
        fn insert(&mut self, document: &Document) -> Result<DocumentId, Error> {
            let _ = &self
                .pool
                .entry(document.id.clone())
                .or_insert_with(|| vec![])
                .push(document.clone());
            Ok(document.id.clone())
        }
    }

    #[test]
    fn success_post_document() {
        let mut repository = DocumentRepositoryImplOnMemory {
            pool: HashMap::new(),
        };
        let input = PostDocumentInput::new(
            DocumentTitle::new(String::from("sample title")),
            DocumentBody::new(String::from("sample body")),
        );
        let result = post_document(&mut repository, input);
        assert!(result.is_ok());
        assert!(Ulid::from_string(&result.unwrap().id.value).is_ok());
    }
}
