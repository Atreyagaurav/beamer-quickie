use std::env;

mod texparse;

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = texparse::BeamerContents::load(&args[1]).unwrap();
    let slide = contents.slides().nth(4).unwrap();
    println!("{}", contents.single_frame_tex(slide));
    // texparse::thumbnail(&texparse::CACHE, slide);
    // println!("{}", beamer.to_string());
}
