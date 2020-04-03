#[macro_export]
macro_rules! sorted_vec {
    ($($x:expr),*) => {
		{
			let mut temp_vec = Vec::new();
			$(
				temp_vec.push($x);
			)*
			temp_vec.sort();
			temp_vec
		}
    };
    ($($x:expr,)*) => ($crate::sorted_vec![$($x),*])
}

#[cfg(target_arch = "wasm32")]
#[cfg_attr(target_arch = "wasm32", macro_use)]
extern crate wasm_bindgen;

#[cfg(target_arch = "wasm32")]
pub mod wasm_utils;


pub mod premiere;
pub use premiere::{
	PremiereMedia, PremiereMedium,
	PremiereSequence, PremiereSequences
};
pub use premiere::reader::{PremiereReader, Reader, PremiereFile};

pub mod errors;
pub mod timeline;
pub mod element;

const TICKS_PER_SECOND: u64 = 254_016_000_000;


