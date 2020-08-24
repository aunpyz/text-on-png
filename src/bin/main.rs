use std::env;
use text_on_png::{FontOptions, TextArea};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 10 {
        panic!("Number of arguments are not satisfied")
    }
    let mut args = args.iter();
    // discard the first argument
    args.next().unwrap();

    let image: &str = args.next().unwrap();
    let font_options: FontOptions = FontOptions::new(
        args.next()
            .unwrap()
            .parse()
            .expect("This argument must be number convertible, parsing font size filed"),
        args.next().unwrap(),
    );
    let text_area: TextArea = TextArea::new(
        args.next()
            .unwrap()
            .parse()
            .expect("This argument must be number convertible, parsing x-coord failed"),
        args.next()
            .unwrap()
            .parse()
            .expect("This argument must be number convertible, parsing y-coord failed"),
        args.next()
            .unwrap()
            .parse()
            .expect("This argument must be number convertible, parsing width failed"),
        args.next()
            .unwrap()
            .parse()
            .expect("This argument must be number convertible, parsing height failed"),
    );
    let message: &str = args.next().unwrap();
    let output: &str = args.next().unwrap();

    match text_on_png::create(image, font_options, text_area, message, output) {
        Ok(_) => println!("{} created", output),
        Err(_) => println!("Error create {}", output),
    }
}
