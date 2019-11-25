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

mod errors;
mod timeline;
mod element;

const TICKS_PER_SECOND: usize = 254_016_000_000;

#[cfg(test)]
mod tests {
	use std::path::Path;
	use std::fs::File;
	use std::io::Read;
	use super::*;
	use crate::errors::NotFoundError;

	const XML_FILE: &str = r"examples\test.unzipped.prproj";
	const GZ_FILE: &str = r"examples\test.zipped.prproj";

	#[test]
	fn works_on_zipped() -> Result<(), NotFoundError> {
		let mut buffer: Vec<u8> = Vec::new();
		read_file(XML_FILE, &mut buffer);
		let mut reader = PremiereReader::new(&buffer);
		let result = reader.read();
		println!("Sequences: {:#?}", reader.sequences());
		println!("Media: {:#?}", reader.media());
		result
	}

	#[test]
	fn works_on_unzipped() -> Result<(), NotFoundError> {
		let mut buffer: Vec<u8> = Vec::new();
		read_file(GZ_FILE, &mut buffer);
		let mut reader = PremiereReader::new(&buffer);
		let result = reader.read();
		println!("Sequences: {:#?}", reader.sequences());
		println!("Media: {:#?}", reader.media());
		result
	}

	fn read_file(file_path: &str, buffer: &mut Vec<u8>) {
		let mut file = File::open(Path::new(file_path)).unwrap();
		file.read_to_end(buffer).unwrap();
	}
}
