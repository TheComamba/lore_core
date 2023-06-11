use std::ffi::{CStr, CString};

use crate::errors::LoreCoreError;

pub(super) fn char_pointer_to_string(string: *const libc::c_char) -> Result<String, LoreCoreError> {
    let string: &str = unsafe {
        CStr::from_ptr(string).to_str().map_err(|e| {
            LoreCoreError::InputError(
                "Could not convert characterpointer to string.".to_string() + &e.to_string(),
            )
        })?
    };
    Ok(string.to_string())
}

pub(super) fn char_pointer_to_optional_string(
    string: *const libc::c_char,
) -> Result<Option<String>, LoreCoreError> {
    let string = char_pointer_to_string(string)?;
    Ok(if string.is_empty() {
        None
    } else {
        Some(string)
    })
}

pub(super) fn char_ptr(message: &str) -> *const libc::c_char {
    CString::new(message).unwrap().into_raw()
}
