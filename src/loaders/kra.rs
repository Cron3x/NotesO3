use std::path::PathBuf;
use std::fs;

pub(super) fn unpack(mut archive: zip::ZipArchive<fs::File>) {
    for i in 0..archive.len(){
        let inner = archive.by_index(i).unwrap();
        println!("\tName: {}", inner.name());
    }
}
