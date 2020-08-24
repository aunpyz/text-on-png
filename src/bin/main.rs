use text_on_png::{FontOptions, TextArea};

fn main() {
    let image: &str = "meme.png";
    let font_options: FontOptions = FontOptions::new(12, "sans-serif");
    let text_area: TextArea = TextArea::new(44, 166, 109, 150);
    let message: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit.";
    let output: &str = "file.png";

    match text_on_png::create(image, font_options, text_area, message, output) {
        Ok(_) => println!("{} created", output),
        Err(_) => println!("Error create {}", output),
    }
}
