// ===== Feature-based build mode flags =====
/// Build mode: "use_system_marisa" or "vendored"
#[cfg(feature = "use_system_marisa")]
pub const BUILD_MODE: &str = "use_system_marisa";
#[cfg(not(feature = "use_system_marisa"))]
pub const BUILD_MODE: &str = "vendored";

// ===== Public modules =====

mod marisa;

#[allow(dead_code)]
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
mod ffi;

#[allow(non_camel_case_types)]
mod utils;

// ===== Re-exports (public API surface) =====
