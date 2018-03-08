use image;
use glium;
use std::collections::HashMap;

pub fn open_image(disp: &glium::Display, name: String) -> glium::Texture2d{
    let image = image::open(name).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let texture = glium::texture::Texture2d::new(disp, image).unwrap();
    texture
}

pub fn gen_texture_buffer(disp: &glium::Display) -> HashMap<String, glium::Texture2d>{
    use std::fs;
    let paths = fs::read_dir("./assets/textures/").unwrap();
    let mut textures: HashMap<String, glium::Texture2d> = HashMap::with_capacity(512);
    for path in paths {
        let path = path.unwrap().path();
        if path.is_file() {
            let name = path.display().to_string();
            print!("Loading texture {} ... ", path.display());
            if name.ends_with(".png") {
                let raw = open_image(disp, path.display().to_string());
                textures.insert(name, raw);

                println!("Done!");
            }
            else{
                println!("Invalid file format");
            }
        }
    }
    textures
}
