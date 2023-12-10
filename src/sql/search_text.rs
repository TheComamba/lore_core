#[derive(Debug)]
pub(crate) struct SqlSearchText {
    search_text: Option<String>,
    pub(crate) is_exact: bool,
}

impl SqlSearchText {
    pub(crate) fn new(search_text: &str, is_exact: bool) -> Self {
        let search_text = "%".to_string() + &search_text.replace('*', "%") + "%";
        Self {
            search_text: Some(search_text),
            is_exact,
        }
    }

    pub(crate) fn empty() -> Self {
        Self {
            search_text: None,
            is_exact: false,
        }
    }

    pub(crate) fn is_some(&self) -> bool {
        self.search_text.is_some()
    }
}

impl ToString for SqlSearchText {
    fn to_string(&self) -> String {
        match &self.search_text {
            Some(search_text) => search_text.clone(),
            None => "%".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct EntityColumnSearchParams {
    pub(crate) label: SqlSearchText,
    pub(crate) descriptor: SqlSearchText,
}

impl EntityColumnSearchParams {
    pub fn new(label: Option<(&str, bool)>, descriptor: Option<(&str, bool)>) -> Self {
        let label = if let Some(label) = label {
            SqlSearchText::new(label.0, label.1)
        } else {
            SqlSearchText::empty()
        };
        let descriptor = if let Some(descriptor) = descriptor {
            SqlSearchText::new(descriptor.0, descriptor.1)
        } else {
            SqlSearchText::empty()
        };
        Self { label, descriptor }
    }

    pub fn empty() -> Self {
        Self {
            label: SqlSearchText::empty(),
            descriptor: SqlSearchText::empty(),
        }
    }
}
