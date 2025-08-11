use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod app;

#[wasm_bindgen(start)]
pub fn run() {
    yew::Renderer::<app::App>::new().render();
}
