#[macro_use]
extern crate lazy_static;

use prproj::{PremiereReader, Reader};
use std::path::{Path, PathBuf};
use std::time::Instant;

lazy_static! {
    static ref HERE: PathBuf = {
        let mut here = std::env::current_dir().unwrap();
        here.pop();
        here
    };

    pub static ref XML_FILE: PathBuf = {
        let mut here = HERE.clone();
        here.push(["test_files", "test.unzipped.prproj"].iter().collect::<PathBuf>());
        here
    };
    pub static ref GZ_FILE: PathBuf = {
        let mut here = HERE.clone();
        here.push(["test_files", "test.zipped.prproj"].iter().collect::<PathBuf>());
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
    println!("{:?}", &*XML_FILE);
    read(&*XML_FILE);
}

#[test]
fn it_reads_gzip_without_errors() {
    read(&*GZ_FILE);
}




