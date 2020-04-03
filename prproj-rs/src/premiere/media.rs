use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::time::Duration;
use super::Size;
#[cfg(target_arch = "wasm32")]
use {
	wasm_bindgen::{
		prelude::*,
		JsValue
	},
	js_sys::{
		Object,
		Reflect
	},
	crate::wasm_utils::{
		WasmDuration,
		IntoWasm, IntoWasmRef
	}
};

#[derive(Debug, Default)]
pub struct PremiereMedia {
	pub media: HashSet<Box<PremiereMedium>>
}

impl PremiereMedia {
	pub(crate) fn insert(&mut self, medium: PremiereMedium) -> Box<PremiereMedium> {
		if self.media.contains(&medium) {
			return match self.media.get(&medium) {
				Some(value) => value.to_owned(),
				None => unreachable!()
			};
		}
		self.media.insert(Box::new(medium.to_owned()));
		self.media.get(&medium).unwrap().to_owned()
	}
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Clone, Debug, Default)]
pub struct PremiereMedium {
	file_name: String, // TODO: impl
	file_path: String, // TODO: impl
	pub frame_rate: u64,
	duration: Duration,
	pub size: Size,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl PremiereMedium {
	#[wasm_bindgen(getter)]
	pub fn fileName(&self) -> JsValue {
		JsValue::from_str(&self.file_name)
	}
	#[wasm_bindgen(getter)]
	pub fn filePath(&self) -> JsValue {
		JsValue::from_str(&self.file_path)
	}
	#[wasm_bindgen(getter)]
	pub fn duration(&self) -> JsValue {
		self.duration.borrow_wasm()
	}
}

impl PremiereMedium {
	pub fn new(
		file_name: String,
		file_path: String,
		frame_rate: u64,
		duration: Duration,
		size: Size,
	) -> Self {
		Self {
			file_name,
			file_path,
			frame_rate,
			duration,
			size
		}
	}
}

impl PartialEq for PremiereMedium {
	fn eq(&self, other: &Self) -> bool {
		self.file_name == other.file_name
	}
}

impl Eq for PremiereMedium {}

impl Hash for PremiereMedium {
	fn hash<H: Hasher>(&self, state: &mut H) { self.file_name.hash(state) }
}
