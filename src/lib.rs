//! Low-level FFI bindings for the [april-asr](https://github.com/abb128/april-asr) C api (libaprilasr).
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

    /// Verifies that FFI bindings compile and aam_api_init is callable
    /// without crashing. Uses the generated APRIL_VERSION constant
    /// rather than a hardcoded value.
    #[test]
    fn it_can_initialize() {
        unsafe { ffi::aam_api_init(ffi::APRIL_VERSION as std::ffi::c_int) };
    }
}
