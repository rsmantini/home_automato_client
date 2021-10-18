use wasm_bindgen::prelude::*;

mod client;

#[wasm_bindgen]
pub fn home_automato_client() {
    #[cfg(feature = "console_error_panic_hook")]
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    let _result = client::run();
}