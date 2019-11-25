use prproj::PremiereReader;
/// https://stackoverflow.com/questions/57349817/how-can-i-get-a-dom-from-a-file-with-rust
use std::{
    fs::File,
    io::Read
};

#[allow(dead_code)]
const XML_FILE: &str = r"D:\projects\supercuts\prototyping\rs\parsers\prproj\examples\test.unzipped.prproj";
const GZ_FILE: &str = r"D:\projects\supercuts\prototyping\rs\parsers\prproj\examples\test.zipped.prproj";

fn main() {
    let mut file = File::open(XML_FILE).unwrap();

    // Read bytes
    let mut contents = Vec::new();
    let _num_bytes_read = file.read_to_end(&mut contents).unwrap();
    let mut reader = PremiereReader::new(&contents);
	if let Err(err) = reader.read() {
        println!("Error reading Premiere Pro file: {}\n{:#?}!", err, err)
    }
    println!("Sequences: {:#?}", reader.sequences());
    println!("Media: {:#?}", reader.media())
}




