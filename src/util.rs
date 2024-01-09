
pub fn get_string_from_chars(chars: &[u8]) -> String {
    String::from(
        std::ffi::CStr::from_bytes_until_nul(chars).unwrap().to_str().unwrap()
    )
} 