extern crate cairo;
extern crate pango;
extern crate pangocairo;

use cairo::{Context, ImageSurface};
use pango::{Alignment, FontDescription, Style, Weight, WrapMode};
use std::fs::File;

struct Rect {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

fn main() {
    // custom parameters
    let image: &str = "meme.png";
    let font_size: i32 = 12 * pango::SCALE;
    let font_family: &str = "sans-serif";
    let rect = Rect {
        x: 44,
        y: 166,
        width: 109,
        height: 150,
    };
    let message: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit.";
    let output: &str = "file.png";

    // set font description
    let mut font = FontDescription::new();
    font.set_family(font_family);
    font.set_style(Style::Italic);
    font.set_weight(Weight::Normal);
    font.set_size(font_size);

    let mut file = File::open(image).expect("Invalid image path");
    let surface = ImageSurface::create_from_png(&mut file).unwrap();
    let context = Context::new(&surface);
    let layout = pangocairo::create_layout(&context).unwrap();

    // update pango layout
    layout.set_width(rect.width * pango::SCALE);
    layout.set_text(message);
    layout.set_wrap(WrapMode::Word);
    layout.set_alignment(Alignment::Center);
    layout.set_font_description(Option::from(&font));

    pangocairo::update_layout(&context, &layout);
    let y: f64 = {
        let (_, height) = layout.get_pixel_size();
        rect.y as f64 + ((rect.height - height) as f64 / 2.0)
    };
    context.move_to(rect.x as f64, y);
    pangocairo::show_layout(&context, &layout);

    let mut file = File::create(output).expect(&format!("Couldn't create '{}'", output));
    match surface.write_to_png(&mut file) {
        Ok(_) => println!("{} created", output),
        Err(_) => println!("Error create {}", output),
    }
}
