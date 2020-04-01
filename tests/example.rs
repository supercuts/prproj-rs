#[macro_use]
extern crate lazy_static;

use prproj::PremiereReader;
use std::path::{Path, PathBuf};
use std::time::Instant;

lazy_static! {
    static ref HERE: PathBuf = std::env::current_dir().unwrap();

    static ref XML_FILE: PathBuf = {
        let mut here = HERE.clone();
        here.push(["tests", "test.unzipped.prproj"].iter().collect::<PathBuf>());
        here
    };
    static ref GZ_FILE: PathBuf = {
        let mut here = HERE.clone();
        here.push(["tests", "test.zipped.prproj"].iter().collect::<PathBuf>());
        here
    };
}

fn read(path: &Path) {
    let before = Instant::now();
    let mut reader = PremiereReader::from_path(&Path::new(path));
    if let Err(err) = reader.read() {
        println!("Error reading Premiere Pro file: {}\n{:#?}!", err, err)
    }
    println!("Time to read: {:#?}", Instant::now() - before);
    println!("Sequences: {:#?}", reader.sequences());
    println!("Media: {:#?}", reader.media());
}

#[test]
fn it_reads_xml_without_errors() {
    read(&*XML_FILE);
}

#[test]
fn it_reads_gzip_without_errors() {
    read(&*GZ_FILE);
}




