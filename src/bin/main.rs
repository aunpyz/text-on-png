use std::env;
use text_on_png::{FontOptions, TextArea};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 10 {
        panic!("Number of arguments are not satisfied")
    }

    let image: &str = &args[1];
    let font_options: FontOptions = FontOptions::new(
        (&args[2].parse())
            .clone()
            .expect("This argument must be number convertible, parsing font size filed"),
        &args[3],
    );
    let text_area: TextArea = TextArea::new(
        (&args[4].parse())
            .clone()
            .expect("This argument must be number convertible, parsing x-coord failed"),
        (&args[5].parse())
            .clone()
            .expect("This argument must be number convertible, parsing y-coord failed"),
        (&args[6].parse())
            .clone()
            .expect("This argument must be number convertible, parsing width failed"),
        (&args[7].parse())
            .clone()
            .expect("This argument must be number convertible, parsing height failed"),
    );
    let message: &str = &args[8];
    let output: &str = &args[9];

    match text_on_png::create(image, font_options, text_area, message, output) {
        Ok(_) => println!("{} created", output),
        Err(_) => println!("Error create {}", output),
    }
}
