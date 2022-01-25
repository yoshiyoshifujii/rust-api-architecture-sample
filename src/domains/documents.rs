use ulid::Ulid;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct DocumentId {
    pub value: String,
}

impl DocumentId {
    pub fn new() -> Self {
        let id = Ulid::new();
        Self {
            value: id.to_string(),
        }
    }
}

#[cfg(test)]
mod document_id_tests {
    use ulid::Ulid;

    use crate::domains::documents::DocumentId;

    #[test]
    fn success() {
        let id = DocumentId::new();
        let actual = Ulid::from_string(&id.value);
        assert!(actual.is_ok());
    }
}

#[derive(Debug, Clone)]
pub struct DocumentTitle {
    value: String,
}

impl DocumentTitle {
    pub fn new(value: String) -> Self {
        if value.is_empty() {
            panic!("Document title is not empty.")
        }
        if value.len() > 120 {
            panic!("Document title length is over 120.")
        }
        Self { value }
    }
}

#[cfg(test)]
mod document_title_tests {
    use std::panic;

    use crate::domains::documents::DocumentTitle;

    #[test]
    fn success_document_title_new() {
        let value = String::from("new title");
        let actual = DocumentTitle::new(value.clone());
        assert_eq!(actual.value, value);
    }

    #[test]
    fn panic_document_title_input_to_empty_string() {
        let value = String::from("");
        let actual = panic::catch_unwind(|| DocumentTitle::new(value));
        assert!(actual.is_err());
    }

    #[test]
    fn panic_document_title_input_to_over_length() {
        let value = String::from("0123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890");
        let actual = panic::catch_unwind(|| DocumentTitle::new(value));
        assert!(actual.is_err());
    }
}

#[derive(Debug, Clone)]
pub struct DocumentBody {
    value: String,
}

impl DocumentBody {
    pub fn new(value: String) -> Self {
        if value.len() > 1_000 {
            panic!("Document title length is over 1,000.")
        }
        Self { value }
    }
}

#[cfg(test)]
mod document_body_tests {
    use crate::domains::documents::DocumentBody;

    #[test]
    fn success() {
        let value = String::from("sample body");
        let body = DocumentBody::new(value.clone());
        assert_eq!(body.value, value);
    }
}

#[derive(Debug, Clone)]
pub struct Document {
    pub id: DocumentId,
    pub title: DocumentTitle,
    pub body: DocumentBody,
}

impl Document {
    pub fn create(id: DocumentId, title: DocumentTitle, body: DocumentBody) -> Self {
        Self { id, title, body }
    }
}
