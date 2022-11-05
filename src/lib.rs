mod utils;

//use indoc::indoc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response, Window};
use serde_yaml::Mapping;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


// Next let's define a macro that's like `println!`, only it works for
// `console.log`. Note that `println!` doesn't actually work on the wasm target
// because the standard library currently just eats all output. To get
// `println!`-like behavior in your app you'll likely want a macro like this.

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}
    

#[wasm_bindgen]
extern {
    fn alert(s: &str);
    
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}


#[wasm_bindgen]
pub async fn greet(name: &str) {
    console_log!("Let's print some numbers...");
    //log("Hello from Rust!");
    let yaml = run().await;
    match yaml {
        Ok(v) => buildResume(v),
        Err(e) => println!("error {e:?}"),
    }
    alert(format!("Hello {}", name).as_str());
}

fn buildResume(s:String) {
    console_log!("buildresume {}",s);
    let t = read(s);
    match t {
        Ok(v) => console_log!("ok"),
        Err(e) => console_log!("not ok {}", e),
    }
    //Ok(())
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Node {
    supported_by : Option<Vec<String>>,
    #[serde(flatten)]
    extras: HashMap<String, String>,
}

fn read(s: String)->  Result<(), serde_yaml::Error> {
    console_log!("hej{}",s);
    let ss = r#"---
    - title: "resume"
      experience: "Sn1"
    - Sn1: "Lala"
    "#;
    console_log!("{}",s);
    console_log!("{}",ss);
    let deser: Vec<Node> = serde_yaml::from_str(&ss)?;
    println!("{:#?}", deser);
    
    

    
    
    let x = deser[0].extras.get("experience").ok_or(&String::from("Hello")).unwrap();
    //&s[..]
    
    add_heading_test(x);

    //console_log!("{}", x);
    //match x {
    //    Ok(v) => {
    //        console_log!("okss");
    //        //Ok(());
    //    },
    //    Err(e) => {
    //        console_log!("error {e:?}");
    //        //Ok(());
    //    },
    //}

    
    Ok(())
}

#[wasm_bindgen]
pub fn add_heading_test(message: &str) -> Result<(), JsValue> { 
    let window = web_sys::window().expect("no window found");
    let document = window.document().expect("no document on window");
    let body = document.body().expect("no body on document");

    let heading = document.create_element("h1")?;
    heading.set_inner_html(message);
    body.append_child(&heading)?;
    Ok(())
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
pub async fn run() -> Result<String, JsValue> {
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
    let yamlText = JsFuture::from(resp.text()?).await?.as_string().unwrap();

    // Send the JSON response back to JS.
    Ok(yamlText)
}