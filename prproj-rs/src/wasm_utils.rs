#![cfg(target_arch = "wasm32")]
use std::time::Duration;
use wasm_bindgen::{
	prelude::*,
	JsValue
};

pub trait IntoWasm {
	fn take(self) -> JsValue;
}
pub trait IntoWasmRef {
	fn borrow_wasm(&self) -> JsValue;
}

impl IntoWasmRef for String {
	fn borrow_wasm(&self) -> JsValue {
		JsValue::from_str(&self)
	}
}

#[wasm_bindgen]
pub struct WasmDuration {
	pub secs: u64,
	pub nanos: u64
}

impl IntoWasmRef for Duration {
	fn borrow_wasm(&self) -> JsValue {
		WasmDuration {
			secs: self.as_secs(),
			nanos: self.as_nanos() as u64
		}.into()
	}
}

/*
#![cfg(target_arch = "wasm32")]

#[wasm_bindgen]
pub struct WasmDuration {
	pub secs: u64
}

impl From<Duration> for WasmDuration {
	fn from(dur: Duration) -> Self {
		Self {
			secs: dur.as_secs()
		}
	}
}
*/
