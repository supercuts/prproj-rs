#[cfg(not(target_arch = "wasm32"))]
#[test]
fn only_wasm() {
	panic!("Only works on wasm!")
}

#[cfg(target_arch = "wasm32")]
use {
	prproj_wasm::read_prproj,
	wasm_bindgen_test::*,
	std::path::PathBuf,
	std::fs::File,
	std::io::Read
};

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test]
fn works() {
	let xml_file = include_bytes!("../../../test_files/test.unzipped.prproj");
	read_prproj(xml_file);
}
