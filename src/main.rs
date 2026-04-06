mod chunk;
mod engine;
mod line_parser;
mod partial_result;
mod processor;

use engine::Engine;

fn main() {
    let engine = Engine::from_args();
    let result = engine.run();
    println!("{:?}", result);
}