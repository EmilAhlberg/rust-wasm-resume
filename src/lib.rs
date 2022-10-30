mod utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response, Window};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    run();
    //alert(format!("Hello {}", name).as_str());
}


#[wasm_bindgen]
pub struct Resume {
    width: u32,
    height: u32,
    content: String
}

use std::fmt;

impl fmt::Display for Resume {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "symbol")?;
        write!(f, "\n")?;
        Ok(())
    }
}

#[wasm_bindgen]
pub fn add_heading() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no window found");
    let document = window.document().expect("no document on window");
    let body = document.body().expect("no body on document");

    let heading = document.create_element("h1")?;
    heading.set_inner_html("This heading was created from Rust!");

    body.append_child(&heading)?;

    Ok(())
}

#[wasm_bindgen]
pub async fn run() -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let url = "https://raw.githubusercontent.com/EmilAhlberg/web-multiplayer-client/main/.github/dependabot.yml";

    let request = Request::new_with_str_and_init(&url, &opts)?;

    request
        .headers()
        .set("Accept", "application/vnd.github.v3+json")?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    // `resp_value` is a `Response` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    // Convert this other `Promise` into a rust `Future`.
    let json = JsFuture::from(resp.json()?).await?;

    // Send the JSON response back to JS.
    Ok(json)
}