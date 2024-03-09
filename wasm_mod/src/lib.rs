mod utils;
use leptos::*;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    // Use this to log in the browser console
    #[wasm_bindgen(js_namespace=console)]
    fn log(s: &str);
}

#[component]
fn HelloWorld() -> impl IntoView {
    log("Setup Hello world component");
    view! {
        <div>
            <h1 style="color:Turquoise;" >Hello World ! I am leptos !</h1>
        </div>
    }
}

fn add_leptos_component(document: &web_sys::Document) -> anyhow::Result<()> {
    log("Trying to get element to add leptos node");
    let body_element = document.body().expect("Couldn´t get the body");
    mount_to(body_element, || view! { <HelloWorld/> });
    log("Leptos added !");
    Ok(())
}

// Will be called in content.js
#[wasm_bindgen]
pub fn hello_content() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    log("Hello from the content script! ");
    if let Err(err) = add_leptos_component(&document()) {
        log(format!("Something failed when adding the leptos component {}", err).as_str());
    }

    log("bye from the content script! ");
}

// Will be called in background.js
#[wasm_bindgen]
pub fn hello_background() {
    log("Hello from the background script!");
}
