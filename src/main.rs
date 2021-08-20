use std::fs;
use std::error::Error;
use rand::{self, seq::SliceRandom};

const PATH: &'static str = "quotes.txt";

fn main() -> Result<(), Box<dyn Error>> {
    let contents: String = fs::read_to_string(PATH)?;
    let mut quotes: Vec<&str> = contents.lines().collect();

    let mut rng = rand::thread_rng();
    quotes.shuffle(&mut rng);
    let quote = quotes[0];

    println!("{}", quote);

    Ok(())
}