mod utils;

use wasm_bindgen::prelude::*;
use web_sys::*;
use magnetite_lib::ob_sys::*;
use utils::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[wasm_bindgen]
pub fn onload(magnetite_host: &Plugin) {
    set_panic_hook();
    magnetite_host.Plugin_addRibbonIcon("dice", "Magnetite");
}

#[wasm_bindgen]
pub fn onunload() {
    let w = window().expect("no window");
    w.alert_with_message("See you next time! Love, Magnetite").expect("we said bye");
}