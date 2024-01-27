use aprilasr_sys::ffi as afi;
use std::ffi::c_int;

static APRIL_VERSION: c_int = afi::APRIL_VERSION as i32;

/// Initialize April API using the version constant specified in bindings.
fn main() {
    unsafe { afi::aam_api_init(APRIL_VERSION) }
    print!("April ASR api v1 initialized and ready for model.")
}
