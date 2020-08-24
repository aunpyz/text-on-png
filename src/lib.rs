extern crate cairo;
extern crate pango;
extern crate pangocairo;

use cairo::{Context, ImageSurface};
use pango::{Alignment, FontDescription, Style, Weight, WrapMode};
use std::fs::File;
use std::error::Error;

/// Area to display text.
pub struct TextArea {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl TextArea {
    /// Create an area to bound a text.
    /// 
    /// All parameters are in pixel size.
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> TextArea {
        TextArea {
            x,
            y,
            width: width,
            height: height,
        }
    }

    fn get_pango_height(&self) -> i32 {
        self.height * pango::SCALE
    }

    fn get_pango_width(&self) -> i32 {
        self.width * pango::SCALE
    }
}

/// Options for a font.
pub struct FontOptions<'a> {
    font_size: i32,
    font_family: &'a str,
}

impl<'a> FontOptions<'a> {
    /// Create a new FontOptions.
    /// 
    /// `font_size` is in pixel size.
    pub fn new(font_size: i32, font_family: &'a str) -> FontOptions<'a> {
        FontOptions {
            font_size: font_size,
            font_family,
        }
    }

    fn get_pango_size(&self) -> i32 {
        self.font_size * pango::SCALE
    }
}

pub fn create(
    image_path: &str,
    font_options: FontOptions,
    text_area: TextArea,
    message: &str,
    output: &str,
) -> Result<(), Box<dyn Error>> {
    // set font description
    let mut font = FontDescription::new();
    font.set_family(font_options.font_family);
    font.set_style(Style::Italic);
    font.set_weight(Weight::Normal);
    font.set_size(font_options.get_pango_size());

    let mut file = File::open(image_path).expect("Could not open file");
    let surface = ImageSurface::create_from_png(&mut file).unwrap();
    let context = Context::new(&surface);
    let layout = pangocairo::create_layout(&context).unwrap();

    // update pango layout
    layout.set_width(text_area.get_pango_width());
    layout.set_text(message);
    layout.set_wrap(WrapMode::Word);
    layout.set_alignment(Alignment::Center);
    layout.set_font_description(Option::from(&font));

    pangocairo::update_layout(&context, &layout);
    let y: f64 = {
        let (_, height) = layout.get_pixel_size();
        text_area.y as f64 + ((text_area.height - height) as f64 / 2.0)
    };
    context.move_to(text_area.x as f64, y);
    pangocairo::show_layout(&context, &layout);

    let mut file = File::create(output).expect(&format!("Couldn't create '{}'", output));
    match surface.write_to_png(&mut file) {
        Ok(_) => Ok(()),
        Err(e) => Err(Box::new(e))
    }
}
