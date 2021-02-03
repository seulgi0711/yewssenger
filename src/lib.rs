#![recursion_limit = "256"]

use wasm_bindgen::prelude::*;
use yew::prelude::App;

mod components;
mod types;
mod yewssenger;

#[wasm_bindgen(start)]
pub fn run_app() {
  wasm_logger::init(wasm_logger::Config::default());
  App::<yewssenger::Yewssenger>::new().mount_to_body();
}
