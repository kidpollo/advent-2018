use std::fs::File;
use std::io::prelude::*;

fn main()-> std::io::Result<()> {
    let mut file = File::open("../../day-2-input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let lines_iterator = contents.lines();

    for line in lines_iterator {
        println!("Got: {}", line);
    }

    Ok(())
}
