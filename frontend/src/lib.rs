use wasm_bindgen::prelude::*;

mod app;

#[wasm_bindgen(start)]
pub fn run() {
    yew::Renderer::<app::App>::new().render();
}
