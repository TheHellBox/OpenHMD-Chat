
use bytevec::ByteEncodable;
use bytevec::ByteDecodable;
use std::collections::HashMap;

#[derive(PartialEq, Debug, Default, Clone)]
pub struct ClParams{
    pub version: (u32, u32),
    pub gamefiles: HashMap<String, String>,
}

bytevec_impls! {
    impl ClParams {
        version: (u32, u32),
        gamefiles: HashMap<String, String>
    }
}
impl ClParams {
    pub fn new() -> ClParams{
        use std::io::prelude::*;
        use std::fs::File;
        use std::io::BufReader;
        use std::fs;
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut files = HashMap::with_capacity(512);

        let textures = fs::read_dir("./assets/textures/").unwrap();
        //Get models pathes
        let models = fs::read_dir("./assets/models/").unwrap();

        let mut pathes = vec![];

        for x in textures{
            let path = x.unwrap().path();
            if path.is_file(){
                //Checking if format is ok
                if path.extension().unwrap() == "png" || path.extension().unwrap() == "jpeg"{
                    pathes.push(path);
                }
            }
        }
        for x in models{
            let path = x.unwrap().path();
            if path.is_file(){
                //Checking if format is ok
                if path.extension().unwrap() == "obj" || path.extension().unwrap() == "mtl"{
                    pathes.push(path);
                }
            }
        }
        for filename in pathes{
            let mut hasher = DefaultHasher::new();
            let mut file = File::open(&filename).unwrap();
            let mut buf = vec![];
            file.read_to_end(&mut buf);
            let hash = buf.hash(&mut hasher);
            let name = filename.display().to_string();
            files.insert(name, format!("{}", hasher.finish()));
        }
        ClParams{
            version: (0, 0),
            gamefiles: files
        }
    }
    pub fn to_network(&self) -> Vec<u8>{
        self.encode::<u16>().unwrap()
    }
    pub fn from_network(message: Vec<u8>) -> ClParams{
        ClParams::decode::<u16>(&message).unwrap()
    }
}
