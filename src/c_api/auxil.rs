use crate::errors::LoreCoreError;
use std::ffi::{CStr, CString};

/// # Safety
///
/// `string` must be a valid C string.
pub(super) unsafe fn char_pointer_to_string(
    string: *const libc::c_char,
) -> Result<String, LoreCoreError> {
    if string.is_null() {
        return Err(LoreCoreError::InputError(
            "Characterpointer is null.".to_string(),
        ));
    }
    let string: &str = CStr::from_ptr(string).to_str().map_err(|e| {
        LoreCoreError::InputError(
            "Could not convert characterpointer to string: ".to_string() + &e.to_string(),
        )
    })?;
    Ok(string.to_string())
}

pub(super) fn string_to_char_pointer(string: &str) -> *const libc::c_char {
    CString::new(string).unwrap().into_raw()
}

pub(super) fn char_ptr(message: &str) -> *const libc::c_char {
    CString::new(message).unwrap().into_raw()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr;

    #[test]
    fn test_char_pointer_to_string() {
        let string = "Eyjafjallajökull!";
        let char_pointer = string_to_char_pointer(string);
        let result = unsafe { char_pointer_to_string(char_pointer) };
        assert_eq!(result.unwrap(), string);
    }

    #[test]
    fn test_null_pointer_to_string_error() {
        use std::ptr;
        let null_pointer = ptr::null();
        let result = unsafe { char_pointer_to_string(null_pointer) };
        assert!(
            result.is_err(),
            "Expected an error when converting a null pointer to a string"
        );
    }

    #[test]
    fn test_null_pointer_to_string() {
        let char_pointer = ptr::null();
        let result = unsafe { char_pointer_to_string(char_pointer) };
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_string_to_char_pointer() {
        let string = "";
        let char_pointer = string_to_char_pointer(string);
        let result = unsafe { char_pointer_to_string(char_pointer) };
        assert_eq!(result.unwrap(), string);
    }

    #[test]
    fn test_string_to_char_pointer() {
        let string = "Eyjafjallajökull!";
        let char_pointer = string_to_char_pointer(string);
        let result = unsafe { char_pointer_to_string(char_pointer) };
        assert_eq!(result.unwrap(), string);
    }
}
