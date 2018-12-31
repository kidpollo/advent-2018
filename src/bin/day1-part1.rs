use std::fs::File;
use std::io::prelude::*;

fn main()-> std::io::Result<()> {
    let mut file = File::open("../../day-1-input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let result = contents.lines().fold(0, |acc, line| acc + line.parse::<i32>().unwrap());
    println!("Result: {}", result);
    Ok(())
}
