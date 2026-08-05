//! Optional enhancement runtime generation.

/// Name of the generated keyboard runtime file under output.
pub const KEYBOARD_RUNTIME_PATH: &str = "js/mkpage-keyboard-v1.js";

/// Version label emitted only in docs and tests.
pub const KEYBOARD_RUNTIME_VERSION: &str = "1";

/// Returns the generated keyboard enhancement runtime.
pub const fn runtime() -> &'static str {
    include_str!("../assets/enhancements/keyboard-runtime.js")
}
