use std::{fs, path::PathBuf};
use std::rc::Rc;
use std::io::Read;

mod kra;

pub fn load_files(){

     let paths = fs::read_dir("./files/").unwrap();

    for path in paths {
        //println!("Name: {}", &path.unwrap().path().display());

        let p = path.unwrap().path().to_path_buf();

        match p.is_file() {
            true => {
                handle_file(p).unwrap()
            }
            false => {eprintln!("UNIPLEMENTED: folders and symlinks")}
        }
    }

    //let fname = std::path::Path::new(&*args[1]);
    //let file = fs::File::open(fname).unwrap();
}

enum SupportedFileType{
    Kra,
    Odt,
}

impl SupportedFileType{
    fn identify_filetype(file: &fs::File) -> Result<Self, String>{
        match zip::ZipArchive::new(file) {
            Ok(mut z) => {
                match z.by_name("mimetype") {
                    Ok(mut t) => {
                        let mut buf = vec![0; t.size() as usize]; 
                        t.read(&mut buf).unwrap();
                        let buf_str = String::from_utf8(buf).unwrap();
                        println!("{}", &buf_str);
                        match buf_str.as_str() {
                            "application/x-krita" => {
                                return Ok(SupportedFileType::Kra)
                            }
                            "application/vnd.oasis.opendocument.text" => {
                                return Ok(SupportedFileType::Odt)
                            }
                            &_ => {}
                        }
                    }
                    Err(_) => {}
                }
            },
            Err(_) => {}
        }

        return Err("Unsuported File".to_owned())
    }

}
fn handle_file(file_path: PathBuf) -> Result<(), String>{
    //cases:
    //  txt
    //  zip
    //  img
    //  ...

    let file = fs::File::open(file_path).unwrap();

    let ft = file.metadata().unwrap().file_type();
    println!("{:#?}",ft);

    match SupportedFileType::identify_filetype(&file) {
        Ok(t) => match t {
            SupportedFileType::Kra => {

                let a = zip::ZipArchive::new(file).unwrap();
                kra::unpack(a)
            }
            SupportedFileType::Odt => {
            }
        }
        Err(e) => {
            eprintln!("UNSUPORTED: {}", e)
        }

    }
    Ok(())
}
