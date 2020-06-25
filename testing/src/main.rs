// use parser::parse;
// use std::fs;
// use renderer::Renderer;

// fn main() {
//     let json = fs::read_to_string("src.json").unwrap();
//     let result = parse(json).unwrap();
//     // let renderer = Renderer::new(&result.record_map.block);
//     // let rendered = renderer.render(&String::from("ddda599f-ff69-4974-9dec-86f6abf3209a"));
//     // let html = rendered.into_string();
//     // println!("{:?}", result.record_map.block.keys());
//     // println!("{}", &html);
//     // fs::write("output.html", &html).unwrap();
// }

use base::parser::parse;
use std::fs;
use renderer::Renderer;

fn main() {
    println!("Hello, world!");
}