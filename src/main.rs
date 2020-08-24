extern crate cairo;

use cairo::{Context, FontSlant, FontWeight, Format, ImageSurface};
use std::fs::File;

struct Rect {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

fn main() {
    // custom parameters
    let image: &str = "meme.png";
    let font_size: f64 = 19.2;
    let font_family: &str = "sans-serif";
    let rect = Rect {
        x: 44.0,
        y: 222.5 + font_size / 2 as f64,
        width: 109.0,
        height: 37.0
    };
    let message: &str = "hello world";
    let output: &str = "file.png";

    let mut file = File::open(image).expect("Invalid image path");
    let surface = ImageSurface::create_from_png(&mut file).unwrap();
    let cr = Context::new(&surface);

    cr.set_font_size(font_size);
    cr.select_font_face(font_family, FontSlant::Italic, FontWeight::Normal);
    cr.rectangle(rect.x, rect.y, rect.width, rect.height);
    cr.show_text(message);

    let mut file = File::create(output).expect(&format!("Couldn't create '{}'", output));
    match surface.write_to_png(&mut file) {
        Ok(_) => println!("{} created", output),
        Err(_) => println!("Error create {}", output),
    }
}
