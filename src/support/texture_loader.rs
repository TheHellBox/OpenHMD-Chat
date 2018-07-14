use image;
use glium::Display;
use glium::texture::Texture2d;

pub fn load(path: String, disp: &Display) -> Texture2d{
    use std::path::Path;
    use glium::texture::RawImage2d;
    if let Ok(img) = image::open(Path::new(&path)){
        let img = img.to_rgba();
        let dis = img.dimensions();
        let glium_raw_tex = RawImage2d::from_raw_rgba_reversed(&img.into_raw(), dis);
        let tex = Texture2d::new(disp, glium_raw_tex).unwrap();
        tex
    }
    else{
        load("./assets/dev.png".to_string(), disp)
    }
}
