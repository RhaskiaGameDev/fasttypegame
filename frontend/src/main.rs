#![recursion_limit = "256"]

mod text_input;

mod app;
use crate::app::*;

use yew::prelude::*;

fn main() {
    println!("{:?}", csv::loader::WORLDCITIES);
    yew::Renderer::<App>::new().render();
}