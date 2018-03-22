use std::io::prelude::*;
use std::fs::File;

pub struct FileWriter{
    pub curret_file: File
}
impl FileWriter{
    pub fn new(filename: String) -> FileWriter{
        let mut f = File::create(filename).unwrap();
        FileWriter{
            curret_file: f
        }
    }
    pub fn write(&mut self, data: Vec<u8>){
        self.curret_file.write_all(data.as_slice());
    }
}
