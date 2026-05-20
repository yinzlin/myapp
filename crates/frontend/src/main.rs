//! Dioxus 前端应用入口

use dioxus::prelude::*;

mod components;
mod pages;
mod state;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        div {
            h1 { "Rust 全栈应用" }
            p { "欢迎使用 Dioxus 前端框架" }
        }
    }
}
