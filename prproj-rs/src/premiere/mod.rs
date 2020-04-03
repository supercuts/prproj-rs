pub mod media;
pub mod reader;
pub mod sequence;

pub use media::{PremiereMedia, PremiereMedium};
pub use reader::{PremiereReader, Reader};
pub use sequence::{PremiereSequence, PremiereSequences};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
use std::borrow::Borrow;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Clone, Copy, Default, Debug)]
pub struct Size {
	pub width: u32,
	pub height: u32,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Default, Debug)]
pub struct Cut {
	pub start: f64,
	pub end: f64,
	//	media: &'a RefCell<PremiereMedia>,
	medium: Box<PremiereMedium>,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl Cut {
	#[wasm_bindgen(getter)]
	pub fn medium(&self) -> PremiereMedium {
		*self.medium.clone()
	}
}

#[derive(Clone)]
enum FindWith {
	ID,
	UID,
}

impl FindWith {
	fn value(&self) -> &'static str {
		match self {
			FindWith::ID => "ObjectID",
			FindWith::UID => "ObjectUID"
		}
	}
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Default, Debug)]
pub struct Cuts {
	cuts: Vec<Cut>
}

impl Cuts {
	fn push(&mut self, cut: Cut) -> usize {
		self.cuts.push(cut);
		self.cuts.len()
	}
}
