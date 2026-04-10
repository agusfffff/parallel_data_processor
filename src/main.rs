mod chunk;
mod engine;
mod errors;
mod partial_result;
mod processor;
mod line_parser; 
mod aggregator;
mod accumulator;
use crate::engine::Engine;
use crate::errors::Error;

fn main() -> Result<(), Error> {
    let engine = Engine::from_env()?;
    let result = engine.run()?;
    println!("{:?}", result);
    Ok(())
}
