#[derive(Debug)]
pub struct SqlSearchText {
    text: Option<String>,
    pub(crate) is_exact: bool,
}

impl SqlSearchText {
    pub fn exact(search_text: &str) -> Self {
        Self {
            text: Some(search_text.to_string()),
            is_exact: true,
        }
    }

    pub fn partial(search_text: &str) -> Self {
        Self {
            text: Some(search_text.to_string()),
            is_exact: false,
        }
    }

    pub fn empty() -> Self {
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
    pub fn new(label: Option<SqlSearchText>, descriptor: Option<SqlSearchText>) -> Self {
        let label = match label {
            Some(label) => label,
            None => SqlSearchText::empty(),
        };
        let descriptor = match descriptor {
            Some(descriptor) => descriptor,
            None => SqlSearchText::empty(),
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
    pub(crate) timestamp: Option<i64>,
    pub(crate) content: SqlSearchText,
}

impl HistoryItemSearchParams {
    pub fn new(
        year: Option<i32>,
        day: Option<i32>,
        timestamp: Option<i64>,
        content: Option<SqlSearchText>,
    ) -> Self {
        let content = match content {
            Some(content) => content,
            None => SqlSearchText::empty(),
        };
        Self {
            year,
            day,
            timestamp,
            content,
        }
    }

    pub fn empty() -> Self {
        Self {
            year: None,
            day: None,
            timestamp: None,
            content: SqlSearchText::empty(),
        }
    }
}

#[derive(Debug)]
pub struct RelationshipSearchParams {
    pub(crate) parent: SqlSearchText,
    pub(crate) child: SqlSearchText,
}

impl RelationshipSearchParams {
    pub fn new(parent: Option<SqlSearchText>, child: Option<SqlSearchText>) -> Self {
        let parent = match parent {
            Some(parent) => parent,
            None => SqlSearchText::empty(),
        };
        let child = match child {
            Some(child) => child,
            None => SqlSearchText::empty(),
        };
        Self { parent, child }
    }

    pub fn empty() -> Self {
        Self {
            parent: SqlSearchText::empty(),
            child: SqlSearchText::empty(),
        }
    }
}
