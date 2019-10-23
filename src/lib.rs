#![feature(proc_macro_hygiene, slice_patterns)]
#![cfg(feature = "frontend")]
#[macro_use]
extern crate wasm_bindgen;

use std::task::Poll;
use std::future::Future;

use wasm_bindgen::prelude::*;
use web_sys::console;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn get_root_element() -> Result<web_sys::Element, JsValue> {
    web_sys::window()
        .and_then(|w| w.document())
        // N.B. query_selector returns Result<Option<Element>>
        // So, calling .ok() on that converts it to an Option<Option<Element>>
        // and hence, we must call .ok_or() twice.
        .and_then(|d| d.query_selector("#modal-1-content").ok())
        .ok_or(JsValue::NULL)?
        .ok_or(JsValue::NULL)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Harmonogram {
	prirodovedci: Vec<(String, String, String, String, String, String, String)>,
	humanities: Vec<(String, String, String, String, String, String, String)>,
	praktici: Vec<(String, String, String, String, String, String, String)>,
}

pub async fn harmonogram() -> Result<Harmonogram, JsValue> {
	let mut opts = RequestInit::new();
	opts.method("GET");
	opts.mode(RequestMode::Cors);

	let req = Request::new_with_str_and_init(
		"https://localhost:8000/harmonogram",
		&opts
	)?;

	let window = web_sys::window().unwrap();
	let resp_val = JsFuture::from(window.fetch_with_request(&req)).await?;

	assert!(resp_val.is_instance_of::<Response>());
	let resp: Response = resp_val.dyn_into().unwrap();

	let json = JsFuture::from(resp.json()?).await?; 

	let harmonogram: Harmonogram = json.into_serde().unwrap();

	Ok(harmonogram)
}


// This is the entry point of your app
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console::log_1(&JsValue::from_str(
        "Welcome to Smithy! Head to `src/lib.rs`. Happy hacking!",
    ));

	/*let h = loop {
		match harmonogram().poll() {
			Poll::Pending => (),
			Poll::Ready(val) => break val.unwrap(),
		};
	};*/

	 

	//console::log_1(&JsValue::from_str(format!("{:?}")));

	// This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let root_element = get_root_element()?;

    let app = smithy::smd!(
        <h1>uwu</h1>
    );
    smithy::mount(Box::new(app), root_element);

    console::log_1(&JsValue::from_str(
        "Welcome to Smithy! Head to `src/lib.rs`. Happy hacking!",
    ));

    Ok(())
}
