use std::fs::File;
use std::io::Write;

pub fn create_file(){
    let mut file=File::create("src/example.txt")
        .expect("create failed");
    file.write_all("hello world".as_bytes())
        .expect("write failed");
}