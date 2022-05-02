mod app;

use wasm_bindgen::prelude::*;

use app::App;

// #[wasm_bindgen]
pub fn main() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
    Ok(())
}
