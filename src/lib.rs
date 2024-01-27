//! System crate for aprilasr speech-to-text library
//!
//! Documentation: [stable](https://docs.rs/aprilasr-sys/)

/// Foreign Function Interface module
#[allow(unused, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub mod ffi {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn it_can_initialize() -> Result<(), String> {
        let result = unsafe { ffi::aam_api_init(1) };
        assert_eq!(result, ());
        Ok(())
    }
}
