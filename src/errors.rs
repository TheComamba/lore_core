use std::fmt::{Debug, Display};

#[derive(Debug, Clone)]
pub enum LoreCoreError {
    FileError(String),
    InputError(String),
    SqlError(String),
}

impl ToString for LoreCoreError {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

pub(super) fn sql_loading_error_no_params<E>(loadee: &str, target: &str, err: E) -> LoreCoreError
where
    E: Display,
{
    sql_loading_error::<String, E>(loadee, target, vec![], err)
}

pub(super) fn sql_loading_error<T, E>(
    loadee: &str,
    target: &str,
    params: Vec<(&str, &T)>,
    err: E,
) -> LoreCoreError
where
    T: Debug,
    E: Display,
{
    let mut message = "Loading ".to_string() + loadee + " to get " + target;
    for (i, (name, value)) in params.iter().enumerate() {
        if i == 0 {
            message += " for parameters ";
        } else {
            message += ", "
        }
        message += name;
        message += "='";
        message += &format!("{:?}", value);
        message += "'";
    }
    message += " failed: ";
    message += &err.to_string();
    LoreCoreError::SqlError(message)
}
