#[derive(Debug)]
pub struct SqlSearchText {
    search_text: Option<String>,
    pub is_exact: bool,
}

impl SqlSearchText {
    pub fn new(search_text: &str, is_exact: bool) -> Self {
        let search_text = "%".to_string() + &search_text.replace('*', "%") + "%";
        Self {
            search_text: Some(search_text),
            is_exact,
        }
    }

    pub fn empty() -> Self {
        Self {
            search_text: None,
            is_exact: false,
        }
    }

    pub fn is_some(&self) -> bool {
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
