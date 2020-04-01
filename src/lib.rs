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

mod premiere;
pub use premiere::PremiereReader;

pub mod errors;
mod timeline;
mod element;

const TICKS_PER_SECOND: usize = 254_016_000_000;


