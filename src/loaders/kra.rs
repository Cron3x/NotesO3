use std::path::PathBuf;
use std::fs;

pub fn unpack(archive: &mut zip::read::ZipArchive<fs::File>) {
    println!("inner files:");
    for i in 0..archive.len(){
        let inner = archive.by_index(i).unwrap();
        println!("\tName: {}", inner.name());
    }
}
