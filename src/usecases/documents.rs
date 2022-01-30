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
    pub id: DocumentId,
}

impl PostDocumentOutput {
    pub fn new(id: DocumentId) -> Self {
        Self { id }
    }
}

pub fn post_document(
    repository: &mut impl DocumentRepository,
    input: &PostDocumentInput,
) -> Result<PostDocumentOutput, Error> {
    let document = Document::create(input.title.to_owned(), input.body.to_owned());
    let result = repository.insert(&document);
    result.map(|_| PostDocumentOutput::new(document.id))
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::collections::HashMap;

    use failure::Error;
    use ulid::Ulid;

    use crate::domains::documents::{Document, DocumentBody, DocumentId, DocumentTitle};
    use crate::repositories::documents::DocumentRepository;
    use crate::usecases::documents::{post_document, PostDocumentInput};

    pub struct DocumentRepositoryImplOnMemory {
        pool: RefCell<HashMap<DocumentId, Vec<Document>>>,
    }

    impl DocumentRepository for DocumentRepositoryImplOnMemory {
        fn insert(&self, document: &Document) -> Result<(), Error> {
            let _ = &self
                .pool
                .borrow_mut()
                .entry(document.id.clone())
                .or_insert_with(|| vec![])
                .push(document.clone());
            Ok(())
        }
    }

    #[test]
    fn success_post_document() {
        let mut repository = DocumentRepositoryImplOnMemory {
            pool: RefCell::new(HashMap::new()),
        };
        let input = PostDocumentInput::new(
            DocumentTitle::new(String::from("sample title")),
            DocumentBody::new(String::from("sample body")),
        );
        let result = post_document(&mut repository, &input);
        assert!(result.is_ok());
        assert!(Ulid::from_string(&result.unwrap().id.value).is_ok());
    }
}
