#![feature(proc_macro_hygiene, slice_patterns)]
#![cfg(feature = "frontend")]
#[macro_use]
extern crate wasm_bindgen;
extern crate serde_json;

use wasm_bindgen::prelude::*;
use web_sys::console;

use serde_json::Value;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::{JsFuture, spawn_local};
use web_sys::{Request, RequestInit, RequestMode, Response};

use std::convert::From;
use std::collections::HashMap;

use smithy::smd;
use smithy::types::{Node, Component};

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
        .and_then(|d| d.query_selector(".harmonogram-tabulka").ok())
        .ok_or(JsValue::NULL)?
        .ok_or(JsValue::NULL)
}


// Harmonogram tabulka
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Row {
	anotace: Value,
	#[serde(rename = "jménohosta")]
	jmeno: Value,
	#[serde(rename = "sedíčas")]
	cas_ok: Value,
	stav: Value,
	obor: Value,
	korektura: Value,
	#[serde(rename = "plánovanýčaspřednášky")]
	cas: Value,
	#[serde(rename = "názevpřednášky")]
	nazev: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrueRow {
	anotace: Option<String>,
	jmeno: Option<String>,
	cas_ok: bool,
	stav: bool,
	obor: Option<String>,
	korektura: bool,
	cas: Option<String>,
	nazev: Option<String>,
}

impl From<Row> for TrueRow {
	fn from(src: Row) -> Self {
		TrueRow {
			anotace: match src.anotace {
				Value::String(s) => Some(s),
				_ => None,
			},
			jmeno: match src.jmeno {
				Value::String(s) => Some(s),
				_ => None,
			},
			cas_ok: match src.cas_ok {
				Value::String(s) if s.as_str() == "Ano" => true,
				_ => false,
			},
			stav: match src.stav {
				Value::String(s) if s.as_str() == "Přijal" => true,
				_ => false,
			},
			obor: match src.obor {
				Value::String(s) => Some(s),
				_ => None,
			},
			korektura: match src.korektura {
				Value::String(s) if s.as_str() == "Provedena"  => true,
				_ => false,
			},
			cas: match src.cas {
				Value::String(s) => Some(s),
				_ => None,
			},
			nazev: match src.nazev {
				Value::String(s) => Some(s),
				_ => None,
			}
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Container {
	rows: Vec<Row>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarmonogramTabulka {
	prirodovedci: Container,
	humanities: Container,
}

// Harmonogram real
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Den(HashMap<String, Vec<TrueRow>>);

impl Den {
	pub fn new() -> Den {
		Den(HashMap::new())
	}
}

impl Component for Den {
	fn render(&mut self) -> Node {
		let mut list = self.0.iter().map(|(a, b)| (a.clone(), b.clone())).collect::<Vec<(String, Vec<TrueRow>)>>();
		list.sort_by(|a, b| a.0.cmp(&b.0));
 
		smd!(
			<table>
				{
					list.iter().cloned().map(|x| smd!(
						<tr>
							<th>{x.0.clone()}</th>

							{ x.1.iter().cloned().map(|y| smd!(
									<td>
										<p class="prednasejici">{y.jmeno.clone()}</p>
										<p class="nazev-prednasky">{y.nazev.clone()}</p>
									</td>
								)).collect::<Vec<_>>()
							}
						</tr>
					)).collect::<Vec<_>>()
				}
			</table>
		).render()
	}
}


#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Harmonogram {
	ctvrtek: Den,
	patek: Den,
	sobota: Den,
}

impl Component for Harmonogram {
	fn render(&mut self) -> Node {
		smd!(
			<h1>Čtvrtek</h1>
			{ self.ctvrtek.clone() }
			<h1>Pátek</h1>
			{ self.patek.clone() }
			<h1>Sobota</h1>
			{ self.sobota.clone() }
		).render()
	}
}

pub async fn harmonogram() -> () {
	let mut opts = RequestInit::new();
	opts.method("GET");
	opts.mode(RequestMode::Cors);

	let req = Request::new_with_str_and_init(
		"/harmonogram",
		&opts
	).unwrap();

	let window = web_sys::window().unwrap();
	let resp_val = JsFuture::from(window.fetch_with_request(&req)).await.unwrap();

	assert!(resp_val.is_instance_of::<Response>());
	let resp: Response = resp_val.dyn_into().unwrap();

	let json = JsFuture::from(resp.json().unwrap()).await.unwrap(); 

	let h: HarmonogramTabulka = json.into_serde().unwrap();

	let valid = h.prirodovedci.rows.iter()
		.chain(h.humanities.rows.iter())
		.map(|r| TrueRow::from(r.clone()))
		.filter(|r| r.stav && r.cas_ok)
		.collect::<Vec<TrueRow>>();
		
	let harmonogram = Harmonogram {
		ctvrtek: valid.iter()
			.fold(Den::new(), |mut acc, x|
				if x.cas.clone().and_then(|c| if c.starts_with("ČT") { Some(()) } else { None }).is_some() {
					acc.0.entry(x.cas.clone().unwrap().chars().skip(3).collect())
						.and_modify(|e| e.push(x.clone()))
						.or_insert(vec![x.clone()]);
					acc
				} else { acc }
			),
		patek: valid.iter()
			.fold(Den::new(), |mut acc, x|
				if x.cas.clone().and_then(|c| if c.starts_with("PÁ") { Some(()) } else { None }).is_some() {
					acc.0.entry(x.cas.clone().unwrap().chars().skip(3).collect())
						.and_modify(|e| e.push(x.clone()))
						.or_insert(vec![x.clone()]);
					acc
				} else { acc }
			),
		sobota: valid.iter()
			.fold(Den::new(), |mut acc, x|
				if x.cas.clone().and_then(|c| if c.starts_with("SO") { Some(()) } else { None }).is_some() {
					acc.0.entry(x.cas.clone().unwrap().chars().skip(3).collect())
						.and_modify(|e| e.push(x.clone()))
						.or_insert(vec![x.clone()]);
					acc
				} else { acc }
			),
	};

	console::log_1(&JsValue::from_serde(&harmonogram).unwrap());

    let root_element = get_root_element().unwrap();

    let app = smithy::smd!(
		{ harmonogram.clone() }
    );
    smithy::mount(Box::new(app), root_element);
}


// This is the entry point of your app
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console::log_1(&JsValue::from_str(
        "Welcome to Smithy! Head to `src/lib.rs`. Happy hacking!",
    ));

	// This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    console_error_panic_hook::set_once();

	spawn_local(harmonogram());
	Ok(())
}
