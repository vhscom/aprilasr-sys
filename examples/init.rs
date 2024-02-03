use aprilasr_sys::ffi as afi;
use std::ffi::c_int;

/// Initialize April API using the version constant specified in bindings.
/// Example intentionally low-level as are the bindings.
fn main() {
    unsafe { afi::aam_api_init(afi::APRIL_VERSION as c_int) }
    print!("April ASR api v1 initialized.")
}
