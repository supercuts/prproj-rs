use prproj::PremiereReader;
use std::path::Path;
use std::time::Instant;

const XML_FILE: &str = r"examples\test.unzipped.prproj";
//const GZ_FILE: &str = r"examples\test.zipped.prproj";

fn main() {
    let before = Instant::now();
    let mut reader = PremiereReader::from_path(&Path::new(XML_FILE));
	if let Err(err) = reader.read() {
        println!("Error reading Premiere Pro file: {}\n{:#?}!", err, err)
    }
    println!("Time to read: {:#?}", Instant::now() - before);
//    println!("Sequences: {:#?}", reader.sequences());
//    println!("Media: {:#?}", reader.media());
}




