#![cfg(target_arch = "wasm32")]

use prproj::{
	PremiereReader as PremiereReaderOriginal,
	PremiereMedia, PremiereMedium, PremiereSequence,
	Reader,
	PremiereFile as PremiereFileOriginal
};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use js_sys::{Reflect, Array, Object};

impl std::convert::From<PremiereFileOriginal> for PremiereFile {
	fn from(original: PremiereFileOriginal) -> Self {
		Self {
			media: original.media,
			sequences: original.sequences
		}
	}
}

#[wasm_bindgen]
pub struct PremiereFile {
	media: Vec<PremiereMedium>,
	sequences: Vec<PremiereSequence>
}


#[wasm_bindgen]
impl PremiereFile {
	#[wasm_bindgen]
	pub fn take(self) -> Result<Object, JsValue> {
		let media: Array =
			self.media
				.into_iter()
				.map(JsValue::from)
				.collect();

		let sequences: Array =
			self.sequences
				.into_iter()
				.map(JsValue::from)
				.collect();

		let obj = Object::new();
		Reflect::set(
			&obj,
			&JsValue::from_str("media"),
			&media
		)?;
		Reflect::set(
			&obj,
			&JsValue::from_str("sequences"),
			&sequences
		)?;
		Ok(obj)
	}
}


#[wasm_bindgen]
pub fn read_prproj(xml: &[u8]) -> Result<PremiereFile, JsValue> {
	let mut reader = PremiereReaderOriginal::new(xml);
	reader.read().map_err(|err|
		JsValue::from_str(
			&format!("{:?}", err)
		)
	)?;
	Ok(reader.take().into())
}

// #[cfg(not(target_arch = "wasm32"))]
// #[cfg(test)]
// mod tests {
// 	#[test]
// 	fn only_wasm() {
// 		panic!("Only works on wasm");
// 	}
// }
//
// #[cfg(target_arch = "wasm32")]
// #[cfg(test)]
// mod tests {
// 	#[macro_use]
// 	extern crate wasm_bindgen_test;
//
// 	use std::path::PathBuf;
// 	use std::fs::File;
// 	use std::io::Read;
// 	use crate::read_prproj;
//
// 	#[wasm_bindgen_test]
// 	fn works() {
// 		let here = {
// 			let mut here = std::env::current_dir().unwrap();
// 			here.pop();
// 			here
// 		};
//
// 		let xml_file = {
// 			let mut here = here.clone();
// 			here.push(["test_files", "test.unzipped.prproj"].iter().collect::<PathBuf>());
// 			here
// 		};
// 		let mut vec = Vec::new();
// 		File::open(xml_file).unwrap().read_to_end(&mut vec).unwrap();
// 		read_prproj(&vec);
// 	}
// }

// struct PremiereReader {
// 	original: PremiereReaderOriginal
// }

/*
impl Reader<JsValue> for PremiereReader {
	fn new(xml: &[u8]) -> Self {
		Self {
			original: PremiereReaderOriginal::new(xml)
		}
	}

	fn read(&mut self) -> Result<(), JsValue> {
		let rv = self.original.read();
		rv.map_err(
			|err|
				JsValue::from_str(
					&format!("{:?}", err)
				)
		)
	}

	fn media
}*/
