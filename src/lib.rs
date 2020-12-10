mod utils;

use wasm_bindgen::prelude::*;
use web_sys::*;
use magnetite_lib::*;
use utils::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn onload() {
    set_panic_hook();
    let w = window().expect("no window");
    w.alert_with_message("Hello, Obsidian!, Love, Magnetite").expect("it said hi");
}
