use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
pub enum StrSimError {
    DifferentLengthArgs
}

pub type HammingResult = Result<usize, StrSimError>;

pub fn hamming(a: &str, b: &str) -> HammingResult {
    let (mut ita, mut itb, mut count) = (a.chars(), b.chars(), 0);
    loop {
        match (ita.next(), itb.next()){
            (Some(x), Some(y)) => if x != y { count += 1 },
            (None, None) => return Ok(count),
            _ => return Err(StrSimError::DifferentLengthArgs),
        }
    }
}

fn main()-> std::io::Result<()> {
    let mut file = File::open("../../day-2-input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let lines = contents.lines().collect::<Vec<&str>>();
    let lines2 = lines.clone();


    'outer: for &a_string in lines.iter() {
        for &b_string in lines2.iter() {
            if 1 == hamming(a_string, b_string).unwrap() {
                println!("{}", a_string);
                println!("{}", b_string);
                break 'outer;
            }
        }
    }

    Ok(())
}
