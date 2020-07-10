use std::error::Error;
use std::io::{self, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.lock().read_to_string(&mut input)?;
    let output = ttdl_lunar_calendar::run(&input)?;
    println!("{}", output);
    Ok(())
}
