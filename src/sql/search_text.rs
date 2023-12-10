#[derive(Debug)]
pub(crate) struct SqlSearchText {
    text: Option<String>,
    pub(crate) is_exact: bool,
}

impl SqlSearchText {
    pub(crate) fn new(search_text: &str, is_exact: bool) -> Self {
        Self {
            text: Some(search_text.to_string()),
            is_exact,
        }
    }

    pub(crate) fn empty() -> Self {
        Self {
            text: None,
            is_exact: false,
        }
    }

    pub(crate) fn is_some(&self) -> bool {
        self.text.is_some()
    }

    pub(crate) fn exact_text(&self) -> String {
        match &self.text {
            Some(text) => text.to_string(),
            None => "".to_string(),
        }
    }

    pub(crate) fn search_pattern(&self) -> String {
        match &self.text {
            Some(text) => "%".to_string() + &text.replace('*', "%") + "%",
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

#[derive(Debug)]
pub struct HistoryItemSearchParams {
    pub(crate) year: Option<i32>,
    pub(crate) day: Option<i32>,
}

impl HistoryItemSearchParams {
    pub fn new(year: Option<i32>, day: Option<i32>) -> Self {
        Self { year, day }
    }

    pub fn empty() -> Self {
        Self {
            year: None,
            day: None,
        }
    }
}
